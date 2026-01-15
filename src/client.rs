use crate::models::{ClerkError, CreateSignInTokenRequest, Organization, SignInToken, User};
use reqwest::Client;
use thiserror::Error;

const BASE_URL: &str = "https://api.clerk.com/v1";

#[derive(Error, Debug)]
pub enum ClerkClientError {
    #[error("CLERK_API_KEY not set")]
    MissingApiKey,
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Clerk API error: {0}")]
    Api(String),
}

pub struct ClerkClient {
    client: Client,
    api_key: String,
}

impl ClerkClient {
    pub fn new() -> Result<Self, ClerkClientError> {
        let api_key = std::env::var("CLERK_API_KEY")
            .map_err(|_| ClerkClientError::MissingApiKey)?;

        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    pub async fn list_users(
        &self,
        limit: u32,
        query: Option<&str>,
    ) -> Result<Vec<User>, ClerkClientError> {
        let mut url = format!("{}/users?limit={}&order_by=-created_at", BASE_URL, limit);
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
            return Err(ClerkClientError::Api(
                err.errors.first().map(|e| e.message.clone()).unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }

    pub async fn list_organizations(&self, limit: u32) -> Result<Vec<Organization>, ClerkClientError> {
        let url = format!("{}/organizations?limit={}&order_by=-created_at", BASE_URL, limit);

        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(ClerkClientError::Api(
                err.errors.first().map(|e| e.message.clone()).unwrap_or_default(),
            ));
        }

        let wrapper: OrganizationsResponse = resp.json().await?;
        Ok(wrapper.data)
    }

    pub async fn create_sign_in_token(
        &self,
        user_id: &str,
        expires_in_seconds: u32,
    ) -> Result<SignInToken, ClerkClientError> {
        let url = format!("{}/sign_in_tokens", BASE_URL);

        let body = CreateSignInTokenRequest {
            user_id: user_id.to_string(),
            expires_in_seconds,
        };

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(ClerkClientError::Api(
                err.errors.first().map(|e| e.message.clone()).unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }
}

#[derive(serde::Deserialize)]
struct OrganizationsResponse {
    data: Vec<Organization>,
}
