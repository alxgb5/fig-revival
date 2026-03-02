//! Fig Auth - Local stub (AWS removed)
//!
//! This is a minimal stub for local-only operation without AWS authentication.

pub mod consts;
mod error;
pub mod secret_store;

// Stub modules for AWS auth
pub mod builder_id {
    //! Builder ID authentication stub

    use crate::{Error, Result};

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct BuilderIdToken {
        pub access_token: String,
        pub start_url: String,
        pub region: String,
    }

    impl BuilderIdToken {
        pub async fn load() -> Result<Option<Self>> {
            Ok(Some(BuilderIdToken {
                access_token: "local-token".to_string(),
                start_url: "local".to_string(),
                region: "us-east-1".to_string(),
            }))
        }
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct DeviceRegistration {
        pub device_code: String,
        pub user_code: String,
        pub verification_uri: String,
        pub expires_in: i32,
        pub interval: i32,
    }

    impl DeviceRegistration {
        pub async fn load_from_secret_store(_start_url: &str) -> Result<Option<Self>> {
            Ok(None)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum TokenType {
        BuilderId,
        Bearer,
    }

    #[derive(Debug, Clone)]
    pub struct PollCreateToken {
        pub access_token: String,
        pub token_type: TokenType,
    }

    pub async fn start_device_authorization(_start_url: &str) -> Result<DeviceRegistration> {
        Ok(DeviceRegistration {
            device_code: "local".to_string(),
            user_code: "local".to_string(),
            verification_uri: "https://local".to_string(),
            expires_in: 600,
            interval: 5,
        })
    }

    pub async fn poll_create_token(_device_code: &str, _start_url: &str) -> Result<PollCreateToken> {
        Ok(PollCreateToken {
            access_token: "local-token".to_string(),
            token_type: TokenType::BuilderId,
        })
    }

    pub async fn login(_start_url: Option<String>) -> Result<BuilderIdToken> {
        Ok(BuilderIdToken {
            access_token: "local-token".to_string(),
            start_url: "local".to_string(),
            region: "us-east-1".to_string(),
        })
    }

    pub async fn logout() -> Result<()> {
        Ok(())
    }
}

pub mod pkce {
    //! PKCE authentication stub

    use crate::{Error, Result};

    #[derive(Debug, Clone, Copy)]
    pub struct Region {
        name: &'static str,
    }

    impl Region {
        pub fn new(name: &'static str) -> Self {
            Region { name }
        }

        pub fn as_str(&self) -> &str {
            self.name
        }
    }

    pub async fn start_pkce_authorization(_region: Region) -> Result<String> {
        Ok("https://local".to_string())
    }

    pub async fn login() -> Result<()> {
        Ok(())
    }

    pub async fn logout() -> Result<()> {
        Ok(())
    }
}

pub use consts::{AMZN_START_URL, START_URL};
pub use error::Error;
pub(crate) use error::Result;

// Stubbed functions - no AWS authentication in local version
pub async fn builder_id_token() -> Result<String> {
    Ok("local-token".to_string())
}

pub async fn is_amzn_user() -> bool {
    false
}

pub async fn is_logged_in() -> bool {
    true // Always "logged in" for local version
}

pub async fn logout() -> Result<()> {
    Ok(())
}

pub async fn refresh_token() -> Result<String> {
    Ok("local-token".to_string())
}
