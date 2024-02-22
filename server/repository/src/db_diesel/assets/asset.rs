use super::asset_row::{
    asset::{self, dsl as asset_dsl},
    AssetRow,
};

use diesel::{dsl::IntoBoxed, prelude::*};

use crate::{
    diesel_macros::{
        apply_date_filter, apply_equal_filter, apply_sort, apply_sort_no_case, apply_string_filter,
    },
    repository_error::RepositoryError,
    DBType, DateFilter, DatetimeFilter, EqualFilter, Pagination, Sort, StorageConnection,
    StringFilter,
};

type Asset = AssetRow;

pub enum AssetSortField {
    SerialNumber,
    InstallationDate,
    ReplacementDate,
}

pub type AssetSort = Sort<AssetSortField>;

#[derive(Clone)]
pub struct AssetFilter {
    pub id: Option<EqualFilter<String>>,
    pub serial_number: Option<StringFilter>,
    pub category_id: Option<EqualFilter<String>>,
    pub type_id: Option<EqualFilter<String>>,
    pub catalogue_item_id: Option<EqualFilter<String>>,
    pub installation_date: Option<DateFilter>,
    pub replacement_date: Option<DateFilter>,
}

impl AssetFilter {
    pub fn new() -> AssetFilter {
        AssetFilter {
            id: None,
            serial_number: None,
            category_id: None,
            type_id: None,
            catalogue_item_id: None,
            installation_date: None,
            replacement_date: None,
        }
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }

    pub fn serial_number(mut self, filter: StringFilter) -> Self {
        self.serial_number = Some(filter);
        self
    }

    pub fn category_id(mut self, filter: EqualFilter<String>) -> Self {
        self.category_id = Some(filter);
        self
    }

    pub fn type_id(mut self, filter: EqualFilter<String>) -> Self {
        self.type_id = Some(filter);
        self
    }

    pub fn catalogue_item_id(mut self, filter: EqualFilter<String>) -> Self {
        self.catalogue_item_id = Some(filter);
        self
    }

    pub fn installation_date(mut self, filter: DateFilter) -> Self {
        self.installation_date = Some(filter);
        self
    }

    pub fn replacement_date(mut self, filter: DateFilter) -> Self {
        self.replacement_date = Some(filter);
        self
    }
}

pub struct AssetRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> AssetRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        AssetRepository { connection }
    }

    pub fn count(&self, filter: Option<AssetFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);

        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_one(&self, filter: AssetFilter) -> Result<Option<Asset>, RepositoryError> {
        Ok(self.query_by_filter(filter)?.pop())
    }

    pub fn query_by_filter(&self, filter: AssetFilter) -> Result<Vec<Asset>, RepositoryError> {
        self.query(Pagination::all(), Some(filter), None)
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<AssetFilter>,
        sort: Option<AssetSort>,
    ) -> Result<Vec<Asset>, RepositoryError> {
        let mut query = create_filtered_query(filter);

        if let Some(sort) = sort {
            match sort.key {
                AssetSortField::SerialNumber => {
                    apply_sort_no_case!(query, sort, asset_dsl::serial_number);
                }
                AssetSortField::InstallationDate => {
                    apply_sort!(query, sort, asset_dsl::installation_date)
                }
                AssetSortField::ReplacementDate => {
                    apply_sort!(query, sort, asset_dsl::replacement_date)
                }
            }
        } else {
            query = query.order(asset_dsl::id.asc())
        }

        let final_query = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64);

        // Debug diesel query
        // println!(
        //    "{}",
        //     diesel::debug_query::<DBType, _>(&final_query).to_string()
        // );

        let result = final_query.load::<Asset>(&self.connection.connection)?;

        Ok(result.into_iter().map(to_domain).collect())
    }
}

fn to_domain(asset_row: AssetRow) -> Asset {
    asset_row
}

type BoxedAssetQuery = IntoBoxed<'static, asset::table, DBType>;

fn create_filtered_query(filter: Option<AssetFilter>) -> BoxedAssetQuery {
    let mut query = asset_dsl::asset.into_boxed();

    if let Some(f) = filter {
        let AssetFilter {
            id,
            serial_number,
            category_id,
            type_id,
            catalogue_item_id,
            installation_date,
            replacement_date,
        } = f;

        apply_equal_filter!(query, id, asset_dsl::id);
        apply_string_filter!(query, serial_number, asset_dsl::serial_number);
        apply_equal_filter!(query, category_id, asset_dsl::asset_category_id);
        apply_equal_filter!(query, type_id, asset_dsl::asset_type_id);
        apply_equal_filter!(query, catalogue_item_id, asset_dsl::catalogue_item_id);
        apply_date_filter!(query, installation_date, asset_dsl::installation_date);
        apply_date_filter!(query, replacement_date, asset_dsl::replacement_date);
    }
    query
}

#[cfg(test)]
mod tests {
    use crate::{
        assets::{
            asset::AssetRepository,
            asset_category_row::{AssetCategoryRow, AssetCategoryRowRepository},
            asset_class_row::{AssetClassRow, AssetClassRowRepository},
            asset_row::{AssetRow, AssetRowRepository},
            asset_type_row::{AssetTypeRow, AssetTypeRowRepository},
        },
        mock::MockDataInserts,
        test_db, EqualFilter,
    };

    use super::AssetFilter;

    #[actix_rt::test]
    async fn test_asset_query_repository() {
        // Prepare
        let (_, storage_connection, _, _) =
            test_db::setup_all("test_asset_query_repository", MockDataInserts::none()).await;

        // TODO: Replace this reference data with mock data, or inserted data from https://github.com/msupply-foundation/open-msupply/issues/3035

        // Create a class row
        let class_id = "test_class_id".to_string();
        let class_name = "test_class_name".to_string();
        let class_row = AssetClassRow {
            id: class_id.clone(),
            name: class_name.clone(),
        };
        let class_row_repo = AssetClassRowRepository::new(&storage_connection);
        class_row_repo.insert_one(&class_row).unwrap();

        // Create a category
        let category_id = "test_category_id".to_string();
        let category_name = "test_category_name".to_string();
        let category_row = AssetCategoryRow {
            id: category_id.clone(),
            name: category_name.clone(),
            class_id: class_id.clone(),
        };
        let category_row_repo = AssetCategoryRowRepository::new(&storage_connection);
        category_row_repo.insert_one(&category_row).unwrap();

        // Create the type
        let type_id = "test_type_id".to_string();
        let type_name = "test_type_name".to_string();

        // Insert a row
        let type_row_repository = AssetTypeRowRepository::new(&storage_connection);
        type_row_repository
            .insert_one(&AssetTypeRow {
                id: type_id.clone(),
                name: type_name.clone(),
                category_id: category_id.clone(),
            })
            .unwrap();

        // Create an asset
        let asset_repository = AssetRepository::new(&storage_connection);
        let asset_row_repository = AssetRowRepository::new(&storage_connection);

        let asset_id = "test_asset_id".to_string();
        let serial_number = "test_serial_number".to_string();
        let asset = AssetRow {
            id: asset_id.clone(),
            serial_number: serial_number.clone(),
            category_id: category_id.clone(),
            type_id: type_id.clone(),
            ..Default::default()
        };

        let _result = asset_row_repository.insert_one(&asset).unwrap();

        // Query by id
        let result = asset_repository
            .query_one(AssetFilter::new().id(EqualFilter::equal_to(&asset_id)))
            .unwrap()
            .unwrap();
        assert_eq!(result.id, asset_id);
        assert_eq!(result.serial_number, serial_number);
    }
}
