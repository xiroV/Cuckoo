use config::{ConfigError, ConfigErrorType, ConfigErrorHandler};

impl ConfigErrorHandler for ConfigError {
    fn missing_field(field: &str) -> Self {
        ConfigError {
            err_type: ConfigErrorType::MissingField,
            field: Some(String::from(field)),
        }
    }

    fn file_not_found() -> Self {
        ConfigError {
            err_type: ConfigErrorType::FileNotFound,
            field: None,
        }
    }
}
