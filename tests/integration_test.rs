use wiremock::matchers::{bearer_token, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;

#[tokio::test]
async fn list_users_success() {
    let mock_server = MockServer::start().await;

    let body = serde_json::json!([
        {
            "id": "user_abc123",
            "first_name": "John",
            "last_name": "Doe",
            "email_addresses": [
                {"id": "email_1", "email_address": "john@test.com"}
            ],
            "primary_email_address_id": "email_1",
            "last_sign_in_at": 1700000000000_i64,
            "created_at": 1699000000000_i64
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/users"))
        .and(bearer_token("sk_test_mock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .mount(&mock_server)
        .await;

    let client = common::make_test_client(&mock_server.uri(), "sk_test_mock");
    let users = client.list_users(10, None).await.unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, "user_abc123");
    assert_eq!(users[0].primary_email(), Some("john@test.com"));
    assert_eq!(users[0].display_name(), "John Doe");
}

#[tokio::test]
async fn list_users_with_query() {
    let mock_server = MockServer::start().await;

    let body = serde_json::json!([
        {
            "id": "user_xyz",
            "first_name": "Jane",
            "last_name": null,
            "email_addresses": [
                {"id": "email_1", "email_address": "jane@test.com"}
            ],
            "primary_email_address_id": null,
            "last_sign_in_at": null,
            "created_at": 1699000000000_i64
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v1/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .mount(&mock_server)
        .await;

    let client = common::make_test_client(&mock_server.uri(), "sk_test_mock");
    let users = client.list_users(10, Some("jane")).await.unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].first_name, Some("Jane".to_string()));
}

#[tokio::test]
async fn list_organizations_success() {
    let mock_server = MockServer::start().await;

    let body = serde_json::json!({
        "data": [
            {
                "id": "org_123",
                "name": "Acme Corp",
                "slug": "acme",
                "members_count": 5,
                "created_at": 1699000000000_i64
            },
            {
                "id": "org_456",
                "name": "Beta Inc",
                "slug": "beta",
                "members_count": 10,
                "created_at": 1698000000000_i64
            }
        ],
        "total_count": 2
    });

    Mock::given(method("GET"))
        .and(path("/v1/organizations"))
        .and(query_param("limit", "100"))
        .and(query_param("offset", "0"))
        .and(query_param("order_by", "-created_at"))
        .and(bearer_token("sk_test_mock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .mount(&mock_server)
        .await;

    let client = common::make_test_client(&mock_server.uri(), "sk_test_mock");
    let orgs = client.list_organizations(100).await.unwrap();

    assert_eq!(orgs.len(), 2);
    assert_eq!(orgs[0].name, "Acme Corp");
    assert_eq!(orgs[1].slug, Some("beta".to_string()));
}

#[tokio::test]
async fn list_organizations_paginates() {
    let mock_server = MockServer::start().await;

    let first_page = serde_json::json!({
        "data": [
            {
                "id": "org_123",
                "name": "Acme Corp",
                "slug": "acme",
                "members_count": 5,
                "created_at": 1699000000000_i64
            },
            {
                "id": "org_456",
                "name": "Beta Inc",
                "slug": "beta",
                "members_count": 10,
                "created_at": 1698000000000_i64
            }
        ],
        "total_count": 3
    });

    let second_page = serde_json::json!({
        "data": [
            {
                "id": "org_789",
                "name": "Gamma LLC",
                "slug": "gamma",
                "members_count": 7,
                "created_at": 1697000000000_i64
            }
        ],
        "total_count": 3
    });

    Mock::given(method("GET"))
        .and(path("/v1/organizations"))
        .and(query_param("limit", "3"))
        .and(query_param("offset", "0"))
        .and(query_param("order_by", "-created_at"))
        .and(bearer_token("sk_test_mock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&first_page))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/organizations"))
        .and(query_param("limit", "1"))
        .and(query_param("offset", "2"))
        .and(query_param("order_by", "-created_at"))
        .and(bearer_token("sk_test_mock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&second_page))
        .mount(&mock_server)
        .await;

    let client = common::make_test_client(&mock_server.uri(), "sk_test_mock");
    let orgs = client.list_organizations(3).await.unwrap();

    assert_eq!(orgs.len(), 3);
    assert_eq!(orgs[2].name, "Gamma LLC");
}

#[tokio::test]
async fn search_organizations_uses_query_and_paginates() {
    let mock_server = MockServer::start().await;

    let first_page = serde_json::json!({
        "data": [
            {
                "id": "org_31CBB0pzZLGKibkIGN2ZSAv2wZ6",
                "name": "Ziff Davis | Tech & Shopping",
                "slug": "ziff-davis-tech-shopping",
                "members_count": 4,
                "created_at": 1696000000000_i64
            },
            {
                "id": "org_39UYMpqIhuBzup1j4vEcSVq6Rzs",
                "name": "Ziff Davis | Gaming",
                "slug": "ziff-davis-gaming",
                "members_count": 6,
                "created_at": 1695000000000_i64
            }
        ],
        "total_count": 3
    });

    let second_page = serde_json::json!({
        "data": [
            {
                "id": "org_39UYHdoezFdeqZD7x5ZBx672WZY",
                "name": "Ziff Davis | Spiceworks",
                "slug": "ziff-davis-spiceworks",
                "members_count": 8,
                "created_at": 1694000000000_i64
            }
        ],
        "total_count": 3
    });

    Mock::given(method("GET"))
        .and(path("/v1/organizations"))
        .and(query_param("limit", "3"))
        .and(query_param("offset", "0"))
        .and(query_param("query", "Ziff Davis"))
        .and(query_param("order_by", "-created_at"))
        .and(bearer_token("sk_test_mock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&first_page))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/organizations"))
        .and(query_param("limit", "1"))
        .and(query_param("offset", "2"))
        .and(query_param("query", "Ziff Davis"))
        .and(query_param("order_by", "-created_at"))
        .and(bearer_token("sk_test_mock"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&second_page))
        .mount(&mock_server)
        .await;

    let client = common::make_test_client(&mock_server.uri(), "sk_test_mock");
    let orgs = client.search_organizations(3, "Ziff Davis").await.unwrap();

    assert_eq!(orgs.len(), 3);
    assert!(orgs.iter().all(|org| org.name.contains("Ziff Davis")));
}

#[tokio::test]
async fn create_sign_in_token_success() {
    let mock_server = MockServer::start().await;

    let body = serde_json::json!({
        "id": "sit_abc",
        "url": "https://clerk.example.com/sign-in?token=xyz",
        "status": "pending"
    });

    Mock::given(method("POST"))
        .and(path("/v1/sign_in_tokens"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .mount(&mock_server)
        .await;

    let client = common::make_test_client(&mock_server.uri(), "sk_test_mock");
    let token = client.create_sign_in_token("user_123", 3600).await.unwrap();

    assert_eq!(token.id, "sit_abc");
    assert!(token.url.contains("sign-in"));
}

#[tokio::test]
async fn api_error_handling() {
    let mock_server = MockServer::start().await;

    let body = serde_json::json!({
        "errors": [
            {"message": "User not found", "code": "resource_not_found"}
        ]
    });

    Mock::given(method("POST"))
        .and(path("/v1/sign_in_tokens"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&body))
        .mount(&mock_server)
        .await;

    let client = common::make_test_client(&mock_server.uri(), "sk_test_mock");
    let result = client.create_sign_in_token("invalid_user", 3600).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("User not found"));
}
