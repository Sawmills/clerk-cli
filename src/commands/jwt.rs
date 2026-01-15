use crate::client::ClerkClient;
use anyhow::Result;
use dialoguer::FuzzySelect;

pub async fn run(user_id: Option<String>, template: Option<String>) -> Result<()> {
    let client = ClerkClient::new()?;

    let user_id = match user_id {
        Some(id) => id,
        None => select_user(&client).await?,
    };

    let templates = client.list_jwt_templates().await?;
    if templates.is_empty() {
        anyhow::bail!("No JWT templates found. Create one in Clerk dashboard first.");
    }

    let template_name = match template {
        Some(t) => {
            if !templates.iter().any(|tmpl| tmpl.name == t) {
                let available: Vec<_> = templates.iter().map(|t| t.name.as_str()).collect();
                anyhow::bail!(
                    "Template '{}' not found. Available: {}",
                    t,
                    available.join(", ")
                );
            }
            t
        }
        None => select_template(&templates)?,
    };

    let session_id = get_or_create_session(&client, &user_id).await?;
    let token = client
        .create_session_token(&session_id, &template_name)
        .await?;

    println!("{}", token.jwt);

    Ok(())
}

async fn select_user(client: &ClerkClient) -> Result<String> {
    let users = client.list_users(50, None).await?;
    if users.is_empty() {
        anyhow::bail!("No users found");
    }

    let display: Vec<String> = users
        .iter()
        .map(|u| {
            let email = u.primary_email().unwrap_or("no email");
            let name = u.display_name();
            if name.is_empty() {
                format!("{} ({})", email, u.id)
            } else {
                format!("{} <{}> ({})", name, email, u.id)
            }
        })
        .collect();

    let selection = FuzzySelect::new()
        .with_prompt("Select user")
        .items(&display)
        .default(0)
        .interact()?;

    Ok(users[selection].id.clone())
}

fn select_template(templates: &[crate::models::JwtTemplate]) -> Result<String> {
    let display: Vec<String> = templates
        .iter()
        .map(|t| format!("{} ({}s lifetime)", t.name, t.lifetime))
        .collect();

    let selection = FuzzySelect::new()
        .with_prompt("Select JWT template")
        .items(&display)
        .default(0)
        .interact()?;

    Ok(templates[selection].name.clone())
}

async fn get_or_create_session(client: &ClerkClient, user_id: &str) -> Result<String> {
    let sessions = client.list_sessions(user_id, Some("active")).await?;

    if let Some(session) = sessions.first() {
        return Ok(session.id.clone());
    }

    eprintln!("No active session found, creating one...");

    let sign_in_token = client.create_sign_in_token(user_id, 300).await?;

    let ticket = sign_in_token
        .url
        .split("__clerk_ticket=")
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Failed to extract ticket from sign-in URL"))?;

    let session_id = client.exchange_ticket_for_session(ticket).await?;

    Ok(session_id)
}

pub async fn run_list_templates() -> Result<()> {
    let client = ClerkClient::new()?;
    let templates = client.list_jwt_templates().await?;

    if templates.is_empty() {
        println!("No JWT templates found.");
        return Ok(());
    }

    for t in templates {
        let lifetime_str = if t.lifetime >= 86400 {
            format!("{}d", t.lifetime / 86400)
        } else if t.lifetime >= 3600 {
            format!("{}h", t.lifetime / 3600)
        } else if t.lifetime >= 60 {
            format!("{}m", t.lifetime / 60)
        } else {
            format!("{}s", t.lifetime)
        };
        println!("{} ({})", t.name, lifetime_str);
    }

    Ok(())
}
