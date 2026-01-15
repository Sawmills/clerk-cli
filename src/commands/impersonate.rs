use crate::client::ClerkClient;
use crate::commands::orgs::pick_org;
use nucleo_picker::{Picker, render::StrRenderer};

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

    let mut picker = Picker::new(StrRenderer);
    let injector = picker.injector();

    for m in &memberships {
        let name = m.public_user_data.display_name();
        let email = m
            .public_user_data
            .identifier
            .as_deref()
            .unwrap_or("no email");
        let role = &m.role;
        let display = if name.is_empty() {
            format!("{} [{}]", email, role)
        } else {
            format!("{} <{}> [{}]", name, email, role)
        };
        injector.push(display);
    }

    let selected = picker.pick()?;
    if let Some(display) = selected {
        for m in &memberships {
            let name = m.public_user_data.display_name();
            let email = m
                .public_user_data
                .identifier
                .as_deref()
                .unwrap_or("no email");
            let role = &m.role;
            let m_display = if name.is_empty() {
                format!("{} [{}]", email, role)
            } else {
                format!("{} <{}> [{}]", name, email, role)
            };
            if m_display == *display {
                return Ok(m.public_user_data.user_id.clone());
            }
        }
    }

    anyhow::bail!("No user selected.");
}
