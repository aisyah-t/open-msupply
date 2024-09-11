use super::{vaccination_card::vaccination_card::dsl as vaccination_card_dsl, StorageConnection};

use crate::{diesel_macros::apply_equal_filter, RepositoryError};
use diesel::prelude::*;

table! {
    vaccination_card (id) {
        id -> Text,
        vaccine_course_dose_id -> Text,
        label -> Text,
        min_interval_days -> Integer,
        min_age -> Double,
        program_enrolment_id -> Text,
        vaccination_id -> Nullable<Text>,
        vaccination_date -> Nullable<Date>,
        given -> Nullable<Bool>,
        stock_line_id -> Nullable<Text>,
    }
}

use chrono::NaiveDate;

#[derive(Clone, Queryable, Debug, PartialEq)]
#[diesel(table_name = vaccination_card)]
pub struct VaccinationCardRow {
    pub id: String,
    pub vaccine_course_dose_id: String,
    pub label: String,
    pub min_interval_days: i32,
    pub min_age: f64,
    pub program_enrolment_id: String,
    pub vaccination_id: Option<String>,
    pub vaccination_date: Option<NaiveDate>,
    pub given: Option<bool>,
    pub stock_line_id: Option<String>,
}

pub struct VaccinationCardRepository<'a> {
    connection: &'a StorageConnection,
}

impl<'a> VaccinationCardRepository<'a> {
    pub fn new(connection: &'a StorageConnection) -> Self {
        VaccinationCardRepository { connection }
    }

    pub fn query_by_enrolment_id(
        &self,
        program_enrolment_id: String,
    ) -> Result<Vec<VaccinationCardRow>, RepositoryError> {
        let mut query = vaccination_card_dsl::vaccination_card.into_boxed();

        apply_equal_filter!(
            query,
            program_enrolment_id,
            vaccination_card_dsl::program_enrolment_id
        );

        Ok(query.load::<VaccinationCardRow>(self.connection.lock().connection())?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{mock::MockDataInserts, test_db, VaccinationCardRepository};

    #[actix_rt::test]
    async fn test_vaccination_card() {
        let (_, connection, _, _) =
            test_db::setup_all("test_vaccination_card", MockDataInserts::all()).await;

        VaccinationCardRepository::new(&connection)
            .query_by_enrolment_id(mock_immunisation_program_enrolment_a().id) // TODO fix this
            // get back 11 (not)
            .expect("Failed to query vaccination card by enrolment id");
    }
}
