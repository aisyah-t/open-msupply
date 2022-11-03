use async_graphql::*;
use graphql_core::{
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use repository::{EncounterFilter, EqualFilter, Permission};
use service::{
    auth::{context_permissions, Resource, ResourceAccessRequest},
    programs::encounter::{InsertEncounter, InsertEncounterError},
};

use crate::types::encounter::EncounterNode;

#[derive(InputObject)]
pub struct InsertEncounterInput {
    /// The encounter type
    pub r#type: String,
    pub patient_id: String,
    /// Encounter document data
    pub data: serde_json::Value,
    /// The schema id used for the encounter data
    pub schema_id: String,
    /// The program type
    pub program_type: String,
}

#[derive(Union)]
pub enum InsertEncounterResponse {
    Response(EncounterNode),
}

pub fn insert_encounter(
    ctx: &Context<'_>,
    store_id: String,
    input: InsertEncounterInput,
) -> Result<InsertEncounterResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::MutateEncounter,
            store_id: Some(store_id.clone()),
        },
    )?;
    let allowed_docs = context_permissions(Permission::DocumentMutate, &user.permissions);

    let service_provider = ctx.service_provider();
    let service_context = service_provider.basic_context()?;

    let document = match service_provider.encounter_service.insert_encounter(
        &service_context,
        service_provider,
        &user.user_id,
        InsertEncounter {
            data: input.data,
            schema_id: input.schema_id,
            patient_id: input.patient_id,
            program: input.program_type,
            r#type: input.r#type,
        },
        allowed_docs.clone(),
    ) {
        Ok(document) => document,
        Err(error) => {
            let formatted_error = format!("{:#?}", error);
            let std_err = match error {
                InsertEncounterError::NotAllowedToMutDocument => {
                    StandardGraphqlError::Forbidden(formatted_error)
                }
                InsertEncounterError::InvalidPatientOrProgram => {
                    StandardGraphqlError::BadUserInput(formatted_error)
                }
                InsertEncounterError::InvalidDataSchema(_) => {
                    StandardGraphqlError::BadUserInput(formatted_error)
                }
                InsertEncounterError::DataSchemaDoesNotExist => {
                    StandardGraphqlError::BadUserInput(formatted_error)
                }
                InsertEncounterError::InternalError(_) => {
                    StandardGraphqlError::InternalError(formatted_error)
                }
                InsertEncounterError::DatabaseError(_) => {
                    StandardGraphqlError::InternalError(formatted_error)
                }
            };
            return Err(std_err.extend());
        }
    };

    let encounter_row = service_provider
        .encounter_service
        .encounter(
            &service_context,
            EncounterFilter::new().name(EqualFilter::equal_to(&document.name)),
            allowed_docs.clone(),
        )?
        .ok_or(
            StandardGraphqlError::InternalError("Encounter went missing".to_string()).extend(),
        )?;

    Ok(InsertEncounterResponse::Response(EncounterNode {
        store_id,
        encounter_row,
        allowed_docs,
    }))
}
