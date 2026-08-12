mod db;
mod models;

use anyhow::Result;
use dotenv::dotenv;
use tokio::runtime;  // ← Add this import

fn main() -> Result<()> {  // ← Remove 'async' and '#[tokio::main]'
    dotenv().ok();
    println!("🚀 Setting up Zenith database...");

    // Create a runtime and block on the async function
    let rt = runtime::Runtime::new()?;
    rt.block_on(async {
        db::setup().await?;
        Ok::<_, anyhow::Error>(())
    })?;

    println!("✅ Database setup complete!");
    Ok(())
}