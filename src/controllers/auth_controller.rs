use crate::DbConn;
use crate::models::response::Response;
use crate::models::user::{LoginDTO, UserDTO};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::services::auth_service;

#[post("/login", format = "json", data = "<login>")]
pub fn login(login: Json<LoginDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    let response = auth_service::login(login.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response)
    )
}

#[post("/signup", format = "json", data ="<user>")]
pub fn signup(user: Json<UserDTO>, conn: DbConn) -> status::Custom<Json<Response>> {
    let response = auth_service::signup(user.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response)
    )
}
