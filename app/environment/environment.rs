use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;

use crate::{EnvLock, owned_var_or, var, var_or};

pub struct Environment {
    pub hostname: IpAddr,
    pub port: u16,
    pub domain: &'static str,
    pub log_level: LevelFilter,
    pub log_directory: PathBuf,
    pub database_url: &'static str,
    pub db_conn_max: u16,
}

impl Environment {
    /// # Panics
    /// Will panic if it fails to parse the environment variables
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            hostname: owned_var_or("HOSTNAME", IpAddr::V4(Ipv4Addr::LOCALHOST)),
            port: owned_var_or("PORT", 3000),
            domain: var_or::<String, _>("DOMAIN", "localhost"),
            log_level: var_or::<String, _>("LOG_LEVEL", "INFO")
                .parse::<LevelFilter>()
                .unwrap(),
            log_directory: owned_var_or(
                "LOG_DIRECTORY",
                PathBuf::from("./logs"),
            ),
            database_url: var::<String, _>("DATABASE_URL"),
            db_conn_max: owned_var_or("DB_CONN_MAX", 10),
        }
    }
}

pub static ENV: EnvLock = EnvLock::new();
