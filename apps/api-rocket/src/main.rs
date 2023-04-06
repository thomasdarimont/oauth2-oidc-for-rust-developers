//#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use rocket::routes;
use std::error::Error;


use crate::middleware::auth::jwt::JwtAuth;
use rocket::tokio::task::spawn_blocking;

use crate::middleware::logging;

pub mod api;
pub mod domain;
pub mod middleware;
pub mod support;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::init_logging();

    let auth = spawn_blocking(JwtAuth::new).await?;

    let _ = rocket::build()
        .attach(middleware::cors::Cors)
        .mount("/", routes![api::me_info::get_me_info, api::me_info::options_me_info])
        .manage(auth)
        .launch()
        .await?;

    Ok(())
}
