use crate::config::Config;
use actix_4_jwt_auth::biscuit::jwk::JWKSet;
use actix_4_jwt_auth::biscuit::{Empty, Validation, ValidationOptions};
use actix_4_jwt_auth::{Oidc, OidcBiscuitValidator, OidcConfig};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use awc::http::header;
use awc::Connector;
use openssl::ssl::{SslConnector, SslMethod};

mod api;
mod config;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_environment_with_defaults();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(&config.log_level_default));

    let ssl_acceptor_builder =
        middleware::ssl::create_ssl_acceptor_builder(&config.cert_location, &config.key_location);

    // workaround for fix default awc client used by OidcConfig::Issuer
    let oidc = fetch_oidc_config(&config).await;
    //let oidc = Oidc::new(OidcConfig::Issuer(config.oidc_issuer.clone().into())).await.unwrap();

    let jwt_validator = create_oidc_jwt_validator(&config);

    HttpServer::new(move || {
        let cors = middleware::cors::create_cors_config(config.allowed_cors_origin.clone());

        App::new()
            .app_data(oidc.clone())
            // see https://actix.rs/actix-web/actix_web/middleware/struct.Logger.html
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" %T"))
            .wrap(jwt_validator.clone())
            .wrap(cors)
            .service(api::me_info::handle_me_info)
    })
    .bind_openssl(config.server_bind_addr, ssl_acceptor_builder)?
    .run()
    .await
}

fn create_oidc_jwt_validator(config: &Config) -> OidcBiscuitValidator {
    OidcBiscuitValidator {
        options: ValidationOptions {
            issuer: Validation::Validate(config.oidc_issuer.as_str().to_string()),
            ..ValidationOptions::default()
        },
    }
}

async fn fetch_oidc_config(config: &Config) -> Oidc {
    // fetch jwks, and construct OidcConfig from JWKS set

    let client_tls_config = SslConnector::builder(SslMethod::tls())
        .unwrap() //
        .build();

    let client = awc::Client::builder() //
        .add_default_header((header::USER_AGENT, "api-actix/0.1.0")) //
        .connector(Connector::new().openssl(client_tls_config)) //
        .finish();

    let jwks_key_set = client
        .get(config.oidc_issuer.as_str().to_string() + "/protocol/openid-connect/certs")
        .send()
        .await //
        .unwrap()
        .json::<JWKSet<Empty>>()
        .await
        .unwrap();

    Oidc::new(OidcConfig::Jwks(jwks_key_set)).await.unwrap()
}
