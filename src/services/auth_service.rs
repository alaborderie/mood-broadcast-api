use crate::jwt;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{LoginDTO, User, UserDTO};
use crate::DbConn;
use rocket::http::Status;
use rocket::serde::json::{from_str, json};

pub async fn signup(user: UserDTO, conn: DbConn) -> ResponseWithStatus {
    if User::signup(user, conn).await {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Signup was successfull."),
                data: from_str("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from("Bad request, try to sign up again with correct valeus!"),
                data: from_str("").unwrap(),
            },
        }
    }
}

pub async fn login(login: LoginDTO, conn: DbConn) -> ResponseWithStatus {
    if let Some(result) = User::login(login, conn).await {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Login was successfull."),
                data: json!({ "token": jwt::generate_token(result), "type": "Bearer" }),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from("Wrong email/password combination, try again."),
                data: from_str("").unwrap(),
            },
        }
    }
}
