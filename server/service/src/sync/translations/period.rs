use chrono::NaiveDate;
use repository::{PeriodRow, StorageConnection, SyncBufferRow};
use serde::{Deserialize, Serialize};

use super::{
    IntegrationRecords, LegacyTableName, PullDeleteRecordTable, PullUpsertRecord, SyncTranslation,
};

const LEGACY_TABLE_NAME: &'static str = LegacyTableName::PERIOD;

fn match_pull_table(sync_record: &SyncBufferRow) -> bool {
    sync_record.table_name == LEGACY_TABLE_NAME
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct LegacyPeriodRow {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "periodScheduleID")]
    pub period_schedule_id: String,
    #[serde(rename = "startDate")]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    pub end_date: NaiveDate,
    pub name: String,
}

pub(crate) struct PeriodTranslation {}
impl SyncTranslation for PeriodTranslation {
    fn try_translate_pull_upsert(
        &self,
        _: &StorageConnection,
        sync_record: &SyncBufferRow,
    ) -> Result<Option<IntegrationRecords>, anyhow::Error> {
        if !match_pull_table(sync_record) {
            return Ok(None);
        }

        let data = serde_json::from_str::<LegacyPeriodRow>(&sync_record.data)?;

        let result = PeriodRow {
            id: data.id.to_string(),
            period_schedule_id: data.period_schedule_id.to_string(),
            start_date: data.start_date,
            end_date: data.end_date,
            name: data.name.to_string(),
        };

        Ok(Some(IntegrationRecords::from_upsert(
            PullUpsertRecord::Period(result),
        )))
    }

    fn try_translate_pull_delete(
        &self,
        _: &StorageConnection,
        sync_record: &SyncBufferRow,
    ) -> Result<Option<IntegrationRecords>, anyhow::Error> {
        let result = match_pull_table(sync_record).then(|| {
            IntegrationRecords::from_delete(&sync_record.record_id, PullDeleteRecordTable::Period)
        });

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repository::{mock::MockDataInserts, test_db::setup_all};

    #[actix_rt::test]
    async fn test_period_translation() {
        use crate::sync::test::test_data::period as test_data;
        let translator = PeriodTranslation {};

        let (_, connection, _, _) =
            setup_all("test_period_translation", MockDataInserts::none()).await;

        for record in test_data::test_pull_upsert_records() {
            let translation_result = translator
                .try_translate_pull_upsert(&connection, &record.sync_buffer_row)
                .unwrap();

            assert_eq!(translation_result, record.translated_record);
        }

        for record in test_data::test_pull_delete_records() {
            let translation_result = translator
                .try_translate_pull_delete(&connection, &record.sync_buffer_row)
                .unwrap();

            assert_eq!(translation_result, record.translated_record);
        }
    }
}
