use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod api;
mod config;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_environment_with_defaults();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(&config.log_level_default));

    // workaround for fix default awc client used by OidcConfig::Issuer
    let oidc = middleware::oidc::fetch_oidc_config(&config).await;
    //let oidc = Oidc::new(OidcConfig::Issuer(config.oidc_issuer.clone().into())).await.unwrap();

    let jwt_validator = middleware::oidc::create_oidc_jwt_validator(&config);

    HttpServer::new(move || {
        let cors = middleware::cors::create_cors_config(config.allowed_cors_origin.to_string());

        App::new()
            .app_data(oidc.clone())
            // see https://actix.rs/actix-web/actix_web/middleware/struct.Logger.html
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" %T"))
            .wrap(jwt_validator.clone())
            .wrap(cors)
            .service(api::handle_me_info)
    })
    .bind_openssl(
        config.server_bind_addr,
        middleware::ssl::create_ssl_acceptor_builder(&config.cert_location, &config.key_location),
    )?
    .run()
    .await
}
