use crate::service_provider::ServiceContext;
use repository::asset::{AssetFilter, AssetRepository};
use repository::asset_catalogue_item_property::{
    AssetCatalogueItemPropertyRepository, AssetCataloguePropertyItemFilter,
};
use repository::asset_catalogue_item_property_row::AssetCatalogueItemPropertyRowRepository;
use repository::asset_catalogue_item_row::AssetCatalogueItemRowRepository;
use repository::EqualFilter;
use repository::{RepositoryError, StorageConnection};

#[derive(PartialEq, Debug)]
pub enum DeleteAssetCatalogueItemError {
    AssetCatalogueItemDoesNotExist,
    AssetCatalogueItemInUse,
    DatabaseError(RepositoryError),
}

pub fn delete_asset_catalogue_item(
    ctx: &ServiceContext,
    id: String,
) -> Result<String, DeleteAssetCatalogueItemError> {
    let property_ids = AssetCatalogueItemPropertyRepository::new(&ctx.connection)
        .query(Some(
            AssetCataloguePropertyItemFilter::new().catalogue_item_id(EqualFilter::equal_to(&id)),
        ))?
        .into_iter()
        .map(|property| property.id)
        .collect::<Vec<String>>();
    let asset_catalogue_item_id = ctx
        .connection
        .transaction_sync(|connection| {
            validate(connection, &id)?;
            property_ids.into_iter().for_each(|id| {
                match AssetCatalogueItemPropertyRowRepository::new(connection).delete(&id) {
                    Ok(_) => (),
                    Err(_) => (),
                };
            });
            match AssetCatalogueItemRowRepository::new(connection).delete(&id) {
                Ok(_) => Ok(id),
                Err(err) => Err(DeleteAssetCatalogueItemError::from(err)),
            }
        })
        .map_err(|error| error.to_inner_error())?;
    Ok(asset_catalogue_item_id)
}

pub fn validate(
    connection: &StorageConnection,
    id: &str,
) -> Result<(), DeleteAssetCatalogueItemError> {
    let _asset_catalogue_item_row =
        match AssetCatalogueItemRowRepository::new(connection).find_one_by_id(id)? {
            Some(asset_catalogue_item_row) => asset_catalogue_item_row,
            None => return Err(DeleteAssetCatalogueItemError::AssetCatalogueItemDoesNotExist),
        };

    let assets_using_item = AssetRepository::new(connection).count(Some(
        AssetFilter::new().catalogue_item_id(EqualFilter::equal_to(id)),
    ))?;
    if assets_using_item > 0 {
        return Err(DeleteAssetCatalogueItemError::AssetCatalogueItemInUse);
    }
    Ok(())
}

impl From<RepositoryError> for DeleteAssetCatalogueItemError {
    fn from(error: RepositoryError) -> Self {
        DeleteAssetCatalogueItemError::DatabaseError(error)
    }
}