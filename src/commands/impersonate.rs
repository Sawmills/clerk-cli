use crate::client::ClerkClient;
use dialoguer::{theme::ColorfulTheme, Select};

pub async fn run(user_id: Option<String>) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let target_user_id = match user_id {
        Some(id) => id,
        None => prompt_user_selection(&client).await?,
    };

    let token = client.create_sign_in_token(&target_user_id, 3600).await?;

    println!("\n\x1b[32mSign-in token generated successfully!\x1b[0m");
    println!("Use this URL to sign in as the user:\n");
    println!("{}\n", token.url);
    println!("(Link expires in 1 hour)");

    Ok(())
}

async fn prompt_user_selection(client: &ClerkClient) -> anyhow::Result<String> {
    let users = client.list_users(10, None).await?;

    if users.is_empty() {
        anyhow::bail!("No users found to impersonate.");
    }

    let items: Vec<String> = users
        .iter()
        .map(|u| {
            let email = u.primary_email().unwrap_or("No Email");
            let name = u.display_name();
            if name.is_empty() {
                format!("{} - {}", email, u.id)
            } else {
                format!("{} ({}) - {}", name, email, u.id)
            }
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a user to impersonate")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(users[selection].id.clone())
}
