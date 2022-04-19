use crate::jwt::UserToken;
use crate::models::game::GameDTO;
use crate::models::response::Response;
use crate::services::game_service;
use crate::DbConn;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<game>")]
pub async fn create_game(
    game: Json<GameDTO>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = game_service::create(game.0, conn).await;
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[get("/")]
pub async fn list_game(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = game_service::get_list(conn).await;
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[get("/<game_id>")]
pub async fn find_game_by_id(
    game_id: i32,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: DbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let response = game_service::get_one(game_id, conn).await;
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
