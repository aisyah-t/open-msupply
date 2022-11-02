use async_graphql::*;
use graphql_core::{
    pagination::PaginationInput,
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use repository::{EncounterFilter, PaginationOption, Permission};
use service::{
    auth::{context_permissions, Resource, ResourceAccessRequest},
    programs::encounter::encounter_fields::{EncounterFields, EncounterFieldsResult},
};

use crate::types::encounter::EncounterNode;

use super::{EncounterFilterInput, EncounterSortInput};

#[derive(InputObject, Clone)]
pub struct EncounterFieldsInput {
    pub fields: Vec<String>,
}

pub struct EncounterFieldsNode {
    pub store_id: String,
    pub encounter_fields_result: EncounterFieldsResult,

    allowed_docs: Vec<String>,
}

#[derive(SimpleObject)]
pub struct EncounterFieldsConnector {
    pub total_count: u32,
    pub nodes: Vec<EncounterFieldsNode>,
}

#[derive(Union)]
pub enum EncounterFieldsResponse {
    Response(EncounterFieldsConnector),
}

#[Object]
impl EncounterFieldsNode {
    pub async fn encounter(&self) -> EncounterNode {
        EncounterNode {
            store_id: self.store_id.clone(),
            encounter_row: self.encounter_fields_result.row.clone(),
            allowed_docs: self.allowed_docs.clone(),
        }
    }

    pub async fn fields(&self) -> &Vec<serde_json::Value> {
        &self.encounter_fields_result.fields
    }
}

pub fn encounter_fields(
    ctx: &Context<'_>,
    store_id: String,
    input: EncounterFieldsInput,
    page: Option<PaginationInput>,
    filter: Option<EncounterFilterInput>,
    sort: Option<EncounterSortInput>,
) -> Result<EncounterFieldsResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::QueryEncounter,
            store_id: Some(store_id.clone()),
        },
    )?;
    let allowed_docs = context_permissions(Permission::ProgramQuery, &user.permissions);

    let mut filter = filter
        .map(|f| f.to_domain_filter())
        .unwrap_or(EncounterFilter::new());
    // restrict query results to allowed entries
    filter.r#type = Some(
        filter
            .r#type
            .unwrap_or_default()
            .restrict_results(&allowed_docs),
    );

    let service_provider = ctx.service_provider();
    let context = service_provider.basic_context()?;

    let result = service_provider
        .encounter_service
        .encounters_fields(
            &context,
            EncounterFields {
                fields: input.fields,
            },
            page.map(PaginationOption::from),
            Some(filter),
            sort.map(EncounterSortInput::to_domain),
        )
        .map_err(StandardGraphqlError::from_list_error)?;

    let nodes = result
        .rows
        .into_iter()
        .map(|encounter_fields| EncounterFieldsNode {
            store_id: store_id.clone(),
            encounter_fields_result: encounter_fields,
            allowed_docs: allowed_docs.clone(),
        })
        .collect();

    Ok(EncounterFieldsResponse::Response(
        EncounterFieldsConnector {
            total_count: result.count,
            nodes,
        },
    ))
}
