use config::{Config, ConfigError, File};
use std::env;

pub fn config_create() -> Result<Config, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "config".to_string());

    let s = Config::builder()
        .add_source(File::with_name("src/rust_config/config/config.json"))
       
        .add_source(
            File::with_name(&format!("src/rust_config/config/{}", run_mode))
                .required(false),
        ).build();


    s
}