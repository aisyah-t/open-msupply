use crate::{
    activity_log::activity_log_entry,
    service_provider::ServiceContext,
    validate::{check_other_party, CheckOtherPartyType, OtherPartyErrors},
};

use chrono::Utc;
use repository::{
    ActivityLogType, PeriodRow, ProgramRequisitionSettingsRowRepository, RepositoryError, RnRForm,
    RnRFormLineRow, RnRFormLineRowRepository, RnRFormRow, RnRFormRowRepository, RnRFormStatus,
    StorageConnection,
};

use super::{
    generate_rnr_form_lines::generate_rnr_form_lines,
    query::get_rnr_form,
    validate::{
        check_master_list_exists, check_period_exists, check_program_exists,
        check_rnr_form_does_not_exist, check_rnr_form_exists_for_period,
    },
};
#[derive(Default, Debug, PartialEq, Clone)]
pub struct InsertRnRForm {
    pub id: String,
    pub supplier_id: String,
    pub program_id: String,
    pub period_id: String,
}

#[derive(Debug, PartialEq)]
pub enum InsertRnRFormError {
    DatabaseError(RepositoryError),
    InternalError(String),
    RnRFormAlreadyExists,
    SupplierDoesNotExist,
    SupplierNotVisible,
    NotASupplier,
    ProgramDoesNotExist,
    ProgramHasNoMasterList,
    PeriodDoesNotExist,
    PeriodNotInProgramSchedule,
    RnRFormAlreadyExistsForPeriod,
    NewlyCreatedRnRFormDoesNotExist,
}

pub fn insert_rnr_form(
    ctx: &ServiceContext,
    input: InsertRnRForm,
) -> Result<RnRForm, InsertRnRFormError> {
    let rnr_form = ctx
        .connection
        .transaction_sync(|connection| {
            let (period_row, master_list_id) = validate(connection, &ctx.store_id, &input)?;
            let (rnr_form, rnr_form_lines) = generate(ctx, input, period_row, &master_list_id)?;

            let rnr_form_repo = RnRFormRowRepository::new(connection);
            let rnr_form_line_repo = RnRFormLineRowRepository::new(connection);

            rnr_form_repo.upsert_one(&rnr_form)?;

            for line in rnr_form_lines {
                rnr_form_line_repo.upsert_one(&line)?;
            }

            activity_log_entry(
                ctx,
                ActivityLogType::RnrFormCreated,
                Some(rnr_form.id.clone()),
                None,
                None,
            )?;

            get_rnr_form(ctx, rnr_form.id)
                .map_err(InsertRnRFormError::DatabaseError)?
                .ok_or(InsertRnRFormError::NewlyCreatedRnRFormDoesNotExist)
        })
        .map_err(|err| err.to_inner_error())?;

    Ok(rnr_form)
}

fn validate(
    connection: &StorageConnection,
    store_id: &str,
    input: &InsertRnRForm,
) -> Result<(PeriodRow, String), InsertRnRFormError> {
    if !check_rnr_form_does_not_exist(connection, &input.id)? {
        return Err(InsertRnRFormError::RnRFormAlreadyExists);
    }

    check_other_party(
        connection,
        store_id,
        &input.supplier_id,
        CheckOtherPartyType::Supplier,
    )?;

    let program = check_program_exists(connection, &input.program_id)?
        .ok_or(InsertRnRFormError::ProgramDoesNotExist)?;

    let master_list_id = match program.master_list_id {
        Some(id) => id,
        None => return Err(InsertRnRFormError::ProgramHasNoMasterList),
    };

    if !check_master_list_exists(connection, store_id, &master_list_id)? {
        return Err(InsertRnRFormError::ProgramHasNoMasterList);
    }

    let period = check_period_exists(connection, &input.period_id)?
        .ok_or(InsertRnRFormError::PeriodDoesNotExist)?;

    // find all period schedules for the provided program
    let period_schedule_ids = ProgramRequisitionSettingsRowRepository::new(connection)
        .find_many_by_program_id(&input.program_id)?
        .iter()
        .map(|s| s.period_schedule_id.clone())
        .collect::<Vec<String>>();

    // check period is part of one of those schedules
    if !period_schedule_ids.contains(&period.period_schedule_id) {
        return Err(InsertRnRFormError::PeriodNotInProgramSchedule);
    }

    if check_rnr_form_exists_for_period(connection, &input.period_id, &input.program_id)?.is_some()
    {
        return Err(InsertRnRFormError::RnRFormAlreadyExistsForPeriod);
    };

    Ok((period, master_list_id))
}

fn generate(
    ctx: &ServiceContext,
    InsertRnRForm {
        id,
        supplier_id,
        program_id,
        period_id,
    }: InsertRnRForm,
    period: PeriodRow,
    master_list_id: &str,
) -> Result<(RnRFormRow, Vec<RnRFormLineRow>), RepositoryError> {
    let current_datetime = Utc::now().naive_utc();

    let rnr_form = RnRFormRow {
        id,
        period_id,
        program_id,
        name_link_id: supplier_id,
        created_datetime: current_datetime,
        store_id: ctx.store_id.clone(),
        // default
        finalised_datetime: None,
        status: RnRFormStatus::Draft,
        linked_requisition_id: None,
    };

    let rnr_form_lines = generate_rnr_form_lines(ctx, &rnr_form.id, master_list_id, period)?;

    Ok((rnr_form, rnr_form_lines))
}

impl From<RepositoryError> for InsertRnRFormError {
    fn from(error: RepositoryError) -> Self {
        InsertRnRFormError::DatabaseError(error)
    }
}

impl From<OtherPartyErrors> for InsertRnRFormError {
    fn from(error: OtherPartyErrors) -> Self {
        match error {
            OtherPartyErrors::OtherPartyDoesNotExist => InsertRnRFormError::SupplierDoesNotExist,
            OtherPartyErrors::OtherPartyNotVisible => InsertRnRFormError::SupplierNotVisible,
            OtherPartyErrors::TypeMismatched => InsertRnRFormError::NotASupplier,
            OtherPartyErrors::DatabaseError(err) => InsertRnRFormError::DatabaseError(err),
        }
    }
}
