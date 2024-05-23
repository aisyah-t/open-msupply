use crate::migrations::DATETIME;
use crate::{migrations::sql, StorageConnection};

pub(crate) fn migrate(connection: &StorageConnection) -> anyhow::Result<()> {
    #[cfg(feature = "postgres")]
    sql!(
        connection,
        r#"
          CREATE TYPE asset_log_status AS ENUM (
            'NOT_IN_USE',
            'FUNCTIONING',
            'FUNCTIONING_BUT_NEEDS_ATTENTION',
            'NOT_FUNCTIONING',
            'DECOMMISSIONED'
          );
        "#
    )?;

    const ASSET_LOG_STATUS_ENUM_TYPE: &str = if cfg!(feature = "postgres") {
        "asset_log_status"
    } else {
        "TEXT"
    };

    sql!(
        connection,
        r#"
        CREATE TABLE asset_log (
            id TEXT NOT NULL PRIMARY KEY,
            asset_id TEXT NOT NULL REFERENCES asset(id),
            user_id TEXT NOT NULL,
            status {ASSET_LOG_STATUS_ENUM_TYPE},
            reason_id TEXT REFERENCES asset_log_reason(id),
            comment TEXT,
            type TEXT,
            log_datetime {DATETIME} NOT NULL
          );
        "#,
    )?;

    if cfg!(feature = "postgres") {
        sql!(
            connection,
            r#"
                ALTER TYPE changelog_table_name ADD VALUE IF NOT EXISTS 'asset_log';
            "#
        )?;
    }

    Ok(())
}
