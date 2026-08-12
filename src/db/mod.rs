mod config;
mod connection;
mod migrations;

pub use config::DbConfig;
pub use connection::{create_database_if_not_exists, get_client};
pub use migrations::run_migrations;

use anyhow::Result;

pub fn setup() -> Result<()> {
    let config = DbConfig::from_env();

    // 1. Create database if it doesn't exist
    create_database_if_not_exists(&config)?;

    // 2. Connect to the database
    let mut client = get_client(&config)?;

    // 3. Create tables
    run_migrations(&mut client)?;

    Ok(())
}