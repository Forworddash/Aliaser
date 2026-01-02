use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Represents a complete identity for a service
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Identity {
    pub service: String,
    #[zeroize(skip)]
    pub created_at: DateTime<Utc>,
    #[zeroize(skip)]
    pub updated_at: DateTime<Utc>,
    pub credentials: Credentials,
    pub personal_info: Option<PersonalInfo>,
    pub notes: Option<String>,
}

/// Credentials for authentication
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub alias: Option<String>,
}

/// Personal information for an identity
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct PersonalInfo {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthdate: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub custom_fields: Vec<CustomField>,
}

/// Custom key-value field
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct CustomField {
    pub key: String,
    pub value: String,
}

impl Identity {
    pub fn new(service: String, credentials: Credentials) -> Self {
        let now = Utc::now();
        Self {
            service,
            created_at: now,
            updated_at: now,
            credentials,
            personal_info: None,
            notes: None,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl PersonalInfo {
    pub fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
            birthdate: None,
            address: None,
            phone: None,
            custom_fields: Vec::new(),
        }
    }

    pub fn add_custom_field(&mut self, key: String, value: String) {
        self.custom_fields.push(CustomField { key, value });
    }
}

impl Default for PersonalInfo {
    fn default() -> Self {
        Self::new()
    }
}
