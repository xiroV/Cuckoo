pub mod config;
pub mod inbox;

use self::config::Config;

pub struct ServiceBundle {
    pub config: Config
}