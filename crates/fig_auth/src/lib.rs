//! Fig Auth - Local stub (AWS removed)
//!
//! This is a minimal stub for local-only operation without AWS authentication.

pub mod consts;
mod error;
pub mod secret_store;

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
