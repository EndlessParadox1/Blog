use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct RedisConfig {
    pub dsn: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub pg: deadpool_postgres::Config,
    pub redis: RedisConfig,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;
        cfg.try_deserialize()
    }
}
