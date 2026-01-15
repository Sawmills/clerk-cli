//! E2E tests against live Clerk API
//! 
//! Run with: CLERK_API_KEY=sk_test_xxx cargo test --test e2e_test -- --ignored
//! 
//! These tests require a valid Clerk API key and will make real API calls.

mod common;

fn get_api_key() -> Option<String> {
    std::env::var("CLERK_API_KEY").ok()
}

fn make_live_client() -> Option<common::TestClerkClient> {
    get_api_key().map(|key| common::make_test_client("https://api.clerk.com", &key))
}

#[tokio::test]
#[ignore]
async fn e2e_list_users() {
    let client = make_live_client().expect("CLERK_API_KEY required for e2e tests");

    let users = client.list_users(5, None).await;
    assert!(users.is_ok(), "Failed to list users: {:?}", users.err());

    let users = users.unwrap();
    println!("Fetched {} users from live API", users.len());

    for user in &users {
        println!("  - {} ({})", user.id, user.primary_email().unwrap_or("no email"));
    }
}

#[tokio::test]
#[ignore]
async fn e2e_list_users_with_query() {
    let client = make_live_client().expect("CLERK_API_KEY required for e2e tests");

    let users = client.list_users(10, Some("@")).await;
    assert!(users.is_ok(), "Failed to search users: {:?}", users.err());

    println!("Search returned {} users", users.unwrap().len());
}

#[tokio::test]
#[ignore]
async fn e2e_list_organizations() {
    let client = make_live_client().expect("CLERK_API_KEY required for e2e tests");

    let orgs = client.list_organizations(10).await;
    assert!(orgs.is_ok(), "Failed to list organizations: {:?}", orgs.err());

    let orgs = orgs.unwrap();
    println!("Fetched {} organizations from live API", orgs.len());

    for org in &orgs {
        println!("  - {} ({}) - {} members", org.id, org.name, org.members_count.unwrap_or(0));
    }
}

#[tokio::test]
#[ignore]
async fn e2e_create_sign_in_token() {
    let client = make_live_client().expect("CLERK_API_KEY required for e2e tests");

    let users = client.list_users(1, None).await.expect("Failed to list users");

    if users.is_empty() {
        println!("No users available to test impersonation");
        return;
    }

    let user_id = &users[0].id;
    println!("Creating sign-in token for user: {}", user_id);

    let token = client.create_sign_in_token(user_id, 60).await;
    assert!(token.is_ok(), "Failed to create sign-in token: {:?}", token.err());

    let token = token.unwrap();
    println!("Sign-in URL: {}", token.url);
    assert!(!token.url.is_empty());
}

#[tokio::test]
#[ignore]
async fn e2e_invalid_api_key() {
    let client = common::make_test_client("https://api.clerk.com", "sk_test_invalid_key");

    let result = client.list_users(1, None).await;
    assert!(result.is_err(), "Expected error with invalid API key");
    println!("Got expected error: {}", result.unwrap_err());
}

#[tokio::test]
#[ignore]
async fn e2e_sign_in_token_invalid_user() {
    let client = make_live_client().expect("CLERK_API_KEY required for e2e tests");

    let result = client.create_sign_in_token("user_nonexistent_12345", 60).await;
    assert!(result.is_err(), "Expected error for non-existent user");
    println!("Got expected error: {}", result.unwrap_err());
}
