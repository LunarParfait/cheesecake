use std::net::{IpAddr, Ipv4Addr};

use tracing::Level;

use crate::{owned_var_or, var, var_or, EnvLock};

pub struct Environment {
    pub hostname: IpAddr,
    pub port: u16,
    pub domain: &'static str,
    pub log_severity: tracing::Level,
    pub log_directory: &'static str,
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
            log_severity: match var_or::<String, _>("LOG_LEVEL", "INFO") {
                "TRACE" => Level::TRACE,
                "DEBUG" => Level::DEBUG,
                "INFO" => Level::INFO,
                "WARN" => Level::WARN,
                "ERROR" => Level::ERROR,
                _ => panic!("Invalid LOG_LEVEL"),
            },
            log_directory: var_or::<String, _>("LOG_DIRECTORY", "logs/"),
            database_url: var::<String, _>("DATABASE_URL"),
            db_conn_max: owned_var_or("DB_CONN_MAX", 10),
        }
    }
}

pub static ENV: EnvLock = EnvLock::new();
