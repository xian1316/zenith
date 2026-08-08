use std::env;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("DB_HOST").expect("DB_HOST must be set"),
            port: env::var("DB_PORT")
                .expect("DB_PORT must be set")
                .parse()
                .expect("DB_PORT must be a number"),
            username: env::var("DB_USER").expect("DB_USER must be set"),
            password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
            database: env::var("DB_NAME").expect("DB_NAME must be set"),
        }
    }
}