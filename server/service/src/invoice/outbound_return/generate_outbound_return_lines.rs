use crate::{service_provider::ServiceContext, ListError, ListResult};
use repository::{EqualFilter, StockLine, StockLineFilter, StockLineRepository};
use util::uuid::uuid;

#[derive(Debug, Clone)]
pub struct OutboundReturnLine {
    pub id: String,
    pub reason_id: Option<String>,
    pub comment: Option<String>,
    pub number_of_packs: u32,
    pub stock_line: StockLine,
}

pub fn generate_outbound_return_lines(
    ctx: &ServiceContext,
    store_id: &str,
    stock_line_ids: Vec<String>,
    item_id: Option<String>,
    return_id: Option<String>,
) -> Result<ListResult<OutboundReturnLine>, ListError> {
    if stock_line_ids.is_empty() && item_id.is_none() && return_id.is_none() {
        return Ok(ListResult {
            count: 0,
            rows: vec![],
        });
    }

    let mut filter = StockLineFilter::new().is_available(true);

    if !stock_line_ids.is_empty() {
        filter.id = Some(EqualFilter::equal_any(
            stock_line_ids.iter().map(String::clone).collect(),
        ))
    }
    filter.item_id = item_id.map(|item_id| EqualFilter::equal_to(&item_id));

    let stock_lines = StockLineRepository::new(&ctx.connection)
        .query_by_filter(filter, Some(store_id.to_string()))?;

    let return_lines: Vec<OutboundReturnLine> = stock_lines
        .iter()
        .map(|stock_line| OutboundReturnLine {
            id: uuid(),
            stock_line: stock_line.clone(),

            reason_id: None,
            comment: None,
            number_of_packs: 0,
        })
        .collect();

    // TODO:
    // if return_id, query for return lines by return id

    Ok(ListResult {
        count: return_lines.len() as u32,
        rows: return_lines,
    })
}

#[cfg(test)]
mod test {
    use crate::service_provider::ServiceProvider;
    use repository::{
        mock::{mock_item_a_lines, MockDataInserts},
        test_db::setup_all,
    };

    #[actix_rt::test]
    async fn generate_outbound_return_lines_nothing_supplied() {
        let (_, _, connection_manager, _) = setup_all(
            "generate_outbound_return_lines_nothing_supplied",
            MockDataInserts::all(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.basic_context().unwrap();
        let service = service_provider.invoice_service;

        let store_id = "store_a";
        let stock_line_ids = vec![];
        let item_id = None;
        let return_id = None;

        let result = service
            .generate_outbound_return_lines(&context, store_id, stock_line_ids, item_id, return_id)
            .unwrap();

        assert_eq!(result.count, 0);
    }

    #[actix_rt::test]
    async fn generate_outbound_return_lines_stock_line_ids() {
        let (_, _, connection_manager, _) = setup_all(
            "generate_outbound_return_lines_stock_line_ids",
            MockDataInserts::all(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.basic_context().unwrap();
        let service = service_provider.invoice_service;

        let store_id = "store_a";
        let stock_line_ids = mock_item_a_lines()
            .iter()
            .map(|stock_line| stock_line.id.clone())
            .collect();
        let item_id = None;
        let return_id = None;

        let result = service
            .generate_outbound_return_lines(&context, store_id, stock_line_ids, item_id, return_id)
            .unwrap();

        assert_eq!(result.count, 2);
        assert_eq!(result.rows[0].stock_line.item_row.id, "item_a");
    }

    #[actix_rt::test]
    async fn generate_inbound_return_lines_item_id() {
        let (_, _, connection_manager, _) = setup_all(
            "generate_inbound_return_lines_item_id",
            MockDataInserts::all(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.basic_context().unwrap();
        let service = service_provider.invoice_service;

        let store_id = "store_a";
        let stock_line_ids = vec![];
        let item_id = Some("item_query_test1".to_string());
        let return_id = None;

        let result = service
            .generate_outbound_return_lines(&context, store_id, stock_line_ids, item_id, return_id)
            .unwrap();

        assert_eq!(result.count, 1);
        assert_eq!(
            result.rows[0].stock_line.stock_line_row.id,
            "item_query_test1"
        );
    }
}
