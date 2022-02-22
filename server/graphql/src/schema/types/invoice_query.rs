use crate::{
    loader::{
        InvoiceLineQueryLoader, InvoiceQueryLoader, InvoiceStatsLoader, NameByIdLoader,
        RequisitionsByIdLoader, StoreLoader,
    },
    ContextExt,
};
use async_graphql::*;
use chrono::{DateTime, Utc};
use dataloader::DataLoader;
use domain::{
    invoice::{Invoice, InvoiceFilter},
    DatetimeFilter, EqualFilter, SimpleStringFilter,
};
use repository::schema::InvoiceStatsRow;
use serde::Serialize;
use service::{usize_to_u32, ListResult};

use super::{
    DatetimeFilterInput, EqualFilterBigNumberInput, EqualFilterInput, EqualFilterStringInput,
    ErrorWrapper, InvoiceLineConnector, NameNode, NameResponse, NodeError, NodeErrorInterface,
    RequisitionNode, SimpleStringFilterInput, SortInput, StoreNode,
};

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(remote = "domain::invoice::InvoiceSortField")]
#[graphql(rename_items = "camelCase")]
pub enum InvoiceSortFieldInput {
    Type,
    OtherPartyName,
    InvoiceNumber,
    Comment,
    Status,
    CreatedDatetime,
    AllocatedDatetime,
    PickedDatetime,
    ShippedDatetime,
    DeliveredDatetime,
    VerifiedDatetime,
}

pub type InvoiceSortInput = SortInput<InvoiceSortFieldInput>;

#[derive(InputObject, Clone)]
pub struct InvoiceFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub invoice_number: Option<EqualFilterBigNumberInput>,
    pub name_id: Option<EqualFilterStringInput>,
    pub store_id: Option<EqualFilterStringInput>,
    pub r#type: Option<EqualFilterInput<InvoiceNodeType>>,
    pub status: Option<EqualFilterInput<InvoiceNodeStatus>>,
    pub comment: Option<SimpleStringFilterInput>,
    pub their_reference: Option<EqualFilterStringInput>,
    pub created_datetime: Option<DatetimeFilterInput>,
    pub allocated_datetime: Option<DatetimeFilterInput>,
    pub picked_datetime: Option<DatetimeFilterInput>,
    pub shipped_datetime: Option<DatetimeFilterInput>,
    pub delivered_datetime: Option<DatetimeFilterInput>,
    pub verified_datetime: Option<DatetimeFilterInput>,
    pub requisition_id: Option<EqualFilterStringInput>,
    pub linked_invoice_id: Option<EqualFilterStringInput>,
}

impl From<InvoiceFilterInput> for InvoiceFilter {
    fn from(f: InvoiceFilterInput) -> Self {
        InvoiceFilter {
            id: f.id.map(EqualFilter::from),
            invoice_number: f.invoice_number.map(EqualFilter::from),
            name_id: f.name_id.map(EqualFilter::from),
            store_id: f.store_id.map(EqualFilter::from),
            r#type: f.r#type.map(EqualFilter::from),
            status: f.status.map(EqualFilter::from),
            comment: f.comment.map(SimpleStringFilter::from),
            their_reference: f.their_reference.map(EqualFilter::from),
            created_datetime: f.created_datetime.map(DatetimeFilter::from),
            allocated_datetime: f.allocated_datetime.map(DatetimeFilter::from),
            picked_datetime: f.picked_datetime.map(DatetimeFilter::from),
            shipped_datetime: f.shipped_datetime.map(DatetimeFilter::from),
            delivered_datetime: f.delivered_datetime.map(DatetimeFilter::from),
            verified_datetime: f.verified_datetime.map(DatetimeFilter::from),
            requisition_id: f.requisition_id.map(EqualFilter::from),
            linked_invoice_id: f.linked_invoice_id.map(EqualFilter::from),
        }
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[graphql(remote = "domain::invoice::InvoiceType")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceNodeType {
    OutboundShipment,
    InboundShipment,
    InventoryAdjustment,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[graphql(remote = "domain::invoice::InvoiceStatus")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // only needed to be comparable in tests
pub enum InvoiceNodeStatus {
    /// Outbound Shipment: available_number_of_packs in a stock line gets
    /// updated when items are added to the invoice.
    /// Inbound Shipment: No stock changes in this status, only manually entered
    /// inbound Shipments have new status
    New,
    /// General description: Outbound Shipment is ready for picking (all unallocated lines need to be fullfilled)
    /// Outbound Shipment: Invoice can only be turned to allocated status when
    /// all unallocated lines are fullfilled
    /// Inbound Shipment: not applicable
    Allocated,
    /// General description: Outbound Shipment was picked from shelf and ready for Shipment
    /// Outbound Shipment: available_number_of_packs and
    /// total_number_of_packs get updated when items are added to the invoice
    /// Inbound Shipment: For inter store stock transfers an inbound Shipment
    /// is created when corresponding outbound Shipment is picked and ready for
    /// Shipment, inbound Shipment is not editable in this status
    Picked,
    /// General description: Outbound Shipment is sent out for delivery
    /// Outbound Shipment: Becomes not editable
    /// Inbound Shipment: For inter store stock transfers an inbound Shipment
    /// becomes editable when this status is set as a result of corresponding
    /// outbound Shipment being chagned to shipped (this is similar to New status)
    Shipped,
    /// General description: Inbound Shipment was received
    /// Outbound Shipment: Status is updated based on corresponding inbound Shipment
    /// Inbound Shipment: Stock is introduced and can be issued
    Delivered,
    /// General description: Received inbound Shipment was counted and verified
    /// Outbound Shipment: Status is updated based on corresponding inbound Shipment
    /// Inbound Shipment: Becomes not editable
    Verified,
}

pub struct InvoiceNode {
    pub invoice: Invoice,
}

#[derive(SimpleObject)]
pub struct InvoiceConnector {
    total_count: u32,
    nodes: Vec<InvoiceNode>,
}

#[Object]
impl InvoiceNode {
    pub async fn id(&self) -> &str {
        &self.invoice.id
    }

    pub async fn other_party_name(&self) -> &str {
        &self.invoice.other_party_name
    }

    pub async fn other_party_id(&self) -> &str {
        &self.invoice.other_party_id
    }

    pub async fn other_party_store(&self, ctx: &Context<'_>) -> Result<Option<StoreNode>> {
        let other_party_store_id = match &self.invoice.other_party_store_id {
            Some(other_party_store_id) => other_party_store_id,
            None => return Ok(None),
        };

        let loader = ctx.get_loader::<DataLoader<StoreLoader>>();
        Ok(loader
            .load_one(other_party_store_id.clone())
            .await?
            .map(StoreNode::from))
    }

    pub async fn r#type(&self) -> InvoiceNodeType {
        self.invoice.r#type.clone().into()
    }

    pub async fn status(&self) -> InvoiceNodeStatus {
        self.invoice.status.clone().into()
    }

    pub async fn invoice_number(&self) -> i64 {
        self.invoice.invoice_number
    }

    pub async fn their_reference(&self) -> &Option<String> {
        &self.invoice.their_reference
    }

    pub async fn comment(&self) -> &Option<String> {
        &self.invoice.comment
    }

    pub async fn on_hold(&self) -> bool {
        self.invoice.on_hold
    }

    pub async fn created_datetime(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.invoice.created_datetime, Utc)
    }

    pub async fn allocated_datetime(&self) -> Option<DateTime<Utc>> {
        self.invoice
            .allocated_datetime
            .map(|v| DateTime::<Utc>::from_utc(v, Utc))
    }

    pub async fn picked_datetime(&self) -> Option<DateTime<Utc>> {
        self.invoice
            .picked_datetime
            .map(|v| DateTime::<Utc>::from_utc(v, Utc))
    }

    pub async fn shipped_datetime(&self) -> Option<DateTime<Utc>> {
        self.invoice
            .shipped_datetime
            .map(|v| DateTime::<Utc>::from_utc(v, Utc))
    }

    pub async fn delivered_datetime(&self) -> Option<DateTime<Utc>> {
        self.invoice
            .delivered_datetime
            .map(|v| DateTime::<Utc>::from_utc(v, Utc))
    }

    pub async fn verified_datetime(&self) -> Option<DateTime<Utc>> {
        self.invoice
            .verified_datetime
            .map(|v| DateTime::<Utc>::from_utc(v, Utc))
    }

    pub async fn colour(&self) -> &Option<String> {
        &self.invoice.colour
    }

    /// Response Requisition that is the origin of this Outbound Shipment
    /// Or Request Requisition for Inbound Shipment that Originated from Outbound Shipment (linked through Response Requisition)
    pub async fn requisition(&self, ctx: &Context<'_>) -> Result<Option<RequisitionNode>> {
        let requisition_id = if let Some(id) = &self.invoice.requisition_id {
            id
        } else {
            return Ok(None);
        };

        let loader = ctx.get_loader::<DataLoader<RequisitionsByIdLoader>>();

        Ok(loader
            .load_one(requisition_id.clone())
            .await?
            .map(RequisitionNode::from_domain))
    }

    /// Inbound Shipment <-> Outbound Shipment, where Inbound Shipment originated from Outbound Shipment
    pub async fn linked_shipment(&self, ctx: &Context<'_>) -> Result<Option<InvoiceNode>> {
        let linked_invoice_id = if let Some(id) = &self.invoice.linked_invoice_id {
            id
        } else {
            return Ok(None);
        };

        let loader = ctx.get_loader::<DataLoader<InvoiceQueryLoader>>();
        Ok(loader
            .load_one(linked_invoice_id.to_string())
            .await?
            .map(InvoiceNode::from_domain))
    }

    pub async fn lines(&self, ctx: &Context<'_>) -> Result<InvoiceLineConnector> {
        let loader = ctx.get_loader::<DataLoader<InvoiceLineQueryLoader>>();
        let result_option = loader.load_one(self.invoice.id.to_string()).await?;

        Ok(InvoiceLineConnector::from_vec(
            result_option.unwrap_or(vec![]),
        ))
    }

    async fn pricing(&self, ctx: &Context<'_>) -> InvoicePriceResponse {
        let loader = ctx.get_loader::<DataLoader<InvoiceStatsLoader>>();
        let default = InvoiceStatsRow {
            invoice_id: self.invoice.id.clone(),
            total_before_tax: 0.0,
            total_after_tax: 0.0,
            stock_total_before_tax: 0.0,
            stock_total_after_tax: 0.0,
            service_total_before_tax: 0.0,
            service_total_after_tax: 0.0,
        };

        match loader.load_one(self.invoice.id.to_string()).await {
            Ok(result_option) => InvoicePriceResponse::Response(InvoicePricingNode {
                invoice_pricing: result_option.unwrap_or(default),
            }),
            // TODO report error
            Err(error) => InvoicePriceResponse::Error(error.into()),
        }
    }

    async fn other_party(&self, ctx: &Context<'_>) -> NameResponse {
        let loader = ctx.get_loader::<DataLoader<NameByIdLoader>>();

        match loader.load_one(self.invoice.other_party_id.clone()).await {
            Ok(response_option) => match response_option {
                Some(name) => NameResponse::Response(NameNode::from_domain(name)),
                None => NameResponse::Error(ErrorWrapper {
                    error: NodeErrorInterface::record_not_found(),
                }),
            },
            Err(error) => NameResponse::Error(error.into()),
        }
    }
}

#[derive(Union)]
pub enum InvoicesResponse {
    Response(InvoiceConnector),
}

impl InvoiceNode {
    pub fn from_domain(invoice: Invoice) -> InvoiceNode {
        InvoiceNode { invoice }
    }
}

// INVOICE LINE PRICING
pub struct InvoicePricingNode {
    invoice_pricing: InvoiceStatsRow,
}

#[Object]
impl InvoicePricingNode {
    // total

    pub async fn total_before_tax(&self) -> f64 {
        self.invoice_pricing.total_before_tax
    }

    pub async fn total_after_tax(&self) -> f64 {
        self.invoice_pricing.total_after_tax
    }

    // stock

    pub async fn stock_total_before_tax(&self) -> f64 {
        self.invoice_pricing.stock_total_before_tax
    }

    pub async fn stock_total_after_tax(&self) -> f64 {
        self.invoice_pricing.stock_total_after_tax
    }

    // service

    pub async fn service_total_before_tax(&self) -> f64 {
        self.invoice_pricing.service_total_before_tax
    }

    pub async fn service_total_after_tax(&self) -> f64 {
        self.invoice_pricing.service_total_after_tax
    }
}

#[derive(Union)]
pub enum InvoicePriceResponse {
    Error(NodeError),
    Response(InvoicePricingNode),
}

impl InvoiceConnector {
    pub fn from_domain(invoices: ListResult<Invoice>) -> InvoiceConnector {
        InvoiceConnector {
            total_count: invoices.count,
            nodes: invoices
                .rows
                .into_iter()
                .map(InvoiceNode::from_domain)
                .collect(),
        }
    }

    pub fn from_vec(invoices: Vec<Invoice>) -> InvoiceConnector {
        InvoiceConnector {
            total_count: usize_to_u32(invoices.len()),
            nodes: invoices.into_iter().map(InvoiceNode::from_domain).collect(),
        }
    }
}
