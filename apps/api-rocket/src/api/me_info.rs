use crate::domain::user::User;
use chrono::offset::Utc;
use rocket::serde::json::Json;
use rocket::{get, options};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct MeInfo {
    pub message: String,
    pub backend: String,
    pub datetime: String,
}

#[options("/api/users/me")]
pub fn options_me_info() {}

#[get("/api/users/me")]
pub fn get_me_info(user: User) -> Json<MeInfo> {
    log::info!("Handle user info request. username={}", &user.username);

    let info = MeInfo {
        datetime: Utc::now().to_string(),
        message: format!("Hello, {}!", user.username),
        backend: String::from("rust-rocket"),
    };

    Json(info)
}
