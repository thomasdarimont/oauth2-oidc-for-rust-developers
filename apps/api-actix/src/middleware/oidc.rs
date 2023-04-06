use crate::config::Config;
use actix_4_jwt_auth::biscuit::jwk::JWKSet;
use actix_4_jwt_auth::biscuit::{Empty, Validation, ValidationOptions};
use actix_4_jwt_auth::{Oidc, OidcBiscuitValidator, OidcConfig};
use actix_web::http::header;
use awc::Connector;
use openssl::ssl::{SslConnector, SslMethod};

pub fn create_oidc_jwt_validator(config: &Config) -> OidcBiscuitValidator {
    OidcBiscuitValidator {
        options: ValidationOptions {
            issuer: Validation::Validate(config.oidc_issuer.to_string()),
            ..ValidationOptions::default()
        },
    }
}

pub async fn fetch_oidc_config(config: &Config) -> Oidc {
    // fetch jwks, and construct OidcConfig from JWKS set

    let jwks = fetch_jwks(&(config.oidc_issuer.clone() + "/protocol/openid-connect/certs")).await;

    Oidc::new(OidcConfig::Jwks(jwks)).await.unwrap()
}

async fn fetch_jwks(jwks_location: &str) -> JWKSet<Empty> {
    let client_tls_config = SslConnector::builder(SslMethod::tls())
        .unwrap() //
        .build();

    let client = awc::Client::builder() //
        .add_default_header((header::USER_AGENT, "api-actix/0.1.0")) //
        .connector(Connector::new().openssl(client_tls_config)) //
        .finish();

    let jwks_request = client.get(jwks_location).send().await;

    let jwks_result = match jwks_request {
        Ok(mut res) => res.json::<JWKSet<Empty>>().await,
        _ => panic!(
            "Could not fetch JWKS Keyset from {}. Is the server running?",
            &jwks_location
        ),
    };

    let Ok(jwks) = jwks_result
        else { panic!("Could not parse JWKS") };
    jwks
}
