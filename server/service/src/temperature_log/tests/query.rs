#[cfg(test)]
mod query {
    use repository::{
        mock::MockDataInserts,
        temperature_log::{TemperatureLogFilter, TemperatureLogSortField},
        test_db::setup_all,
    };
    use repository::{EqualFilter, PaginationOption, Sort};

    use crate::{service_provider::ServiceProvider, ListError, SingleRecordError};

    #[actix_rt::test]
    async fn temperature_log_service_pagination() {
        let (_, _, connection_manager, _) =
            setup_all("test_temperature_log_service_pagination", MockDataInserts::all()).await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.basic_context().unwrap();
        let service = service_provider.temperature_log_service;

        assert_eq!(
            service.get_temperature_logs(
                &context,
                Some(PaginationOption {
                    limit: Some(2000),
                    offset: None
                }),
                None,
                None,
            ),
            Err(ListError::LimitAboveMax(1000))
        );

        assert_eq!(
            service.get_temperature_logs(
                &context,
                Some(PaginationOption {
                    limit: Some(0),
                    offset: None,
                }),
                None,
                None,
            ),
            Err(ListError::LimitBelowMin(1))
        );
    }

    #[actix_rt::test]
    async fn temperature_log_service_single_record() {
        let (_, _, connection_manager, _) =
            setup_all("test_temperature_log_single_record", MockDataInserts::all()).await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.basic_context().unwrap();
        let service = service_provider.temperature_log_service;

        assert_eq!(
            service.get_temperature_log(&context, "invalid_id".to_owned()),
            Err(SingleRecordError::NotFound("invalid_id".to_owned()))
        );

        let result = service
            .get_temperature_log(&context, "temperature_log_1a".to_owned())
            .unwrap();

        assert_eq!(result.temperature_log_row.id, "temperature_log_1a");
        assert_eq!(result.temperature_log_row.temperature, 10.6);
    }

    #[actix_rt::test]
    async fn temperature_log_service_filter() {
        let (_, _, connection_manager, _) =
            setup_all("test_temperature_log_filter", MockDataInserts::all()).await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.basic_context().unwrap();
        let service = service_provider.temperature_log_service;

        let result = service
            .get_temperature_logs(
                &context,
                None,
                Some(TemperatureLogFilter::new().id(EqualFilter::equal_to("temperature_log_1a"))),
                None,
            )
            .unwrap();

        assert_eq!(result.count, 1);
        assert_eq!(result.rows[0].temperature_log_row.id, "temperature_log_1a");

        let result = service
            .get_temperature_logs(
                &context,
                None,
                Some(TemperatureLogFilter::new().id(EqualFilter::equal_any(vec![
                    "temperature_log_1a".to_owned(),
                    "temperature_log_1b".to_owned(),
                ]))),
                None,
            )
            .unwrap();

        assert_eq!(result.count, 2);
        assert_eq!(result.rows[0].temperature_log_row.id, "temperature_log_1a");
        assert_eq!(result.rows[1].temperature_log_row.id, "temperature_log_1b");
    }

    #[actix_rt::test]
    async fn temperature_log_service_sort() {
        let (mock_data, _, connection_manager, _) =
            setup_all("test_temperature_log_sort", MockDataInserts::all()).await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.basic_context().unwrap();
        let service = service_provider.temperature_log_service;
        // Test Timestamp sort with default sort order
        let result = service
            .get_temperature_logs(
                &context,
                None,
                None,
                Some(Sort {
                    key: TemperatureLogSortField::Timestamp,
                    desc: None,
                }),
            )
            .unwrap();

        let mut temperature_logs = mock_data["base"].temperature_logs.clone();
        temperature_logs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        let result_names: Vec<String> = result
            .rows
            .into_iter()
            .map(|temperature_log| temperature_log.temperature_log_row.name)
            .collect();
        let sorted_names: Vec<String> = temperature_logs.into_iter().map(|temperature_log| temperature_log.name).collect();

        assert_eq!(result_names, sorted_names);

        // Test Temperature sort with desc sort
        let result = service
            .get_temperature_logs(
                &context,
                None,
                None,
                Some(Sort {
                    key: TemperatureLogSortField::Temperature,
                    desc: Some(true),
                }),
            )
            .unwrap();

        let mut temperature_logs = mock_data["base"].temperature_logs.clone();
        temperature_logs.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));

        let result_names: Vec<String> = result
            .rows
            .into_iter()
            .map(|temperature_log| temperature_log.temperature_log_row.name)
            .collect();
        let sorted_names: Vec<String> = temperature_logs.into_iter().map(|temperature_log| temperature_log.name).collect();

        assert_eq!(result_names, sorted_names);
    }
}
