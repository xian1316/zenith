use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};
use anyhow::Result;

use super::DbConfig;

pub async fn create_database_if_not_exists(config: &DbConfig) -> Result<()> {
    // Connect to default 'postgres' database to create our database
    let admin_options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password)
        .database("postgres");
    
    let admin_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(admin_options)
        .await?;
    
    // Check if our database exists
    let db_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)"
    )
    .bind(&config.database)
    .fetch_one(&admin_pool)
    .await?;
    
    // Create it if it doesn't exist
    if !db_exists {
        println!("📦 Creating database: {}", config.database);
        sqlx::query(&format!("CREATE DATABASE {}", config.database))
            .execute(&admin_pool)
            .await?;
        println!("✅ Database created!");
    } else {
        println!("ℹ️ Database '{}' already exists", config.database);
    }
    
    admin_pool.close().await;
    Ok(())
}

pub async fn get_pool(config: &DbConfig) -> Result<Pool<Postgres>> {
    let options = PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password)
        .database(&config.database);
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?;
    
    println!("✅ Connected to database: {}", config.database);
    Ok(pool)
}