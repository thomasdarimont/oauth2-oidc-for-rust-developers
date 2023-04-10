use std::env;

pub struct Config {
    pub issuer_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub device_auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
}

impl Config {
    pub fn from_environment_with_defaults() -> Self {
        let issuer_url = env::var("OIDC_ISSUER")
            .unwrap_or("https://id.rust.test:8443/auth/realms/playground".into());

        Self {
            issuer_url: issuer_url.to_string(),
            client_id: env::var("CLIENT_ID").unwrap_or("demo-device-client".into()),
            client_secret: env::var("CLIENT_SECRET").unwrap_or("secret".into()),
            scope: env::var("CLIENT_SCOPE").unwrap_or("openid email offline_access".into()),
            device_auth_url: env::var("DEVICE_AUTH_URL")
                .unwrap_or(issuer_url.to_string() + "/protocol/openid-connect/auth/device"),
            token_url: env::var("TOKEN_URL")
                .unwrap_or(issuer_url.to_string() + "/protocol/openid-connect/token"),
            user_info_url: env::var("USER_INFO_URL")
                .unwrap_or(issuer_url + "/protocol/openid-connect/userinfo"),
        }
    }
}
