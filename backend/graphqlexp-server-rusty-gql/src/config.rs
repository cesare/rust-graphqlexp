use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use graphqlexp_app::modules::RepositoriesModuleConfig;

#[derive(Clone, Deserialize)]
pub struct GraphqlexpConfig {
    pub database: DatabaseConfig,
    pub schema: SchemaConfig,
    pub server: ServerConfig,
}

impl GraphqlexpConfig {
    pub async fn load(path: &PathBuf) -> Result<Self> {
        let mut file = File::open(path).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Clone, Deserialize)]
pub struct ServerConfig {
    bind: String,
    port: u32,
}

impl ServerConfig {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: u32,
    database: String,
    username: String,
    password: String,
}

impl RepositoriesModuleConfig for DatabaseConfig {
    fn database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Clone, Deserialize)]
pub struct SchemaConfig {
    pub config_path: String,
}
