use crate::client::ClerkClient;
use crate::commands::orgs::pick_org;
use dialoguer::FuzzySelect;

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
    let org = pick_org(client).await?;

    let memberships = client.list_org_memberships(&org.id, 100).await?;

    if memberships.is_empty() {
        anyhow::bail!("No users found in organization '{}'.", org.name);
    }

    let display: Vec<String> = memberships
        .iter()
        .map(|m| {
            let name = m.public_user_data.display_name();
            let email = m.public_user_data.identifier.as_deref().unwrap_or("no email");
            let role = &m.role;
            if name.is_empty() {
                format!("{} [{}]", email, role)
            } else {
                format!("{} <{}> [{}]", name, email, role)
            }
        })
        .collect();

    let selection = FuzzySelect::new()
        .with_prompt(format!("Select user from '{}'", org.name))
        .items(&display)
        .default(0)
        .interact()?;

    Ok(memberships[selection].public_user_data.user_id.clone())
}
