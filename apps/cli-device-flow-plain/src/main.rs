use crate::config::Config;
use chrono::{Duration, Local};
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Error;

mod config;

const GRANT_TYPE_DEVICE_FLOW: &str = "urn:ietf:params:oauth:grant-type:device_code";

#[tokio::main]
async fn main() {
    let config = config::Config::from_environment_with_defaults();

    println!("Starting device flow...");
    let response = start_device_flow(&config);

    let device_code_response = response.await.unwrap();
    println!("{device_code_response}");

    let verification_uri = device_code_response.get("verification_uri").unwrap();
    let user_code = device_code_response.get("user_code").unwrap();
    println!(
        "Browse to {} and enter the code {}",
        verification_uri, user_code
    );

    println!("--- OR ----");

    let verification_uri_complete = device_code_response
        .get("verification_uri_complete")
        .unwrap();
    println!("Browse to {}", verification_uri_complete);

    println!("Waiting for device flow completion...");

    let expires_in_seconds = device_code_response
        .get("expires_in")
        .unwrap()
        .as_i64()
        .unwrap();
    let expiry_time = Local::now() + Duration::seconds(expires_in_seconds);
    let device_code = device_code_response
        .get("device_code")
        .unwrap()
        .as_str()
        .unwrap();

    while Local::now() < expiry_time {
        println!("Checking for device flow completion...");
        let device_flow_response = check_device_flow_completion(&config, device_code).await;
        match device_flow_response {
            Ok(value) => {
                println!("Device flow completed!");

                let access_token = value.get("access_token").unwrap().as_str().unwrap();
                let refresh_token = value.get("refresh_token").unwrap().as_str().unwrap();

                println!("Access-token: {access_token}");
                println!("Refresh-token: {refresh_token}");

                // you could now store the tokens on the device
                // - use the refresh-token to obtain new tokens
                // - use the access-token to call APIs

                // let's call user-info
                let user_info = fetch_user_info(&config, access_token).await.unwrap();

                println!("User Info: {user_info}");
                break;
            }
            _ => {
                println!("Device flow pending...");
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                continue;
            }
        };
    }
}

async fn check_device_flow_completion(
    config: &Config,
    device_code: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("client_id", config.client_id.as_str());
    map.insert("client_secret", config.client_secret.as_str());
    map.insert("device_code", device_code);
    map.insert("grant_type", GRANT_TYPE_DEVICE_FLOW);

    let client = reqwest::Client::new();
    let response = client
        .post(config.token_url.as_str())
        .form(&map)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(_error) = response.get("error") {
        return Err(Box::new(Error::default())); // TODO how to pass _error within Err(...?)
    }
    Ok(response)
}

async fn start_device_flow(config: &Config) -> Result<Value, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("client_id", config.client_id.as_str());
    map.insert("client_secret", config.client_secret.as_str());
    map.insert("grant_type", GRANT_TYPE_DEVICE_FLOW);
    map.insert("scope", config.scope.as_str());

    let client = reqwest::Client::new();
    let response = client
        .post(config.device_auth_url.as_str())
        .form(&map)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

async fn fetch_user_info(
    config: &Config,
    access_token: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(config.user_info_url.as_str())
        .header(AUTHORIZATION, "Bearer ".to_string() + access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}
