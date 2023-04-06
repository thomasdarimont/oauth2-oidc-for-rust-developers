// use actix_web::rt::task;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap as Map;

#[derive(Debug, Deserialize)]
pub struct FoundClaims {
    pub iat: usize,
    pub exp: usize,
    pub iss: String,
    pub sub: String,
    pub scope: String,
    pub preferred_username: Option<String>,

    #[serde(flatten)]
    pub other: Map<String, Value>,
}

impl FoundClaims {
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scope.split_ascii_whitespace().any(|s| s == scope)
    }
}
