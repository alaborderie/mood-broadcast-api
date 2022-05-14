use crate::models::friend::Friend;
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

pub async fn get_by_friend_id(user_id: i32, friend_id: i32, conn: DbConn) -> ResponseWithStatus {
    if Friend::is_friend_with(user_id, friend_id, &conn).await {
        let mood_vec = Mood::get_by_user_id(friend_id, &conn).await;
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Ok"),
                data: json!(mood_vec),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::Forbidden.code,
            response: Response {
                message: String::from("Not friend with this user, cannot retrieve their moods."),
                data: from_str("{}").unwrap(),
            },
        }
    }
}

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
