use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_addresses: Vec<EmailAddress>,
    pub primary_email_address_id: Option<String>,
    pub last_sign_in_at: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct EmailAddress {
    pub id: String,
    pub email_address: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub slug: Option<String>,
    pub members_count: Option<u32>,
    pub created_at: i64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SignInToken {
    pub id: String,
    pub url: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSignInTokenRequest {
    pub user_id: String,
    pub expires_in_seconds: u32,
}

#[derive(Debug, Deserialize)]
pub struct ClerkError {
    pub errors: Vec<ClerkErrorDetail>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct JwtTemplate {
    pub id: String,
    pub name: String,
    pub lifetime: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub last_active_organization_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SessionToken {
    pub jwt: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ClerkErrorDetail {
    pub message: String,
    pub code: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_user(
        first: Option<&str>,
        last: Option<&str>,
        emails: Vec<(&str, &str)>,
        primary_id: Option<&str>,
    ) -> User {
        User {
            id: "user_123".to_string(),
            first_name: first.map(String::from),
            last_name: last.map(String::from),
            email_addresses: emails
                .into_iter()
                .map(|(id, addr)| EmailAddress {
                    id: id.to_string(),
                    email_address: addr.to_string(),
                })
                .collect(),
            primary_email_address_id: primary_id.map(String::from),
            last_sign_in_at: None,
            created_at: 0,
        }
    }

    #[test]
    fn display_name_both() {
        let user = make_user(Some("John"), Some("Doe"), vec![], None);
        assert_eq!(user.display_name(), "John Doe");
    }

    #[test]
    fn display_name_first_only() {
        let user = make_user(Some("John"), None, vec![], None);
        assert_eq!(user.display_name(), "John");
    }

    #[test]
    fn display_name_last_only() {
        let user = make_user(None, Some("Doe"), vec![], None);
        assert_eq!(user.display_name(), "Doe");
    }

    #[test]
    fn display_name_none() {
        let user = make_user(None, None, vec![], None);
        assert_eq!(user.display_name(), "");
    }

    #[test]
    fn primary_email_with_id() {
        let user = make_user(
            None,
            None,
            vec![("email_1", "a@test.com"), ("email_2", "b@test.com")],
            Some("email_2"),
        );
        assert_eq!(user.primary_email(), Some("b@test.com"));
    }

    #[test]
    fn primary_email_fallback_first() {
        let user = make_user(
            None,
            None,
            vec![("email_1", "a@test.com"), ("email_2", "b@test.com")],
            None,
        );
        assert_eq!(user.primary_email(), Some("a@test.com"));
    }

    #[test]
    fn primary_email_empty() {
        let user = make_user(None, None, vec![], None);
        assert_eq!(user.primary_email(), None);
    }
}
