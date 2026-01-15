use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

struct TestOrg {
    id: String,
    name: String,
    slug: Option<String>,
}

fn fuzzy_filter(orgs: Vec<TestOrg>, pattern: &str) -> Vec<TestOrg> {
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
}

fn make_org(id: &str, name: &str, slug: Option<&str>) -> TestOrg {
    TestOrg {
        id: id.to_string(),
        name: name.to_string(),
        slug: slug.map(String::from),
    }
}

#[test]
fn fuzzy_exact_match() {
    let orgs = vec![
        make_org("1", "Acme Corp", Some("acme")),
        make_org("2", "Beta Inc", Some("beta")),
    ];
    let results = fuzzy_filter(orgs, "Acme");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Acme Corp");
}

#[test]
fn fuzzy_partial_match() {
    let orgs = vec![
        make_org("1", "Acme Corp", Some("acme")),
        make_org("2", "Beta Inc", Some("beta")),
        make_org("3", "Acme Labs", Some("acme-labs")),
    ];
    let results = fuzzy_filter(orgs, "acm");
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|o| o.name.contains("Acme")));
}

#[test]
fn fuzzy_slug_match() {
    let orgs = vec![
        make_org("1", "Some Company", Some("acme-corp")),
        make_org("2", "Beta Inc", Some("beta")),
    ];
    let results = fuzzy_filter(orgs, "acme");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "1");
}

#[test]
fn fuzzy_no_match() {
    let orgs = vec![
        make_org("1", "Acme Corp", Some("acme")),
        make_org("2", "Beta Inc", Some("beta")),
    ];
    let results = fuzzy_filter(orgs, "zzzznotfound");
    assert!(results.is_empty());
}

#[test]
fn fuzzy_typo_tolerance() {
    let orgs = vec![
        make_org("1", "Acme Corporation", Some("acme")),
        make_org("2", "Beta Inc", Some("beta")),
    ];
    let results = fuzzy_filter(orgs, "acm corp");
    assert!(!results.is_empty());
    assert_eq!(results[0].name, "Acme Corporation");
}

#[test]
fn fuzzy_ranking() {
    let orgs = vec![
        make_org("1", "Acme Labs", Some("acme-labs")),
        make_org("2", "Acme", Some("acme")),
        make_org("3", "Acme Corporation International", Some("acme-intl")),
    ];
    let results = fuzzy_filter(orgs, "Acme");
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|o| o.name.contains("Acme")));
}
