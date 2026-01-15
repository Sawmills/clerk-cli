use crate::client::ClerkClient;
use crate::commands::impersonate;
use comfy_table::{Table, presets::UTF8_FULL};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use nucleo_picker::{Picker, render::StrRenderer};

pub enum MemberAction {
    Impersonate,
}

pub async fn run(limit: u32, fuzzy: Option<String>, ids_only: bool) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;
    let orgs = client.list_organizations(limit).await?;

    let filtered: Vec<_> = if let Some(ref pattern) = fuzzy {
        let matcher = SkimMatcherV2::default();
        let mut scored: Vec<_> = orgs
            .into_iter()
            .filter_map(|org| {
                let name_score = matcher.fuzzy_match(&org.name, pattern);
                let slug_score = org
                    .slug
                    .as_ref()
                    .and_then(|s| matcher.fuzzy_match(s, pattern));
                let best = name_score.max(slug_score);
                best.map(|score| (org, score))
            })
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(org, _)| org).collect()
    } else {
        orgs
    };

    if filtered.is_empty() {
        if !ids_only {
            println!("No organizations found.");
        }
        return Ok(());
    }

    if ids_only {
        for org in &filtered {
            println!("{}", org.id);
        }
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Name", "Slug"]);

    for org in &filtered {
        table.add_row(vec![&org.id, &org.name, org.slug.as_deref().unwrap_or("")]);
    }

    println!("{table}");

    if let Some(ref pattern) = fuzzy {
        println!(
            "Found {} matches for \"{}\" (searched {} orgs)",
            filtered.len(),
            pattern,
            limit
        );
    } else {
        println!("Showing {} organizations.", filtered.len());
    }

    Ok(())
}

pub async fn pick() -> anyhow::Result<()> {
    let client = ClerkClient::new()?;
    let orgs = client.list_organizations(100).await?;

    if orgs.is_empty() {
        anyhow::bail!("No organizations found.");
    }

    let mut picker = Picker::new(StrRenderer);
    let injector = picker.injector();

    for org in &orgs {
        let display = format!(
            "{} ({})",
            org.name,
            org.slug.as_deref().unwrap_or("no-slug")
        );
        injector.push(display);
    }

    let selected = picker.pick()?;
    if let Some(display) = selected {
        for org in &orgs {
            let org_display = format!(
                "{} ({})",
                org.name,
                org.slug.as_deref().unwrap_or("no-slug")
            );
            if org_display == *display {
                println!("{}", org.id);
                return Ok(());
            }
        }
    }

    anyhow::bail!("No organization selected.");
}

pub async fn pick_org(client: &ClerkClient) -> anyhow::Result<crate::models::Organization> {
    let orgs = client.list_organizations(100).await?;

    if orgs.is_empty() {
        anyhow::bail!("No organizations found.");
    }

    let mut picker = Picker::new(StrRenderer);
    let injector = picker.injector();

    for org in &orgs {
        let display = format!(
            "{} ({})",
            org.name,
            org.slug.as_deref().unwrap_or("no-slug")
        );
        injector.push(display);
    }

    let selected = picker.pick()?;
    if let Some(display) = selected {
        for org in orgs {
            let org_display = format!(
                "{} ({})",
                org.name,
                org.slug.as_deref().unwrap_or("no-slug")
            );
            if org_display == *display {
                return Ok(org);
            }
        }
    }

    anyhow::bail!("No organization selected.");
}

pub async fn members(
    org_slug: &str,
    user_id: Option<String>,
    action: Option<MemberAction>,
) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let orgs = client.list_organizations(100).await?;
    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(org_slug) || o.id == org_slug)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", org_slug))?;

    match (user_id, action) {
        (Some(uid), Some(MemberAction::Impersonate)) => {
            impersonate::run(Some(uid)).await?;
        }
        (Some(uid), None) => {
            anyhow::bail!(
                "Action required for user. Usage: clerk orgs {} members {} impersonate",
                org_slug,
                uid
            );
        }
        (None, _) => {
            let memberships = client.list_org_memberships(&org.id, 100).await?;

            if memberships.is_empty() {
                println!("No members found in '{}'.", org.name);
                return Ok(());
            }

            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(vec!["User ID", "Name", "Email", "Role"]);

            for m in &memberships {
                let name = m.public_user_data.display_name();
                let email = m.public_user_data.identifier.as_deref().unwrap_or("");
                table.add_row(vec![&m.public_user_data.user_id, &name, email, &m.role]);
            }

            println!("{table}");
            println!("Showing {} members of '{}'.", memberships.len(), org.name);
        }
    }

    Ok(())
}

pub async fn show(slug_or_id: &str) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;
    let orgs = client.list_organizations(100).await?;

    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(slug_or_id) || o.id == slug_or_id)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", slug_or_id))?;

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Field", "Value"]);
    table.add_row(vec!["ID", &org.id]);
    table.add_row(vec!["Name", &org.name]);
    table.add_row(vec!["Slug", org.slug.as_deref().unwrap_or("")]);

    println!("{table}");

    Ok(())
}
