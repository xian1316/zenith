mod db;
mod models;

use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("🚀 Setting up Zenith database...");
    
    db::setup().await?;
    
    println!("✅ Database setup complete!");
    Ok(())
}