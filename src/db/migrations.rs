use sqlx::PgPool;
use anyhow::Result;

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    println!("📋 Creating tables...");
    
    // Table 1: members
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS members (
            member_id BIGSERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            capital DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
            date_joined DATE NOT NULL DEFAULT CURRENT_DATE
        )
        "#
    )
    .execute(pool)
    .await?;
    println!("  ✅ members");
    
    // Table 2: config
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS config (
            config_id BIGSERIAL PRIMARY KEY,
            variable_name VARCHAR(255) NOT NULL UNIQUE,
            variable_value TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    println!("  ✅ config");
    
    // Table 3: transactions
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            transaction_id BIGSERIAL PRIMARY KEY,
            member_id BIGINT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
            transaction_date DATE NOT NULL DEFAULT CURRENT_DATE,
            transaction_name VARCHAR(255) NOT NULL,
            follow_amount DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
            rate_of_return DECIMAL(5, 2) DEFAULT 0.00
        )
        "#
    )
    .execute(pool)
    .await?;
    println!("  ✅ transactions");
    
    // Add indexes
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_transactions_member_id ON transactions(member_id)"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(transaction_date)"
    )
    .execute(pool)
    .await?;
    println!("  ✅ indexes");
    
    Ok(())
}