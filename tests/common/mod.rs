use reqwest::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Clerk API error: {0}")]
    Api(String),
}

pub struct TestClerkClient {
    client: Client,
    base_url: String,
    api_key: String,
}

const ORGANIZATIONS_PAGE_SIZE: u32 = 500;

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_addresses: Vec<EmailAddress>,
    pub primary_email_address_id: Option<String>,
    pub last_sign_in_at: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct EmailAddress {
    pub id: String,
    pub email_address: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub slug: Option<String>,
    pub members_count: Option<u32>,
    pub created_at: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct SignInToken {
    pub id: String,
    pub url: String,
    pub status: String,
}

#[derive(Debug, serde::Deserialize)]
struct ClerkError {
    errors: Vec<ClerkErrorDetail>,
}

#[derive(Debug, serde::Deserialize)]
struct ClerkErrorDetail {
    message: String,
}

#[derive(Debug, serde::Deserialize)]
struct OrganizationsResponse {
    data: Vec<Organization>,
    #[serde(default, rename = "total_count")]
    total_count: Option<u32>,
}

impl User {
    pub fn primary_email(&self) -> Option<&str> {
        if let Some(ref primary_id) = self.primary_email_address_id {
            self.email_addresses
                .iter()
                .find(|e| &e.id == primary_id)
                .map(|e| e.email_address.as_str())
        } else {
            self.email_addresses
                .first()
                .map(|e| e.email_address.as_str())
        }
    }

    pub fn display_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(f), Some(l)) => format!("{} {}", f, l),
            (Some(f), None) => f.clone(),
            (None, Some(l)) => l.clone(),
            (None, None) => String::new(),
        }
    }
}

impl TestClerkClient {
    pub async fn list_users(
        &self,
        limit: u32,
        query: Option<&str>,
    ) -> Result<Vec<User>, TestClientError> {
        let mut url = format!(
            "{}/v1/users?limit={}&order_by=-created_at",
            self.base_url, limit
        );
        if let Some(q) = query {
            url.push_str(&format!("&query={}", urlencoding::encode(q)));
        }

        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(TestClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }

    pub async fn list_organizations(
        &self,
        limit: u32,
    ) -> Result<Vec<Organization>, TestClientError> {
        self.paginate_organizations(limit, None).await
    }

    pub async fn search_organizations(
        &self,
        limit: u32,
        query: &str,
    ) -> Result<Vec<Organization>, TestClientError> {
        self.paginate_organizations(limit, Some(query)).await
    }

    pub async fn get_organization(
        &self,
        organization_id_or_slug: &str,
    ) -> Result<Organization, TestClientError> {
        let url = format!(
            "{}/v1/organizations/{}",
            self.base_url,
            urlencoding::encode(organization_id_or_slug)
        );

        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(TestClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }

    async fn paginate_organizations(
        &self,
        limit: u32,
        query: Option<&str>,
    ) -> Result<Vec<Organization>, TestClientError> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let mut organizations = Vec::new();
        let mut offset = 0u32;

        loop {
            let remaining = limit.saturating_sub(organizations.len() as u32);
            if remaining == 0 {
                break;
            }

            let page_limit = remaining.min(ORGANIZATIONS_PAGE_SIZE);
            let mut url = format!(
                "{}/v1/organizations?limit={}&offset={}&order_by=-created_at",
                self.base_url, page_limit, offset
            );

            if let Some(q) = query {
                url.push_str(&format!("&query={}", urlencoding::encode(q)));
            }

            let resp = self
                .client
                .get(&url)
                .bearer_auth(&self.api_key)
                .send()
                .await?;

            if !resp.status().is_success() {
                let err: ClerkError = resp.json().await?;
                return Err(TestClientError::Api(
                    err.errors
                        .first()
                        .map(|e| e.message.clone())
                        .unwrap_or_default(),
                ));
            }

            let OrganizationsResponse { data, total_count } = resp.json().await?;
            let fetched = data.len() as u32;
            organizations.extend(data);

            if organizations.len() as u32 >= limit {
                break;
            }

            let total_count = total_count.unwrap_or(organizations.len() as u32);
            let has_more = (organizations.len() as u32) < total_count;

            if !has_more || fetched == 0 {
                break;
            }

            offset = offset.saturating_add(fetched);
        }

        Ok(organizations)
    }

    pub async fn create_sign_in_token(
        &self,
        user_id: &str,
        expires_in_seconds: u32,
    ) -> Result<SignInToken, TestClientError> {
        let url = format!("{}/v1/sign_in_tokens", self.base_url);

        let body = serde_json::json!({
            "user_id": user_id,
            "expires_in_seconds": expires_in_seconds
        });

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(TestClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }
}

pub fn make_test_client(base_url: &str, api_key: &str) -> TestClerkClient {
    TestClerkClient {
        client: Client::new(),
        base_url: base_url.to_string(),
        api_key: api_key.to_string(),
    }
}
