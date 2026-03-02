//! Fig Auth - Local stub (AWS removed)
//!
//! This is a minimal stub for local-only operation without AWS authentication.

pub mod consts;
mod error;
pub mod secret_store;

// Stub modules for AWS auth
pub mod builder_id {
    //! Builder ID authentication stub

    use crate::Result;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct BuilderIdToken {
        pub access_token: String,
        pub start_url: String,
        pub region: String,
    }

    impl BuilderIdToken {
        pub async fn load(_secret_store: &crate::secret_store::SecretStore, _validate: bool) -> Result<Option<Self>> {
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
        pub verification_uri_complete: String,
        pub expires_in: i32,
        pub interval: i32,
    }

    #[derive(Debug, Clone)]
    pub struct Region {
        name: String,
    }

    impl Region {
        pub fn new(name: &str) -> Self {
            Region { name: name.to_string() }
        }

        pub fn as_str(&self) -> &str {
            &self.name
        }
    }

    impl DeviceRegistration {
        pub async fn load_from_secret_store(
            _secret_store: &crate::secret_store::SecretStore,
            _region: &Region,
        ) -> Result<Option<Self>> {
            Ok(None)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum TokenType {
        BuilderId,
        Bearer,
        IamIdentityCenter,
    }

    impl BuilderIdToken {
        pub fn token_type(&self) -> TokenType {
            TokenType::BuilderId
        }
    }

    #[derive(Debug, Clone)]
    pub enum PollCreateToken {
        Pending,
        Complete(String), // Contains access token
        Error(String),
    }

    pub async fn start_device_authorization(_start_url: &str, _region: &Region) -> Result<DeviceRegistration> {
        Ok(DeviceRegistration {
            device_code: "local".to_string(),
            user_code: "local".to_string(),
            verification_uri: "https://local".to_string(),
            verification_uri_complete: "https://local".to_string(),
            expires_in: 600,
            interval: 5,
        })
    }

    pub async fn poll_create_token(_device_code: &str, _start_url: &str, _region: &Region) -> Result<PollCreateToken> {
        Ok(PollCreateToken::Complete("local-token".to_string()))
    }

    pub async fn start_pkce_authorization(
        _start_url: &str,
        _region: &Region,
        _redirect_uri: &str,
        _scopes: &[String],
    ) -> Result<String> {
        Ok("https://local-auth-url".to_string())
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

    use crate::Result;

    #[derive(Debug, Clone)]
    pub struct Region {
        name: String,
    }

    impl Region {
        pub fn new(name: &str) -> Self {
            Region { name: name.to_string() }
        }

        pub fn as_str(&self) -> &str {
            &self.name
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
pub async fn builder_id_token() -> Result<Option<builder_id::BuilderIdToken>> {
    let store = secret_store::SecretStore::new().await?;
    builder_id::BuilderIdToken::load(&store, false).await
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

pub async fn refresh_token() -> Result<Option<String>> {
    Ok(Some("local-token".to_string()))
}

// PKCE authorization wrapper
use crate::secret_store::SecretStore;

#[derive(Debug, Clone)]
pub struct PkceClient;

#[derive(Debug, Clone)]
pub struct PkceRegistration {
    pub url: String,
}

impl PkceRegistration {
    pub async fn finish(self, _client: &PkceClient, _secret_store: Option<&SecretStore>) -> Result<String> {
        Ok("local-token".to_string())
    }
}

pub async fn start_pkce_authorization(
    _start_url: Option<String>,
    _region: Option<String>,
) -> Result<(PkceClient, PkceRegistration)> {
    Ok((
        PkceClient,
        PkceRegistration {
            url: "https://local-pkce-url".to_string(),
        },
    ))
}

// Device authorization wrapper
pub async fn start_device_authorization(
    _secret_store: &secret_store::SecretStore,
    _start_url: Option<String>,
    _region: Option<String>,
) -> Result<builder_id::DeviceRegistration> {
    let region = builder_id::Region::new("us-east-1");
    builder_id::start_device_authorization("local", &region).await
}

pub async fn poll_create_token(
    _secret_store: &secret_store::SecretStore,
    _device_code: String,
    _start_url: Option<String>,
    _region: Option<String>,
) -> Result<builder_id::PollCreateToken> {
    let region = builder_id::Region::new("us-east-1");
    builder_id::poll_create_token("device_code", "start_url", &region).await
}
