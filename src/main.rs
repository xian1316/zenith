mod db;

use anyhow::Result;
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();
    println!("🚀 Setting up Zenith database...");

    // Call the synchronous setup function
    db::setup()?;

    println!("✅ Database setup complete!");
    Ok(())
}