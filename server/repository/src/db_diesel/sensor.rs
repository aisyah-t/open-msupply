use super::{
    sensor_row::{sensor, sensor::dsl as sensor_dsl},
    SensorRow, DBType, StorageConnection,
};
use diesel::prelude::*;

use crate::{
    diesel_macros::{apply_equal_filter, apply_sort_no_case},
    repository_error::RepositoryError,
};

use crate::{EqualFilter, Pagination, Sort};

#[derive(PartialEq, Debug, Clone)]
pub struct Sensor {
    pub sensor_row: SensorRow,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SensorFilter {
    pub id: Option<EqualFilter<String>>,
    pub name: Option<EqualFilter<String>>,
    pub serial: Option<EqualFilter<String>>,
}

#[derive(PartialEq, Debug)]
pub enum SensorSortField {
    Id,
    Sensor,
}

pub type SensorSort = Sort<SensorSortField>;

pub struct SensorRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> SensorRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        SensorRepository { connection }
    }

    pub fn count(&self, filter: Option<SensorFilter>) -> Result<i64, RepositoryError> {
        let query = create_filtered_query(filter);
        Ok(query.count().get_result(&self.connection.connection)?)
    }

    pub fn query_by_filter(&self, filter: SensorFilter) -> Result<Vec<Sensor>, RepositoryError> {
        self.query(Pagination::all(), Some(filter), None)
    }

    pub fn query(
        &self,
        pagination: Pagination,
        filter: Option<SensorFilter>,
        sort: Option<SensorSort>,
    ) -> Result<Vec<Sensor>, RepositoryError> {
        let mut query = create_filtered_query(filter);
        if let Some(sort) = sort {
            match sort.key {
                SensorSortField::Id => {
                    apply_sort_no_case!(query, sort, sensor_dsl::id)
                }
                SensorSortField::Sensor => {
                    apply_sort_no_case!(query, sort, sensor_dsl::serial)
                }
                SensorSortField::Sensor => {
                    apply_sort_no_case!(query, sort, sensor_dsl::name)
                }
            }
        } else {
            query = query.order(sensor_dsl::name.asc())
        }

        let result = query
            .offset(pagination.offset as i64)
            .limit(pagination.limit as i64)
            .load::<SensorRow>(&self.connection.connection)?;

        Ok(result.into_iter().map(to_domain).collect())
    }
}

type BoxedLogQuery = sensor::BoxedQuery<'static, DBType>;

fn create_filtered_query(filter: Option<SensorFilter>) -> BoxedLogQuery {
    let mut query = sensor::table.into_boxed();

    if let Some(filter) = filter {
        apply_equal_filter!(query, filter.id, sensor_dsl::id);
        apply_equal_filter!(query, filter.name, sensor_dsl::name);
        apply_equal_filter!(query, filter.serial, sensor_dsl::serial);
    }

    query
}

pub fn to_domain(sensor_row: SensorRow) -> Sensor {
    Sensor { sensor_row }
}

impl SensorFilter {
    pub fn new() -> SensorFilter {
        SensorFilter {
            id: None,
            name: None,
            serial: None,
        }
    }

    pub fn id(mut self, filter: EqualFilter<String>) -> Self {
        self.id = Some(filter);
        self
    }

    pub fn name(mut self, filter: EqualFilter<String>) -> Self {
        self.name = Some(filter);
        self
    }

    pub fn serial(mut self, filter: EqualFilter<String>) -> Self {
        self.serial = Some(filter);
        self
    }

}
