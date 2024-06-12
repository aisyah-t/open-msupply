use super::{
    sensor_row::sensor, store_row::store,
    temperature_log_row::temperature_log::dsl as temperature_log_dsl, StorageConnection,
};

use crate::{repository_error::RepositoryError, Upsert};

use chrono::NaiveDateTime;
use diesel::prelude::*;

table! {
    temperature_log (id) {
        id -> Text,
        temperature -> Double,
        sensor_id -> Text,
        location_id -> Nullable<Text>,
        store_id -> Text,
        datetime -> Timestamp,
        temperature_breach_id -> Nullable<Text>,
    }
}

joinable!(temperature_log -> sensor (sensor_id));
joinable!(temperature_log -> store (store_id));

#[derive(
    Clone, Queryable, Insertable, AsChangeset, Debug, PartialEq, Default, serde::Serialize,
)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = temperature_log)]
pub struct TemperatureLogRow {
    pub id: String,
    pub temperature: f64,
    pub sensor_id: String,
    pub location_id: Option<String>,
    pub store_id: String,
    pub datetime: NaiveDateTime,
    pub temperature_breach_id: Option<String>,
}

pub struct TemperatureLogRowRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> TemperatureLogRowRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        TemperatureLogRowRepository { connection }
    }

    pub fn _upsert_one(&self, row: &TemperatureLogRow) -> Result<(), RepositoryError> {
        diesel::insert_into(temperature_log_dsl::temperature_log)
            .values(row)
            .on_conflict(temperature_log_dsl::id)
            .do_update()
            .set(row)
            .execute(self.connection.lock().connection())?;
        Ok(())
    }

    pub fn upsert_one(&self, row: &TemperatureLogRow) -> Result<(), RepositoryError> {
        self._upsert_one(row)?;
        Ok(())
    }

    pub fn update_breach_id(
        &self,
        breach_id: &str,
        temperature_log_ids: &Vec<String>,
    ) -> Result<(), RepositoryError> {
        diesel::update(temperature_log_dsl::temperature_log)
            .filter(temperature_log_dsl::id.eq_any(temperature_log_ids))
            .set(temperature_log_dsl::temperature_breach_id.eq(breach_id))
            .execute(self.connection.lock().connection())?;
        Ok(())
    }

    pub fn find_one_by_id(&self, id: &str) -> Result<Option<TemperatureLogRow>, RepositoryError> {
        let result = temperature_log_dsl::temperature_log
            .filter(temperature_log_dsl::id.eq(id))
            .first(self.connection.lock().connection())
            .optional()?;
        Ok(result)
    }

    pub fn find_many_by_id(
        &self,
        ids: &[String],
    ) -> Result<Vec<TemperatureLogRow>, RepositoryError> {
        Ok(temperature_log_dsl::temperature_log
            .filter(temperature_log_dsl::id.eq_any(ids))
            .load(self.connection.lock().connection())?)
    }
}

impl Upsert for TemperatureLogRow {
    fn upsert_sync(&self, con: &StorageConnection) -> Result<(), RepositoryError> {
        TemperatureLogRowRepository::new(con).upsert_one(self)
    }

    // Test only
    fn assert_upserted(&self, con: &StorageConnection) {
        assert_eq!(
            TemperatureLogRowRepository::new(con).find_one_by_id(&self.id),
            Ok(Some(self.clone()))
        )
    }
}
