use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub persistence: PersistenceConfig
}

#[derive(Deserialize, Clone)]
pub struct PersistenceConfig {
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
    pub schema_collection: String,
    pub auth_db: String
}
