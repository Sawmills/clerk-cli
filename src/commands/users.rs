use crate::client::ClerkClient;
use crate::models::{CreateOrgMembershipRequest, CreateUserRequest};
use comfy_table::{Table, presets::UTF8_FULL};

pub async fn list(limit: u32, query: Option<String>) -> anyhow::Result<()> {
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

pub async fn create(
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    password: Option<String>,
) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let request = CreateUserRequest {
        email_address: Some(vec![email]),
        first_name,
        last_name,
        password: password.clone(),
        skip_password_requirement: if password.is_none() { Some(true) } else { None },
    };

    let user = client.create_user(request).await?;

    println!("Created user: {}", user.id);
    if let Some(email) = user.primary_email() {
        println!("Email: {}", email);
    }
    if !user.display_name().is_empty() {
        println!("Name: {}", user.display_name());
    }

    Ok(())
}

pub async fn show(user_id: &str) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;
    let user = client.get_user(user_id).await?;

    println!("User: {}", user.id);
    if let Some(email) = user.primary_email() {
        println!("Email: {}", email);
    }
    println!("Name: {}", user.display_name());
    if let Some(ts) = user.last_sign_in_at {
        if let Some(dt) = chrono::DateTime::from_timestamp(ts / 1000, 0) {
            println!("Last Sign In: {}", dt.format("%Y-%m-%d %H:%M"));
        }
    }
    if let Some(dt) = chrono::DateTime::from_timestamp(user.created_at / 1000, 0) {
        println!("Created: {}", dt.format("%Y-%m-%d %H:%M"));
    }

    Ok(())
}

pub async fn add_to_org(user_id: &str, org_slug: &str, role: &str) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let orgs = client.list_organizations(100).await?;
    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(org_slug) || o.id == org_slug)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", org_slug))?;

    let request = CreateOrgMembershipRequest {
        user_id: user_id.to_string(),
        role: role.to_string(),
    };

    let membership = client.create_org_membership(&org.id, request).await?;
    println!("Added user {} to '{}' with role {}", user_id, org.name, membership.role);

    Ok(())
}

pub async fn remove_from_org(user_id: &str, org_slug: &str) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let orgs = client.list_organizations(100).await?;
    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(org_slug) || o.id == org_slug)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", org_slug))?;

    client.delete_org_membership(&org.id, user_id).await?;
    println!("Removed user {} from '{}'", user_id, org.name);

    Ok(())
}
