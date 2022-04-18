use crate::models::game::{Game, GameDTO};
use crate::models::response::{Response, ResponseWithStatus};
use crate::DbConn;
use rocket::http::Status;
use rocket::serde::json::{from_str, json};

pub async fn get_list(conn: DbConn) -> ResponseWithStatus {
    let game_vec = Game::get_list(&conn).await;
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from("Ok"),
            data: json!(game_vec),
        },
    }
}

pub async fn get_one(game_id: i32, conn: DbConn) -> ResponseWithStatus {
    if let Some(game_object) = Game::get_one(game_id, &conn).await {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Ok"),
                data: json!(game_object),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: String::from("Not found, no game found with this id"),
                data: from_str("{}").unwrap(),
            },
        }
    }
}

pub async fn create(game: GameDTO, conn: DbConn) -> ResponseWithStatus {
    if Game::create(game, &conn).await {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Created"),
                data: from_str("{}").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from("Bad request, game object not correct"),
                data: from_str("{}").unwrap(),
            },
        }
    }
}
