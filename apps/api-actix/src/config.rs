use std::env;
pub struct Config {
    pub server_bind_addr: String,
    pub cert_location: String,
    pub key_location: String,
    pub oidc_issuer: String,
    pub allowed_cors_origin: String,
    pub log_level_default: String,
}

impl Config {
    pub fn from_environment_with_defaults() -> Self {
        Self {
            server_bind_addr: env::var("SERVER_BIND_ADDRESS").unwrap_or("127.0.0.1:4863".into()),
            cert_location: env::var("CERT_LOCATION").unwrap_or("../../certs/rust.test.pem".into()),
            key_location: env::var("KEY_LOCATION").unwrap_or("../../certs/rust.test-key.pem".into()),
            oidc_issuer: env::var("OIDC_ISSUER").unwrap_or("https://id.rust.test:8443/auth/realms/playground".into()),
            allowed_cors_origin: env::var("ALLOWED_CORS_ORIGIN").unwrap_or("https://apps.rust.test:4443".into()),
            log_level_default: env::var("LOG_LEVEL_DEFAULT").unwrap_or("info".into()),
        }
    }
}
