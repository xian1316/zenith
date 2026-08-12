use postgres::Client;
use anyhow::Result;

pub fn run_migrations(client: &mut Client) -> Result<()> {
    println!("📋 Creating tables...");

    // Table 1: members
    client.batch_execute(
        r#"
        CREATE TABLE IF NOT EXISTS members (
            member_id BIGSERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            capital DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
            date_joined DATE NOT NULL DEFAULT CURRENT_DATE
        )
        "#,
    )?;
    println!("  ✅ members");

    // Table 2: config
    client.batch_execute(
        r#"
        CREATE TABLE IF NOT EXISTS config (
            config_id BIGSERIAL PRIMARY KEY,
            variable_name VARCHAR(255) NOT NULL UNIQUE,
            variable_value TEXT NOT NULL
        )
        "#,
    )?;
    println!("  ✅ config");

    // Table 3: transactions
    client.batch_execute(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            transaction_id BIGSERIAL PRIMARY KEY,
            member_id BIGINT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
            transaction_date DATE NOT NULL DEFAULT CURRENT_DATE,
            transaction_name VARCHAR(255) NOT NULL,
            follow_amount DECIMAL(15, 2) NOT NULL DEFAULT 0.00,
            rate_of_return DECIMAL(5, 2) DEFAULT 0.00
        )
        "#,
    )?;
    println!("  ✅ transactions");

    // Add indexes
    client.batch_execute(
        "CREATE INDEX IF NOT EXISTS idx_transactions_member_id ON transactions(member_id)"
    )?;
    
    client.batch_execute(
        "CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(transaction_date)"
    )?;
    println!("  ✅ indexes");

    Ok(())
}