use crate::jwt::UserToken;
use crate::models::mood::MoodDTO;
use crate::models::user::User;
use crate::models::response::Response;
use crate::services::{mood_service};
use crate::DbConn;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<mood>")]
pub async fn create_mood(
    mood: Json<MoodDTO>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = mood_service::create(mood.0, conn).await;
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[get("/")]
pub async fn list_user_moods(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let user_token = token.unwrap();
    let user_id = User::find_user_id_from_login_session(user_token, &conn).await;
    let response = mood_service::get_by_user_id(user_id.unwrap_or(0), conn).await;
    println!("{:?}", response);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/*
#[get("/<mood_id>")]
pub async fn find_mood_by_id(
    mood_id: i32,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = mood_service::get_one(mood_id, conn).await;
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
*/
