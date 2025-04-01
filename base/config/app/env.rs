use crate::cheesecake::env_utils::{AppEnvironmentTrait, EnvLock};

pub struct AppEnv {}

pub static ENV: EnvLock<AppEnv> = EnvLock::new();

impl AppEnvironmentTrait for AppEnv {
    fn new() -> Self {
        Self {}
    }
}
