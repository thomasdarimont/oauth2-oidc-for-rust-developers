use oauth2::basic::BasicClient;
use oauth2::devicecode::{DeviceAuthorizationResponse, ExtraDeviceAuthorizationFields};
use oauth2::reqwest::http_client;
use oauth2::{AuthType, AuthUrl, ClientId, ClientSecret, DeviceAuthorizationUrl, Scope, TokenUrl};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct StoringFields(HashMap<String, serde_json::Value>);

impl ExtraDeviceAuthorizationFields for StoringFields {}

type StoringDeviceAuthorizationResponse = DeviceAuthorizationResponse<StoringFields>;

fn main() {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").unwrap_or("demo-device-client".to_string()));
    let client_secret =
        ClientSecret::new(env::var("CLIENT_SECRET").unwrap_or("secret".to_string()));
    let issuer_url = "https://id.rust.test:8443/auth/realms/playground";

    let auth_url = AuthUrl::new(issuer_url.to_string() + "/protocol/openid-connect/auth/device")
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(issuer_url.to_string() + "/protocol/openid-connect/token")
        .expect("Invalid token endpoint URL");
    let device_auth_url = DeviceAuthorizationUrl::new(
        issuer_url.to_string() + "/protocol/openid-connect/auth/device",
    )
    .expect("Invalid device authorization endpoint URL");

    // Set up the config for the Keycloak OAuth2 process.
    //
    // Google's OAuth endpoint expects the client_id to be in the request body,
    // so ensure that option is set.
    let device_client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_device_authorization_url(device_auth_url)
        .set_auth_type(AuthType::RequestBody);

    // Request the set of codes from the Device Authorization endpoint.
    let details: StoringDeviceAuthorizationResponse = device_client
        .exchange_device_code()
        .unwrap()
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .request(http_client)
        .expect("Failed to request codes from device auth endpoint");

    // Display the URL and user-code.
    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        details.verification_uri().to_string(),
        details.user_code().secret().to_string()
    );

    // Now poll for the token
    let token = device_client
        .exchange_device_access_token(&details)
        .request(http_client, std::thread::sleep, None)
        .expect("Failed to get token");

    println!("Keycloak returned the following token:\n{:?}\n", token);
}
