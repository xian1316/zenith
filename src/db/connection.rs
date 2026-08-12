use postgres::{Client, NoTls};
use anyhow::Result;

use super::DbConfig;

pub fn create_database_if_not_exists(config: &DbConfig) -> Result<()> {
    // Connect to default 'postgres' database
    let connection_string = format!(
        "host={} port={} user={} password={} dbname=postgres",
        config.host, config.port, config.username, config.password
    );
    
    let mut admin_client = Client::connect(&connection_string, NoTls)?;

    // Check if our database exists
    let db_exists: bool = admin_client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)",
            &[&config.database],
        )?
        .get(0);

    // Create it if it doesn't exist
    if !db_exists {
        println!("📦 Creating database: {}", config.database);
        admin_client.batch_execute(&format!("CREATE DATABASE {}", config.database))?;
        println!("✅ Database created!");
    } else {
        println!("ℹ️ Database '{}' already exists", config.database);
    }

    Ok(())
}

pub fn get_client(config: &DbConfig) -> Result<Client> {
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        config.host, config.port, config.username, config.password, config.database
    );
    
    let client = Client::connect(&connection_string, NoTls)?;
    println!("✅ Connected to database: {}", config.database);
    Ok(client)
}