use super::{item_row::item::dsl::*, unit_row::unit, StorageConnection};

use crate::{item_link, repository_error::RepositoryError, ItemLinkRowRepository};

use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

table! {
    item (id) {
        id -> Text,
        name -> Text,
        code -> Text,
        unit_id -> Nullable<Text>,
        default_pack_size -> Integer,
        #[sql_name = "type"] type_ -> crate::db_diesel::item_row::ItemRowTypeMapping,
        // TODO, this is temporary, remove
        legacy_record -> Text,
        is_active -> Bool,
    }
}

table! {
    item_is_visible (id) {
        id -> Text,
        is_visible -> Bool,
    }
}

joinable!(item -> unit (unit_id));
joinable!(item_is_visible -> item (id));
allow_tables_to_appear_in_same_query!(item, item_link);

#[derive(DbEnum, Debug, Clone, PartialEq, Eq)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum ItemRowType {
    Stock,
    Service,
    NonStock,
}

#[derive(Clone, Insertable, Queryable, Debug, PartialEq, AsChangeset, Eq)]
#[table_name = "item"]
pub struct ItemRow {
    pub id: String,
    pub name: String,
    pub code: String,
    pub unit_id: Option<String>,
    pub default_pack_size: i32,
    #[column_name = "type_"]
    pub r#type: ItemRowType,
    // TODO, this is temporary, remove
    pub legacy_record: String,
    pub is_active: bool,
}

impl Default for ItemRow {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            code: Default::default(),
            unit_id: Default::default(),
            default_pack_size: Default::default(),
            r#type: ItemRowType::Stock,
            legacy_record: Default::default(),
            is_active: true,
        }
    }
}

pub struct ItemRowRepository<'a> {
    connection: &'a StorageConnection,
}

fn upsert_item_link<'a>(
    connection: &'a StorageConnection,
    item_row: &ItemRow,
) -> Result<(), RepositoryError> {
    ItemLinkRowRepository::new(connection).insert_one_from_item(item_row)?;
    Ok(())
}

impl<'a> ItemRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        ItemRowRepository { connection }
    }

    #[cfg(feature = "postgres")]
    pub fn upsert_one(&self, item_row: &ItemRow) -> Result<(), RepositoryError> {
        diesel::insert_into(item)
            .values(item_row)
            .on_conflict(id)
            .do_update()
            .set(item_row)
            .execute(&self.connection.connection)?;

        upsert_item_link(&self.connection, item_row)?;

        Ok(())
    }

    #[cfg(not(feature = "postgres"))]
    pub fn upsert_one(&self, item_row: &ItemRow) -> Result<(), RepositoryError> {
        diesel::replace_into(item)
            .values(item_row)
            .execute(&self.connection.connection)?;

        upsert_item_link(&self.connection, item_row)?;
        Ok(())
    }

    pub async fn insert_one(&self, item_row: &ItemRow) -> Result<(), RepositoryError> {
        diesel::insert_into(item)
            .values(item_row)
            .execute(&self.connection.connection)?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<ItemRow>, RepositoryError> {
        let result = item.load(&self.connection.connection);
        Ok(result?)
    }

    pub fn find_one_by_id(&self, item_id: &str) -> Result<Option<ItemRow>, RepositoryError> {
        let result = item
            .filter(id.eq(item_id).and(is_active.eq(true)))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn find_inactive_by_id(&self, item_id: &str) -> Result<Option<ItemRow>, RepositoryError> {
        let result = item
            .filter(id.eq(item_id).and(is_active.eq(false)))
            .first(&self.connection.connection)
            .optional()?;
        Ok(result)
    }

    pub fn find_many_by_id(&self, ids: &[String]) -> Result<Vec<ItemRow>, RepositoryError> {
        let result = item
            .filter(id.eq_any(ids))
            .load(&self.connection.connection)?;
        Ok(result)
    }

    pub fn delete(&self, item_id: &str) -> Result<(), RepositoryError> {
        diesel::update(item.filter(id.eq(item_id)))
            .set(is_active.eq(false))
            .execute(&self.connection.connection)?;
        Ok(())
    }
}
