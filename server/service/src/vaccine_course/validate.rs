use repository::{
    vaccine_course::vaccine_course_row::{VaccineCourseRow, VaccineCourseRowRepository},
    ProgramRow, ProgramRowRepository, RepositoryError, StorageConnection,
};

pub fn check_vaccine_course_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<VaccineCourseRow>, RepositoryError> {
    VaccineCourseRowRepository::new(connection).find_one_by_id(id)
}

pub fn check_program_exists(
    id: &str,
    connection: &StorageConnection,
) -> Result<Option<ProgramRow>, RepositoryError> {
    ProgramRowRepository::new(connection).find_one_by_id(id)
}