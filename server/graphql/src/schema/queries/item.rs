use crate::{
    schema::types::{
        sort_filter_types::{
            convert_sort, EqualFilterBoolInput, EqualFilterStringInput, SimpleStringFilterInput,
            SortInput,
        },
        ItemConnector, PaginationInput,
    },
    standard_graphql_error::StandardGraphqlError,
    ContextExt,
};
use async_graphql::*;
use domain::{EqualFilter, PaginationOption, SimpleStringFilter};
use repository::ItemFilter;
use service::item::get_items;

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
#[graphql(remote = "domain::item::ItemSortField")]
#[graphql(rename_items = "camelCase")]
pub enum ItemSortFieldInput {
    Name,
    Code,
}
pub type ItemSortInput = SortInput<ItemSortFieldInput>;

#[derive(InputObject, Clone)]
pub struct ItemFilterInput {
    pub id: Option<EqualFilterStringInput>,
    pub name: Option<SimpleStringFilterInput>,
    pub code: Option<SimpleStringFilterInput>,
    pub is_visible: Option<EqualFilterBoolInput>,
}

impl From<ItemFilterInput> for ItemFilter {
    fn from(f: ItemFilterInput) -> Self {
        ItemFilter {
            id: f.id.map(EqualFilter::from),
            name: f.name.map(SimpleStringFilter::from),
            code: f.code.map(SimpleStringFilter::from),
            is_visible: f.is_visible.and_then(|filter| filter.equal_to),
            r#type: None,
        }
    }
}

#[derive(Union)]
pub enum ItemsResponse {
    Response(ItemConnector),
}

pub fn items(
    ctx: &Context<'_>,
    page: Option<PaginationInput>,
    filter: Option<ItemFilterInput>,
    sort: Option<Vec<ItemSortInput>>,
) -> Result<ItemsResponse> {
    let connection_manager = ctx.get_connection_manager();
    let items = get_items(
        connection_manager,
        page.map(PaginationOption::from),
        filter.map(ItemFilter::from),
        convert_sort(sort),
    )
    .map_err(StandardGraphqlError::from_list_error)?;

    Ok(ItemsResponse::Response(ItemConnector::from_domain(items)))
}
