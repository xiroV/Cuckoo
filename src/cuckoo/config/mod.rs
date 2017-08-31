/*
 * The Config Interface
 *
 * To implement a new config parser the parser should implement the
 * ConfigReader for the Config structure, and to use the new interface
 * the `mod ...` line in this file should be changed to the name of
 * the new config parser module.
 */

mod configrs;

pub struct Account {
    pub id: String,
    pub name: String,
    pub mail: String,
    pub imap_server: String,
    pub imap_user: String
}

pub struct Config {
    pub accounts: Vec<Account>
}

#[derive(Debug)]
pub enum ConfigErrorType {
    FieldError { field_name: String, field_type: String },
    FileNotFound { absolute_file_path: String }
}

#[derive(Debug)]
pub struct ConfigError {
    pub etype: ConfigErrorType,
    pub human_readable_error: String
}

impl ConfigError {
    fn file_not_found(path: &str, message: &str) -> ConfigError {
        ConfigError {
            etype: ConfigErrorType::FileNotFound { absolute_file_path: path.to_string() },
            human_readable_error: message.to_string()
        }
    }

    fn field_error_with_cause(field_name: &str, field_type: &str, cause: &str) -> ConfigError {
        ConfigError {
            etype: ConfigErrorType::FieldError {
                field_name: field_name.to_string(),
                field_type: field_type.to_string()
            },
            human_readable_error:
                format!("Expected to find field '{}' of type '{}'. Cause of error is: {:?}",
                        field_name, field_type, cause)
        }
    }

    fn field_error(field_name: &str, field_type: &str) -> ConfigError {
        ConfigError {
            etype: ConfigErrorType::FieldError {
                field_name: field_name.to_string(),
                field_type: field_type.to_string()
            },
            human_readable_error: format!("Expected to find field '{}' of type '{}'",
                                          field_name, field_type)
        }
    }
}

pub trait ConfigReader {
    fn new() -> Self;
    fn read(&mut self) -> Result<(), ConfigError>;
    fn get_account(self, search_id: &String) -> Option<Account>;
}

