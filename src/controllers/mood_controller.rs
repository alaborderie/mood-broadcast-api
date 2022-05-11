use crate::jwt::UserToken;
use crate::models::mood::{MoodDTO, PartialMood};
use crate::models::user::User;
use crate::models::response::Response;
use crate::services::mood_service;
use crate::DbConn;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<partial_mood>")]
pub async fn create_mood(
    partial_mood: Json<PartialMood>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let user_token = token.unwrap();
    let user_id = User::find_user_id_from_login_session(user_token, &conn).await;
    let mood = MoodDTO {
        user_id: user_id.unwrap(),
        game_id: partial_mood.game_id,
        begin_timestamp: partial_mood.begin_timestamp,
        end_timestamp: partial_mood.end_timestamp
    };
    let response = mood_service::create(mood, conn).await;
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

//#[get("/friends")]
//pub async fn get_moods_of_friends

#[get("/friends/<friend_id>")]
pub async fn get_moods_by_friend_id(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    friend_id: i32,
    conn: DbConn
    ) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let user_token = token.unwrap();
    let user_id = User::find_user_id_from_login_session(user_token, &conn).await;
    let response = mood_service::get_by_friend_id(user_id.unwrap_or(0), friend_id, conn).await;
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
        )
}

