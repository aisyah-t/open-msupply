use async_graphql::*;

use graphql_core::{
    simple_generic_errors::{OtherPartyNotACustomer, OtherPartyNotVisible},
    standard_graphql_error::{validate_auth, StandardGraphqlError},
    ContextExt,
};
use graphql_types::types::InvoiceNode;
use service::invoice::inbound_return::update::{
    UpdateInboundReturn as ServiceInput, UpdateInboundReturnError as ServiceError,
};
use service::{
    auth::{Resource, ResourceAccessRequest},
    invoice::inbound_return::update::UpdateInboundReturnStatus,
};

#[derive(InputObject)]
#[graphql(name = "UpdateInboundReturnInput")]
pub struct UpdateInput {
    pub id: String,
    other_party_id: Option<String>,
    status: Option<UpdateInboundReturnStatusInput>,
    on_hold: Option<bool>,
    comment: Option<String>,
    colour: Option<String>,
    their_reference: Option<String>,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum UpdateInboundReturnStatusInput {
    Delivered,
    Verified,
}

#[derive(SimpleObject)]
#[graphql(name = "UpdateInboundReturnError")]
pub struct UpdateError {
    pub error: UpdateErrorInterface,
}

#[derive(Interface)]
#[graphql(name = "UpdateInboundReturnErrorInterface")]
#[graphql(field(name = "description", ty = "&str"))]
pub enum UpdateErrorInterface {
    OtherPartyNotACustomer(OtherPartyNotACustomer),
    OtherPartyNotVisible(OtherPartyNotVisible),
}

#[derive(Union)]
#[graphql(name = "UpdateInboundReturnResponse")]
pub enum UpdateResponse {
    Response(InvoiceNode),
    Error(UpdateError),
}

pub fn update(ctx: &Context<'_>, store_id: &str, input: UpdateInput) -> Result<UpdateResponse> {
    let user = validate_auth(
        ctx,
        &ResourceAccessRequest {
            resource: Resource::MutateInboundReturn,
            store_id: Some(store_id.to_string()),
        },
    )?;

    let service_provider = ctx.service_provider();
    let service_context = service_provider.context(store_id.to_string(), user.user_id)?;

    let result = service_provider
        .invoice_service
        .update_inbound_return(&service_context, input.to_domain());

    let result = match result {
        Ok(inbound_return) => UpdateResponse::Response(InvoiceNode::from_domain(inbound_return)),
        Err(err) => UpdateResponse::Error(UpdateError {
            error: map_error(err)?,
        }),
    };

    Ok(result)
}

fn map_error(error: ServiceError) -> Result<UpdateErrorInterface> {
    use StandardGraphqlError::*;
    let formatted_error = format!("{:#?}", error);

    let graphql_error = match error {
        ServiceError::OtherPartyNotACustomer => {
            return Ok(UpdateErrorInterface::OtherPartyNotACustomer(
                OtherPartyNotACustomer,
            ))
        }
        ServiceError::OtherPartyNotVisible => {
            return Ok(UpdateErrorInterface::OtherPartyNotVisible(
                OtherPartyNotVisible,
            ))
        }
        // Standard Graphql Errors
        ServiceError::InvoiceDoesNotExist
        | ServiceError::NotAnInboundReturn
        | ServiceError::NotThisStoreInvoice
        | ServiceError::CannotReverseInvoiceStatus
        | ServiceError::ReturnIsNotEditable
        | ServiceError::CannotChangeStatusOfInvoiceOnHold
        | ServiceError::OtherPartyDoesNotExist => BadUserInput(formatted_error),

        ServiceError::UpdatedInvoiceDoesNotExist | ServiceError::DatabaseError(_) => {
            InternalError(formatted_error)
        }
    };

    Err(graphql_error.extend())
}

impl UpdateInput {
    pub fn to_domain(self) -> ServiceInput {
        let UpdateInput {
            id,
            comment,
            status,
            on_hold,
            colour,
            their_reference,
            other_party_id,
        }: UpdateInput = self;

        ServiceInput {
            id,
            status: status.map(|status| status.to_domain()),
            comment,
            on_hold,
            colour,
            their_reference,
            other_party_id,
        }
    }
}

impl UpdateInboundReturnStatusInput {
    pub fn to_domain(&self) -> UpdateInboundReturnStatus {
        use UpdateInboundReturnStatus::*;
        match self {
            UpdateInboundReturnStatusInput::Delivered => Delivered,
            UpdateInboundReturnStatusInput::Verified => Verified,
        }
    }
}
