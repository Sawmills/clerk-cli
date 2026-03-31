use crate::models::{
    ClerkError, CreateOrgMembershipRequest, CreateOrganizationRequest, CreateSamlConnectionRequest,
    CreateSignInTokenRequest, CreateUserRequest, JwtTemplate, OrgMembership,
    OrgMembershipsResponse, Organization, SamlConnection, SamlConnectionsResponse, Session,
    SessionToken, SignInToken, UpdateSamlConnectionRequest, User,
};
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;

const BASE_URL: &str = "https://api.clerk.com/v1";
const ORGANIZATIONS_PAGE_SIZE: u32 = 500;

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
    frontend_api: String,
}

impl ClerkClient {
    pub fn new() -> Result<Self, ClerkClientError> {
        let api_key =
            std::env::var("CLERK_API_KEY").map_err(|_| ClerkClientError::MissingApiKey)?;

        Ok(Self {
            client: Client::new(),
            api_key,
            frontend_api: std::env::var("CLERK_FRONTEND_API")
                .unwrap_or_else(|_| "https://clerk.sawmills.ai".to_string()),
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
        self.paginate_organizations(limit, None).await
    }

    pub async fn search_organizations(
        &self,
        limit: u32,
        query: &str,
    ) -> Result<Vec<Organization>, ClerkClientError> {
        self.paginate_organizations(limit, Some(query)).await
    }

    async fn paginate_organizations(
        &self,
        limit: u32,
        query: Option<&str>,
    ) -> Result<Vec<Organization>, ClerkClientError> {
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
            let OrganizationsResponse { data, total_count } = self
                .fetch_organizations_page(page_limit, offset, query)
                .await?;
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

    async fn fetch_organizations_page(
        &self,
        limit: u32,
        offset: u32,
        query: Option<&str>,
    ) -> Result<OrganizationsResponse, ClerkClientError> {
        let mut url = format!(
            "{}/organizations?limit={}&offset={}&order_by=-created_at",
            BASE_URL, limit, offset
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
            return Err(ClerkClientError::Api(
                err.errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_default(),
            ));
        }

        Ok(resp.json().await?)
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

    pub async fn create_session_token_with_org(
        &self,
        user_id: &str,
        org_id: &str,
        template_name: &str,
        user_email: &str,
    ) -> Result<SessionToken, ClerkClientError> {
        let sign_in_token = self.create_sign_in_token(user_id, 300).await?;
        let ticket = self.extract_ticket(&sign_in_token)?;

        let (sign_in_id, auth_header) = self.start_sign_in_attempt(user_email).await?;
        let (session_id, auth_header) = self
            .complete_ticket_first_factor(&sign_in_id, &auth_header, &ticket)
            .await?;

        self.touch_session(&session_id, org_id, &auth_header)
            .await?;
        let jwt = self
            .mint_session_token(&session_id, org_id, template_name, &auth_header)
            .await?;

        Ok(SessionToken { jwt })
    }

    fn extract_ticket(&self, token: &SignInToken) -> Result<String, ClerkClientError> {
        if let Some(t) = &token.token {
            return Ok(t.clone());
        }

        token
            .url
            .split("__clerk_ticket=")
            .nth(1)
            .map(|s| s.to_string())
            .ok_or_else(|| {
                ClerkClientError::Api("Failed to extract ticket from sign-in token".to_string())
            })
    }

    async fn start_sign_in_attempt(
        &self,
        email: &str,
    ) -> Result<(String, String), ClerkClientError> {
        let body = format!("identifier={}", urlencoding::encode(email));
        let resp = self
            .client
            .post(format!("{}/v1/client/sign_ins", self.frontend_api))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ClerkClientError::Api(format!(
                "Sign-in start failed ({}): {}",
                status, text
            )));
        }

        let auth_header = resp
            .headers()
            .get("authorization")
            .or_else(|| resp.headers().get("Authorization"))
            .and_then(|h| h.to_str().ok())
            .map(|h| h.to_string())
            .ok_or_else(|| {
                ClerkClientError::Api("Authorization header missing from sign-in start".to_string())
            })?;

        let data: Value = resp.json().await.unwrap_or_else(|_| Value::Null);
        let sign_in_id = data
            .get("response")
            .and_then(|r| r.get("id"))
            .and_then(|v| v.as_str())
            .or_else(|| data.get("id").and_then(|v| v.as_str()))
            .ok_or_else(|| {
                ClerkClientError::Api("Sign-in start did not return an id".to_string())
            })?;

        Ok((sign_in_id.to_string(), ensure_bearer(auth_header)))
    }

    async fn complete_ticket_first_factor(
        &self,
        sign_in_id: &str,
        auth_header: &str,
        ticket: &str,
    ) -> Result<(String, String), ClerkClientError> {
        let body = format!("strategy=ticket&ticket={}", urlencoding::encode(ticket));
        let resp = self
            .client
            .post(format!(
                "{}/v1/client/sign_ins/{}/attempt_first_factor",
                self.frontend_api, sign_in_id
            ))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", auth_header)
            .body(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ClerkClientError::Api(format!(
                "Ticket factor failed ({}): {}",
                status, text
            )));
        }

        let next_auth = resp
            .headers()
            .get("authorization")
            .or_else(|| resp.headers().get("Authorization"))
            .and_then(|h| h.to_str().ok())
            .map(|h| h.to_string())
            .ok_or_else(|| {
                ClerkClientError::Api(
                    "Authorization header missing from ticket factor response".to_string(),
                )
            })?;

        let data: Value = resp.json().await.unwrap_or_else(|_| Value::Null);
        let session_id = data
            .get("response")
            .and_then(|r| r.get("created_session_id"))
            .and_then(|v| v.as_str())
            .or_else(|| data.get("created_session_id").and_then(|v| v.as_str()))
            .ok_or_else(|| {
                ClerkClientError::Api(format!(
                    "Ticket flow did not yield a session id (status: {:?})",
                    data.get("response").and_then(|r| r.get("status"))
                ))
            })?;

        Ok((session_id.to_string(), ensure_bearer(next_auth)))
    }

    async fn touch_session(
        &self,
        session_id: &str,
        org_id: &str,
        auth_header: &str,
    ) -> Result<(), ClerkClientError> {
        let body = format!("active_organization_id={}", urlencoding::encode(org_id));
        let resp = self
            .client
            .post(format!(
                "{}/v1/client/sessions/{}/touch",
                self.frontend_api, session_id
            ))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", auth_header)
            .body(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ClerkClientError::Api(format!(
                "Session touch failed ({}): {}",
                status, text
            )));
        }

        Ok(())
    }

    async fn mint_session_token(
        &self,
        session_id: &str,
        org_id: &str,
        template_name: &str,
        auth_header: &str,
    ) -> Result<String, ClerkClientError> {
        let body = format!("organization_id={}", urlencoding::encode(org_id));
        let mut path = format!(
            "{}/v1/client/sessions/{}/tokens",
            self.frontend_api, session_id
        );
        if !template_name.is_empty() {
            path.push('/');
            path.push_str(template_name);
        }

        let resp = self
            .client
            .post(path)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", auth_header)
            .body(body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ClerkClientError::Api(format!(
                "Session token mint failed ({}): {}",
                status, text
            )));
        }

        let data: Value = resp.json().await.unwrap_or_else(|_| Value::Null);
        let token = data
            .get("jwt")
            .and_then(|v| v.as_str())
            .or_else(|| data.get("token").and_then(|v| v.as_str()))
            .ok_or_else(|| ClerkClientError::Api("Empty JWT returned".to_string()))?;

        Ok(token.to_string())
    }

    pub async fn exchange_ticket_for_session(
        &self,
        ticket: &str,
    ) -> Result<String, ClerkClientError> {
        let url = format!(
            "{}/v1/client/sign_ins?_clerk_js_version=5",
            self.frontend_api
        );

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

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User, ClerkClientError> {
        let url = format!("{}/users", BASE_URL);

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
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

    pub async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, ClerkClientError> {
        let url = format!("{}/organizations", BASE_URL);

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
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

    pub async fn create_org_membership(
        &self,
        org_id: &str,
        request: CreateOrgMembershipRequest,
    ) -> Result<OrgMembership, ClerkClientError> {
        let url = format!("{}/organizations/{}/memberships", BASE_URL, org_id);

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
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

    pub async fn delete_org_membership(
        &self,
        org_id: &str,
        user_id: &str,
    ) -> Result<(), ClerkClientError> {
        let url = format!(
            "{}/organizations/{}/memberships/{}",
            BASE_URL, org_id, user_id
        );

        let resp = self
            .client
            .delete(&url)
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

        Ok(())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User, ClerkClientError> {
        let url = format!("{}/users/{}", BASE_URL, user_id);

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

    pub async fn list_saml_connections(
        &self,
        organization_id: Option<&str>,
    ) -> Result<Vec<SamlConnection>, ClerkClientError> {
        let mut url = format!("{}/saml_connections?limit=100", BASE_URL);
        if let Some(org_id) = organization_id {
            url.push_str(&format!("&organization_id={}", org_id));
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

        let wrapper: SamlConnectionsResponse = resp.json().await?;
        Ok(wrapper.data)
    }

    pub async fn create_saml_connection(
        &self,
        request: CreateSamlConnectionRequest,
    ) -> Result<SamlConnection, ClerkClientError> {
        let url = format!("{}/saml_connections", BASE_URL);

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
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

    pub async fn delete_saml_connection(
        &self,
        connection_id: &str,
    ) -> Result<(), ClerkClientError> {
        let url = format!("{}/saml_connections/{}", BASE_URL, connection_id);

        let resp = self
            .client
            .delete(&url)
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

        Ok(())
    }

    pub async fn update_saml_connection(
        &self,
        connection_id: &str,
        request: UpdateSamlConnectionRequest,
    ) -> Result<SamlConnection, ClerkClientError> {
        let url = format!("{}/saml_connections/{}", BASE_URL, connection_id);

        let resp = self
            .client
            .patch(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
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

    pub async fn delete_organization(&self, org_id: &str) -> Result<(), ClerkClientError> {
        let url = format!("{}/organizations/{}", BASE_URL, org_id);

        let resp = self
            .client
            .delete(&url)
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

        Ok(())
    }
}

fn ensure_bearer(header: String) -> String {
    if header.starts_with("Bearer ") {
        header
    } else {
        format!("Bearer {}", header)
    }
}

#[derive(serde::Deserialize)]
struct OrganizationsResponse {
    data: Vec<Organization>,
    #[serde(default, rename = "total_count")]
    total_count: Option<u32>,
}
