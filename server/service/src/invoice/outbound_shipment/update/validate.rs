use crate::invoice::{
    check_invoice_is_editable, check_invoice_status, check_status_change, check_store,
    InvoiceIsNotEditable, InvoiceRowStatusError, NotThisStoreInvoice,
};
use repository::EqualFilter;
use repository::{
    InvoiceLineFilter, InvoiceLineRepository, InvoiceLineRowType, InvoiceRow, InvoiceRowRepository,
    InvoiceRowStatus, InvoiceRowType, RepositoryError, StorageConnection,
};

use super::{UpdateOutboundShipment, UpdateOutboundShipmentError};

pub fn validate(
    connection: &StorageConnection,
    store_id: &str,
    patch: &UpdateOutboundShipment,
) -> Result<(InvoiceRow, bool), UpdateOutboundShipmentError> {
    let invoice = check_invoice_exists(&patch.id, connection)?;
    check_store(&invoice, store_id)?;
    check_invoice_type(&invoice)?;
    check_invoice_is_editable(&invoice)?;

    // Status check
    let status_changed = check_status_change(&invoice, patch.full_status());
    if status_changed {
        check_invoice_status(&invoice, patch.full_status(), &patch.on_hold)?;
        check_can_change_status_to_allocated(connection, &invoice, patch.full_status())?;
    }
    Ok((invoice, status_changed))
}

fn check_invoice_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<InvoiceRow, UpdateOutboundShipmentError> {
    let result = InvoiceRowRepository::new(connection).find_one_by_id(id);

    if let Err(RepositoryError::NotFound) = &result {
        return Err(UpdateOutboundShipmentError::InvoiceDoesNotExist);
    }
    Ok(result?)
}

// If status is changed to allocated and above, return error if there are
// unallocated lines with quantity above 0, zero quantity unallocated lines will be deleted
fn check_can_change_status_to_allocated(
    connection: &StorageConnection,
    invoice_row: &InvoiceRow,
    status_option: Option<InvoiceRowStatus>,
) -> Result<(), UpdateOutboundShipmentError> {
    if invoice_row.status != InvoiceRowStatus::New {
        return Ok(());
    };

    // Status sequence for outbound shipment: New, Allocated, Picked, Shipped
    if let Some(new_status) = status_option {
        if new_status == InvoiceRowStatus::New {
            return Ok(());
        }

        let repository = InvoiceLineRepository::new(connection);
        let unallocated_lines = repository.query_by_filter(
            InvoiceLineFilter::new()
                .invoice_id(EqualFilter::equal_to(&invoice_row.id))
                .r#type(InvoiceLineRowType::UnallocatedStock.equal_to())
                .number_of_packs(EqualFilter::not_equal_to_f64(0.0)),
        )?;

        if unallocated_lines.len() > 0 {
            return Err(
                UpdateOutboundShipmentError::CanOnlyChangeToAllocatedWhenNoUnallocatedLines(
                    unallocated_lines,
                ),
            );
        }
    }

    Ok(())
}

fn check_invoice_type(invoice: &InvoiceRow) -> Result<(), UpdateOutboundShipmentError> {
    if invoice.r#type != InvoiceRowType::OutboundShipment {
        Err(UpdateOutboundShipmentError::NotAnOutboundShipment)
    } else {
        Ok(())
    }
}

impl From<InvoiceIsNotEditable> for UpdateOutboundShipmentError {
    fn from(_: InvoiceIsNotEditable) -> Self {
        UpdateOutboundShipmentError::InvoiceIsNotEditable
    }
}

impl From<InvoiceRowStatusError> for UpdateOutboundShipmentError {
    fn from(error: InvoiceRowStatusError) -> Self {
        use UpdateOutboundShipmentError::*;
        match error {
            InvoiceRowStatusError::CannotChangeStatusOfInvoiceOnHold => {
                CannotChangeStatusOfInvoiceOnHold
            }
            InvoiceRowStatusError::CannotReverseInvoiceStatus => CannotReverseInvoiceStatus,
        }
    }
}

impl From<NotThisStoreInvoice> for UpdateOutboundShipmentError {
    fn from(_: NotThisStoreInvoice) -> Self {
        UpdateOutboundShipmentError::NotThisStoreInvoice
    }
}
