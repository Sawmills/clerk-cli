use crate::client::ClerkClient;
use comfy_table::{presets::UTF8_FULL, Table};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub async fn run(limit: u32, fuzzy: Option<String>) -> anyhow::Result<()> {
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
        println!("No organizations found.");
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
