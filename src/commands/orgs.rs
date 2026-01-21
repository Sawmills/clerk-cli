use crate::client::ClerkClient;
use crate::commands::{impersonate, jwt};
use crate::models::{
    CreateOrgMembershipRequest, CreateOrganizationRequest, CreateSamlConnectionRequest,
    UpdateSamlConnectionRequest,
};
use comfy_table::{Table, presets::UTF8_FULL};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use nucleo_picker::{Picker, render::StrRenderer};

pub enum MemberAction {
    Impersonate,
    Jwt(Option<String>),
    Add { user_id: String, role: String },
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
        (Some(uid), Some(MemberAction::Jwt(template))) => {
            jwt::run(Some(uid), template).await?;
        }
        (_, Some(MemberAction::Add { user_id, role })) => {
            let request = CreateOrgMembershipRequest {
                user_id: user_id.clone(),
                role,
            };
            let membership = client.create_org_membership(&org.id, request).await?;
            println!(
                "Added user {} to '{}' as {}",
                user_id, org.name, membership.role
            );
        }
        (Some(uid), None) => {
            anyhow::bail!(
                "Action required for user. Usage: clerk orgs {} members {} impersonate|jwt|add",
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

pub async fn create(name: String, slug: Option<String>) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let request = CreateOrganizationRequest { name, slug };

    let org = client.create_organization(request).await?;

    println!("Created organization: {}", org.id);
    println!("Name: {}", org.name);
    if let Some(slug) = &org.slug {
        println!("Slug: {}", slug);
    }

    Ok(())
}

pub async fn delete(slug_or_id: &str, force: bool) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;
    let orgs = client.list_organizations(100).await?;

    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(slug_or_id) || o.id == slug_or_id)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", slug_or_id))?;

    if !force {
        println!(
            "Are you sure you want to delete organization '{}' ({})? [y/N]",
            org.name,
            org.slug.as_deref().unwrap_or(&org.id)
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    client.delete_organization(&org.id).await?;
    println!("Deleted organization: {} ({})", org.name, org.id);

    Ok(())
}

pub struct CreateSamlArgs {
    pub name: String,
    pub provider: String,
    pub domain: String,
    pub entity_id: Option<String>,
    pub sso_url: Option<String>,
    pub certificate: Option<String>,
    pub metadata_url: Option<String>,
}

pub async fn add_sso(org_slug: &str, args: CreateSamlArgs) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let orgs = client.list_organizations(100).await?;
    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(org_slug) || o.id == org_slug)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", org_slug))?;

    let request = CreateSamlConnectionRequest {
        name: args.name,
        provider: args.provider,
        domain: args.domain,
        organization_id: Some(org.id),
        idp_entity_id: args.entity_id,
        idp_sso_url: args.sso_url,
        idp_certificate: args.certificate,
        idp_metadata_url: args.metadata_url,
    };

    let conn = client.create_saml_connection(request).await?;

    println!("Created SAML Connection: {}", conn.id);
    println!("Name: {}", conn.name);
    println!("Provider: {}", conn.provider);
    println!("Domain: {}", conn.domain);
    println!("ACS URL: {}", conn.acs_url);
    println!("SP Entity ID: {}", conn.sp_entity_id);
    println!("SP Metadata URL: {}", conn.sp_metadata_url);

    Ok(())
}

pub struct UpdateSamlArgs {
    pub name: Option<String>,
    pub provider: Option<String>,
    pub domain: Option<String>,
    pub active: Option<bool>,
    pub entity_id: Option<String>,
    pub sso_url: Option<String>,
    pub certificate: Option<String>,
    pub metadata_url: Option<String>,
}

pub async fn list_sso(org_slug: &str) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let orgs = client.list_organizations(100).await?;
    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(org_slug) || o.id == org_slug)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", org_slug))?;

    let connections = client.list_saml_connections(Some(&org.id)).await?;

    if connections.is_empty() {
        println!("No SSO connections found for '{}'.", org.name);
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Name", "Provider", "Domain", "Active"]);

    for conn in &connections {
        let active = if conn.active { "Yes" } else { "No" };
        table.add_row(vec![
            conn.id.as_str(),
            conn.name.as_str(),
            conn.provider.as_str(),
            conn.domain.as_str(),
            active,
        ]);
    }

    println!("{table}");
    println!(
        "Showing {} SSO connection(s) for '{}'.",
        connections.len(),
        org.name
    );

    Ok(())
}

pub async fn list_all_sso() -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let connections = client.list_saml_connections(None).await?;

    if connections.is_empty() {
        println!("No SSO connections found.");
        return Ok(());
    }

    let orgs = client.list_organizations(100).await?;
    let org_map: std::collections::HashMap<_, _> =
        orgs.into_iter().map(|o| (o.id.clone(), o)).collect();

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        "ID",
        "Name",
        "Organization",
        "Provider",
        "Domain",
        "Active",
    ]);

    for conn in &connections {
        let active = if conn.active { "Yes" } else { "No" };
        let org_name = conn
            .organization_id
            .as_ref()
            .and_then(|id| org_map.get(id))
            .map(|o| o.slug.as_deref().unwrap_or(&o.name))
            .unwrap_or("-");
        table.add_row(vec![
            conn.id.as_str(),
            conn.name.as_str(),
            org_name,
            conn.provider.as_str(),
            conn.domain.as_str(),
            active,
        ]);
    }

    println!("{table}");
    println!("Showing {} SSO connection(s).", connections.len());

    Ok(())
}

pub async fn update_sso(
    org_slug: &str,
    name_or_id: &str,
    args: UpdateSamlArgs,
) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let orgs = client.list_organizations(100).await?;
    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(org_slug) || o.id == org_slug)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", org_slug))?;

    let connections = client.list_saml_connections(Some(&org.id)).await?;
    let conn = connections
        .into_iter()
        .find(|c| c.id == name_or_id || c.name == name_or_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "SSO connection '{}' not found in organization '{}'",
                name_or_id,
                org.name
            )
        })?;

    let request = UpdateSamlConnectionRequest {
        name: args.name,
        provider: args.provider,
        domain: args.domain,
        active: args.active,
        idp_entity_id: args.entity_id,
        idp_sso_url: args.sso_url,
        idp_certificate: args.certificate,
        idp_metadata_url: args.metadata_url,
        organization_id: None,
    };

    let updated = client.update_saml_connection(&conn.id, request).await?;

    println!("Updated SAML Connection: {}", updated.id);
    println!("Name: {}", updated.name);
    println!("Provider: {}", updated.provider);
    println!("Domain: {}", updated.domain);
    println!("Active: {}", updated.active);
    println!("ACS URL: {}", updated.acs_url);
    println!("SP Entity ID: {}", updated.sp_entity_id);
    println!("SP Metadata URL: {}", updated.sp_metadata_url);

    Ok(())
}

pub async fn delete_sso(org_slug: &str, name_or_id: &str, force: bool) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let orgs = client.list_organizations(100).await?;
    let org = orgs
        .into_iter()
        .find(|o| o.slug.as_deref() == Some(org_slug) || o.id == org_slug)
        .ok_or_else(|| anyhow::anyhow!("Organization '{}' not found", org_slug))?;

    let connections = client.list_saml_connections(Some(&org.id)).await?;
    let conn = connections
        .into_iter()
        .find(|c| c.id == name_or_id || c.name == name_or_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "SSO connection '{}' not found in organization '{}'",
                name_or_id,
                org.name
            )
        })?;

    if !force {
        println!(
            "Are you sure you want to delete SSO connection '{}' ({})? [y/N]",
            conn.name, conn.id
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    client.delete_saml_connection(&conn.id).await?;
    println!("Deleted SSO connection: {} ({})", conn.name, conn.id);

    Ok(())
}

pub async fn complete_sso_connections(org_slug: Option<&str>) -> anyhow::Result<()> {
    let client = ClerkClient::new()?;

    let org_id = if let Some(slug) = org_slug {
        let orgs = client.list_organizations(100).await?;
        match orgs
            .into_iter()
            .find(|o| o.slug.as_deref() == Some(slug) || o.id == slug)
        {
            Some(o) => Some(o.id),
            None => return Ok(()),
        }
    } else {
        None
    };

    let connections = client.list_saml_connections(org_id.as_deref()).await?;

    for conn in connections {
        println!("{}:{} ({})", conn.name, conn.domain, conn.provider);
    }

    Ok(())
}
