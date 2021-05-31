use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct RedditSettings {
    pub user_agent: String,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String
}

impl RedditSettings {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(build_config()?
            .get("reddit")?)
    }
}

#[derive(Debug, Deserialize)]
pub struct CoomSettings {
    pub min_affinity: f64
}

impl CoomSettings {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(build_config()?
            .get("coom")?)
    }
}

#[derive(Debug, Deserialize)]
pub struct StartupSettings {
    pub token: String
}

impl StartupSettings {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(build_config()?.try_into()?)
    }
}

fn build_config() -> Result<Config, ConfigError> {
    let mut config = Config::new();

    config.merge(File::with_name("Settings").required(false))?
        .merge(Environment::with_prefix("baebot"))?;

    Ok(config)
}
