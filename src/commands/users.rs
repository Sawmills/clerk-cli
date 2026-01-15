use crate::client::ClerkClient;
use comfy_table::{presets::UTF8_FULL, Table};

pub async fn run(limit: u32, query: Option<String>) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;
    let users = client.list_users(limit, query.as_deref()).await?;

    if users.is_empty() {
        println!("No users found.");
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Email", "Name", "Last Sign In"]);

    for user in &users {
        let email = user.primary_email().unwrap_or("N/A");
        let name = user.display_name();
        let last_sign_in = user
            .last_sign_in_at
            .map(|ts| {
                chrono::DateTime::from_timestamp(ts / 1000, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "Invalid".to_string())
            })
            .unwrap_or_else(|| "Never".to_string());

        table.add_row(vec![&user.id, email, &name, &last_sign_in]);
    }

    println!("{table}");
    println!("Showing {} users.", users.len());

    Ok(())
}
