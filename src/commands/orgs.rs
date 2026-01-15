use crate::client::ClerkClient;
use comfy_table::{Table, presets::UTF8_FULL};
use dialoguer::FuzzySelect;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

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
    table.set_header(vec!["ID", "Name", "Slug", "Members"]);

    for org in &filtered {
        let members = org
            .members_count
            .map(|c| c.to_string())
            .unwrap_or_else(|| "?".to_string());
        table.add_row(vec![
            &org.id,
            &org.name,
            org.slug.as_deref().unwrap_or(""),
            &members,
        ]);
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

    let display: Vec<String> = orgs
        .iter()
        .map(|o| {
            let members = o.members_count.map(|c| c.to_string()).unwrap_or_default();
            format!(
                "{} ({}) - {} members",
                o.name,
                o.slug.as_deref().unwrap_or("no-slug"),
                members
            )
        })
        .collect();

    let selection = FuzzySelect::new()
        .with_prompt("Select organization")
        .items(&display)
        .default(0)
        .interact()?;

    println!("{}", orgs[selection].id);

    Ok(())
}

pub async fn pick_org(client: &ClerkClient) -> anyhow::Result<crate::models::Organization> {
    let orgs = client.list_organizations(100).await?;

    if orgs.is_empty() {
        anyhow::bail!("No organizations found.");
    }

    let display: Vec<String> = orgs
        .iter()
        .map(|o| {
            let members = o.members_count.map(|c| c.to_string()).unwrap_or_default();
            format!(
                "{} ({}) - {} members",
                o.name,
                o.slug.as_deref().unwrap_or("no-slug"),
                members
            )
        })
        .collect();

    let selection = FuzzySelect::new()
        .with_prompt("Select organization")
        .items(&display)
        .default(0)
        .interact()?;

    Ok(orgs.into_iter().nth(selection).unwrap())
}
