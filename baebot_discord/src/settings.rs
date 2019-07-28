use baebot_models::{AppConfig, AuthConfig};
use config;

pub fn build_settings() -> AppConfig {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::new("config/Settings", config::FileFormat::Toml)).unwrap();

    settings.try_into().unwrap()
}

pub fn build_auth() -> AuthConfig {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::new("config/Auth", config::FileFormat::Toml)).unwrap();

    settings.try_into().unwrap()
}

