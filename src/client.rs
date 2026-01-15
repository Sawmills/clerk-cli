use crate::models::{
    ClerkError, CreateSignInTokenRequest, JwtTemplate, OrgMembership, OrgMembershipsResponse,
    Organization, Session, SessionToken, SignInToken, User,
};
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
        let api_key =
            std::env::var("CLERK_API_KEY").map_err(|_| ClerkClientError::MissingApiKey)?;

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
    ) -> Result<Vec<Organization>, ClerkClientError> {
        let url = format!(
            "{}/organizations?limit={}&order_by=-created_at",
            BASE_URL, limit
        );

        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(ClerkClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
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
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }

    pub async fn list_jwt_templates(&self) -> Result<Vec<JwtTemplate>, ClerkClientError> {
        let url = format!("{}/jwt_templates", BASE_URL);

        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(ClerkClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }

    pub async fn list_sessions(
        &self,
        user_id: &str,
        status: Option<&str>,
    ) -> Result<Vec<Session>, ClerkClientError> {
        let mut url = format!("{}/sessions?user_id={}", BASE_URL, user_id);
        if let Some(s) = status {
            url.push_str(&format!("&status={}", s));
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
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }

    pub async fn create_session_token(
        &self,
        session_id: &str,
        template_name: &str,
    ) -> Result<SessionToken, ClerkClientError> {
        let url = format!(
            "{}/sessions/{}/tokens/{}",
            BASE_URL, session_id, template_name
        );

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(ClerkClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
    }

    pub async fn exchange_ticket_for_session(
        &self,
        ticket: &str,
    ) -> Result<String, ClerkClientError> {
        let frontend_url = std::env::var("CLERK_FRONTEND_API")
            .unwrap_or_else(|_| "https://clerk.sawmills.ai".to_string());

        let url = format!("{}/v1/client/sign_ins?_clerk_js_version=5", frontend_url);

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("strategy=ticket&ticket={}", ticket))
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await?;
            return Err(ClerkClientError::Api(format!(
                "Failed to exchange ticket: {}",
                text
            )));
        }

        let json: serde_json::Value = resp.json().await?;
        json["response"]["created_session_id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| ClerkClientError::Api("No session created".to_string()))
    }

    pub async fn list_org_memberships(
        &self,
        org_id: &str,
        limit: u32,
    ) -> Result<Vec<OrgMembership>, ClerkClientError> {
        let url = format!(
            "{}/organizations/{}/memberships?limit={}",
            BASE_URL, org_id, limit
        );

        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err: ClerkError = resp.json().await?;
            return Err(ClerkClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        let wrapper: OrgMembershipsResponse = resp.json().await?;
        Ok(wrapper.data)
    }
}

#[derive(serde::Deserialize)]
struct OrganizationsResponse {
    data: Vec<Organization>,
}
