use crate::client::ClerkClient;

pub async fn complete_orgs() -> anyhow::Result<()> {
    let Ok(client) = ClerkClient::new() else {
        return Ok(());
    };

    let Ok(orgs) = client.list_organizations(100).await else {
        return Ok(());
    };

    for org in orgs {
        if let Some(slug) = &org.slug {
            println!("{}:{}", slug, org.name);
        }
    }

    Ok(())
}

pub async fn complete_users(org_slug: Option<String>) -> anyhow::Result<()> {
    let Ok(client) = ClerkClient::new() else {
        return Ok(());
    };

    let Some(slug) = org_slug else {
        return Ok(());
    };

    let Ok(orgs) = client.list_organizations(100).await else {
        return Ok(());
    };

    let Some(org) = orgs.into_iter().find(|o| o.slug.as_deref() == Some(&slug)) else {
        return Ok(());
    };

    let Ok(memberships) = client.list_org_memberships(&org.id, 100).await else {
        return Ok(());
    };

    for m in memberships {
        let name = m.public_user_data.display_name();
        let email = m.public_user_data.identifier.as_deref().unwrap_or("");
        let desc = if name.is_empty() {
            email.to_string()
        } else {
            format!("{} <{}>", name, email)
        };
        println!("{}:{}", m.public_user_data.user_id, desc);
    }

    Ok(())
}
