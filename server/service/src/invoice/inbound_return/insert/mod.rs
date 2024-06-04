use repository::{
    ActivityLogType, Invoice, InvoiceRowRepository, RepositoryError, TransactionError,
};

use crate::{
    activity_log::activity_log_entry,
    invoice::get_invoice,
    invoice_line::{
        stock_in_line::insert::{insert_stock_in_line, InsertStockInLineError},
        update_return_reason_id::{update_return_reason_id, UpdateLineReturnReasonError},
    },
    service_provider::ServiceContext,
};
pub mod generate;
pub mod validate;
use generate::generate;
use validate::validate;

use super::InboundReturnLineInput;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InsertInboundReturn {
    pub id: String,
    pub other_party_id: String,
    pub outbound_shipment_id: Option<String>,
    pub inbound_return_lines: Vec<InboundReturnLineInput>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InsertInboundReturnError {
    InvoiceAlreadyExists,
    // Original invoice/shipment validation
    OutboundShipmentDoesNotExist,
    OutboundShipmentDoesNotBelongToCurrentStore,
    OriginalInvoiceNotAnOutboundShipment,
    CannotReturnOutboundShipment,
    // Name validation
    OtherPartyNotACustomer,
    OtherPartyNotVisible,
    OtherPartyDoesNotExist,
    // Internal
    NewlyCreatedInvoiceDoesNotExist,
    DatabaseError(RepositoryError),
    // Line Errors
    LineInsertError {
        line_id: String,
        error: InsertStockInLineError,
    },
    LineReturnReasonUpdateError {
        line_id: String,
        error: UpdateLineReturnReasonError,
    },
}

type OutError = InsertInboundReturnError;

pub fn insert_inbound_return(
    ctx: &ServiceContext,
    input: InsertInboundReturn,
) -> Result<Invoice, OutError> {
    let inbound_return: Invoice = ctx
        .connection
        .transaction_sync(|connection| {
            let other_party = validate(connection, &ctx.store_id, &input)?;
            let (inbound_return, insert_stock_in_lines, update_line_return_reasons) = generate(
                connection,
                &ctx.store_id,
                &ctx.user_id,
                input.clone(),
                other_party,
            )?;

            InvoiceRowRepository::new(&connection).upsert_one(&inbound_return)?;

            for line in insert_stock_in_lines {
                insert_stock_in_line(ctx, line.clone()).map_err(|error| {
                    OutError::LineInsertError {
                        line_id: line.id,
                        error,
                    }
                })?;
            }

            for line in update_line_return_reasons {
                update_return_reason_id(ctx, line.clone()).map_err(|error| {
                    OutError::LineReturnReasonUpdateError {
                        line_id: line.line_id,
                        error,
                    }
                })?;
            }

            activity_log_entry(
                &ctx,
                ActivityLogType::InvoiceCreated,
                Some(inbound_return.id.to_owned()),
                None,
                None,
            )?;

            get_invoice(ctx, None, &inbound_return.id)
                .map_err(|error| OutError::DatabaseError(error))?
                .ok_or(OutError::NewlyCreatedInvoiceDoesNotExist)
        })
        .map_err(|error| error.to_inner_error())?;

    Ok(inbound_return)
}

impl From<RepositoryError> for OutError {
    fn from(error: RepositoryError) -> Self {
        OutError::DatabaseError(error)
    }
}

impl From<TransactionError<OutError>> for OutError {
    fn from(error: TransactionError<OutError>) -> Self {
        match error {
            TransactionError::Transaction { msg, level } => {
                OutError::DatabaseError(RepositoryError::TransactionError { msg, level })
            }
            TransactionError::Inner(e) => e,
        }
    }
}

#[cfg(test)]
mod test {
    use repository::{
        mock::{
            currency_a, mock_inbound_return_a, mock_inbound_return_a_invoice_line_a, mock_item_a,
            mock_item_b, mock_name_customer_a, mock_outbound_return_a, mock_outbound_shipment_a,
            mock_outbound_shipment_e, mock_store_a, mock_user_account_a, MockData, MockDataInserts,
        },
        test_db::setup_all_with_data,
        InvoiceLineRowRepository, InvoiceRow, InvoiceRowRepository, InvoiceStatus, InvoiceType,
        NameRow, NameStoreJoinRow, ReturnReasonRow,
    };
    use util::{inline_edit, inline_init};

    use crate::{
        invoice::inbound_return::insert::{
            InsertInboundReturn, InsertInboundReturnError as ServiceError,
        },
        invoice_line::{
            stock_in_line::InsertStockInLineError,
            update_return_reason_id::UpdateLineReturnReasonError,
        },
        service_provider::ServiceProvider,
    };

    use super::InboundReturnLineInput;

    #[actix_rt::test]
    async fn test_insert_inbound_return_errors() {
        fn not_visible() -> NameRow {
            NameRow {
                id: "not_visible".to_string(),
                ..Default::default()
            }
        }

        fn not_a_customer() -> NameRow {
            NameRow {
                id: "not_a_customer".to_string(),
                ..Default::default()
            }
        }

        fn not_a_customer_join() -> NameStoreJoinRow {
            NameStoreJoinRow {
                id: "not_a_customer_join".to_string(),
                name_link_id: not_a_customer().id,
                store_id: mock_store_a().id,
                name_is_customer: false,
                ..Default::default()
            }
        }

        let (_, _, connection_manager, _) = setup_all_with_data(
            "test_insert_inbound_return_errors",
            MockDataInserts::all(),
            MockData {
                names: vec![not_visible(), not_a_customer()],
                name_store_joins: vec![not_a_customer_join()],
                ..Default::default()
            },
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider
            .context(mock_store_a().id, mock_user_account_a().id)
            .unwrap();

        // InvoiceAlreadyExists
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = mock_inbound_return_a().id;
                })
            ),
            Err(ServiceError::InvoiceAlreadyExists)
        );

        // OutboundShipmentDoesNotExist
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_id".to_string();
                    r.outbound_shipment_id = Some("does_not_exist".to_string());
                })
            ),
            Err(ServiceError::OutboundShipmentDoesNotExist)
        );

        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_id".to_string();
                    r.outbound_shipment_id = Some(mock_outbound_shipment_a().id);
                })
            ),
            Err(ServiceError::OutboundShipmentDoesNotBelongToCurrentStore)
        );

        // OriginalInvoiceNotAnOutboundShipment
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_id".to_string();
                    r.outbound_shipment_id = Some(mock_outbound_return_a().id);
                })
            ),
            Err(ServiceError::OriginalInvoiceNotAnOutboundShipment)
        );

        // CannotReturnOutboundShipment
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_id".to_string();
                    // in NEW status
                    r.outbound_shipment_id = Some(mock_outbound_shipment_e().id);
                })
            ),
            Err(ServiceError::CannotReturnOutboundShipment)
        );

        // OtherPartyDoesNotExist
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_id".to_string();
                    r.other_party_id = "does_not_exist".to_string();
                })
            ),
            Err(ServiceError::OtherPartyDoesNotExist)
        );

        // OtherPartyNotVisible
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_id".to_string();
                    r.other_party_id = not_visible().id.clone();
                })
            ),
            Err(ServiceError::OtherPartyNotVisible)
        );

        // OtherPartyNotACustomer
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_id".to_string();
                    r.other_party_id = not_a_customer().id.clone();
                })
            ),
            Err(ServiceError::OtherPartyNotACustomer)
        );

        // LineInsertError
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                InsertInboundReturn {
                    id: "new_id".to_string(),
                    other_party_id: mock_name_customer_a().id,
                    inbound_return_lines: vec![InboundReturnLineInput {
                        id: mock_inbound_return_a_invoice_line_a().id,
                        number_of_packs: 1.0,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ),
            Err(ServiceError::LineInsertError {
                line_id: mock_inbound_return_a_invoice_line_a().id,
                error: InsertStockInLineError::LineAlreadyExists,
            }),
        );

        // LineReturnReasonUpdateError
        assert_eq!(
            service_provider.invoice_service.insert_inbound_return(
                &context,
                InsertInboundReturn {
                    id: "some_new_id".to_string(),
                    other_party_id: mock_name_customer_a().id,
                    inbound_return_lines: vec![InboundReturnLineInput {
                        id: "new_line_id".to_string(),
                        item_id: mock_item_a().id,
                        number_of_packs: 1.0,
                        pack_size: 1,
                        reason_id: Some("does_not_exist".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ),
            Err(ServiceError::LineReturnReasonUpdateError {
                line_id: "new_line_id".to_string(),
                error: UpdateLineReturnReasonError::ReasonDoesNotExist,
            }),
        );
    }

    #[actix_rt::test]
    async fn test_insert_inbound_return_success() {
        fn return_reason() -> ReturnReasonRow {
            inline_init(|r: &mut ReturnReasonRow| {
                r.id = "return_reason".to_string();
                r.is_active = true;
            })
        }

        fn returnable_outbound_shipment() -> InvoiceRow {
            inline_init(|r: &mut InvoiceRow| {
                r.id = "returnable_outbound_shipment".to_string();
                r.currency_id = Some(currency_a().id);
                r.r#type = InvoiceType::OutboundShipment;
                r.status = InvoiceStatus::Verified;
                r.store_id = mock_store_a().id;
                r.name_link_id = mock_name_customer_a().id;
            })
        }

        let (_, connection, connection_manager, _) = setup_all_with_data(
            "test_insert_inbound_return_success",
            MockDataInserts::all(),
            inline_init(|r: &mut MockData| {
                r.invoices = vec![returnable_outbound_shipment()];
                r.return_reasons = vec![return_reason()];
            }),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider
            .context(mock_store_a().id, mock_user_account_a().id)
            .unwrap();

        service_provider
            .invoice_service
            .insert_inbound_return(
                &context,
                inline_init(|r: &mut InsertInboundReturn| {
                    r.id = "new_inbound_return_id".to_string();
                    r.other_party_id = mock_name_customer_a().id;
                    r.outbound_shipment_id = Some(returnable_outbound_shipment().id);
                    r.inbound_return_lines = vec![
                        InboundReturnLineInput {
                            id: "new_inbound_return_line_id".to_string(),
                            reason_id: Some(return_reason().id),
                            number_of_packs: 1.0,
                            item_id: mock_item_a().id,
                            pack_size: 1,
                            ..Default::default()
                        },
                        InboundReturnLineInput {
                            id: "new_inbound_return_line_id_2".to_string(),
                            reason_id: Some(return_reason().id),
                            number_of_packs: 0.0,
                            item_id: mock_item_b().id,
                            pack_size: 1,
                            ..Default::default()
                        },
                    ];
                }),
            )
            .unwrap();

        let invoice = InvoiceRowRepository::new(&connection)
            .find_one_by_id("new_inbound_return_id")
            .unwrap()
            .unwrap();

        assert_eq!(invoice.id, "new_inbound_return_id");
        assert_eq!(
            invoice,
            inline_edit(&invoice, |mut u| {
                u.name_link_id = mock_name_customer_a().id;
                u.user_id = Some(mock_user_account_a().id);
                u.original_shipment_id = Some(returnable_outbound_shipment().id);
                u
            })
        );

        let lines = InvoiceLineRowRepository::new(&connection)
            .find_many_by_invoice_id("new_inbound_return_id")
            .unwrap();

        // line with number_of_packs == 0.0 should not be inserted
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            inline_edit(&lines[0], |mut u| {
                u.invoice_id = "new_inbound_return_id".to_string();
                u.id = "new_inbound_return_line_id".to_string();
                u.item_link_id = mock_item_a().id;
                u.number_of_packs = 1.0;
                u
            })
        );
    }
}
