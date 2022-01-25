use serde::Deserialize;

#[derive(Deserialize)]
pub struct GraphqlexpConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    bind: String,
    port: u32,
}

impl ServerConfig {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: u32,
    database: String,
    username: String,
    password: String,
}
