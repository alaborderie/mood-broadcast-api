use crate::models::mood::{Mood, MoodDTO};
use crate::models::response::{Response, ResponseWithStatus};
use crate::DbConn;
use rocket::http::Status;
use rocket::serde::json::{from_str, json};

pub async fn get_by_user_id(user_id: i32, conn: DbConn) -> ResponseWithStatus {
    println!("{:?}", user_id);
    let mood_vec = Mood::get_by_user_id(user_id, &conn).await;
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from("Ok"),
            data: json!(mood_vec),
        },
    }
}

/*
pub async fn get_one(mood_id: i32, conn: DbConn) -> ResponseWithStatus {
    if let Some(mood_object) = Mood::get_one(mood_id, &conn).await {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Ok"),
                data: json!(mood_object),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::NotFound.code,
            response: Response {
                message: String::from("Not found, no mood found with this id"),
                data: from_str("{}").unwrap(),
            },
        }
    }
}
*/
pub async fn create(mood: MoodDTO, conn: DbConn) -> ResponseWithStatus {
    if Mood::create(mood, &conn).await {
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
                message: String::from("Bad request, mood object not correct"),
                data: from_str("{}").unwrap(),
            },
        }
    }
}
