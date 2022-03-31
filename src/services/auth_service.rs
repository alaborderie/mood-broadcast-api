use crate::DbConn;
use crate::jwt;
use crate::models::response::{Response, ResponseWithStatus};
use crate::models::user::{LoginDTO, User, UserDTO};
use rocket::http::Status;
use rocket::serde::json::{json, from_str};

pub fn signup(user: UserDTO, conn: DbConn) -> ResponseWithStatus {
    if User::signup(user, &conn) {
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

pub fn login(login: LoginDTO, conn: DbConn) -> ResponseWithStatus {
    if let Some(result) = User::login(login, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from("Login was successfull."),
                data: from_str(json!({ "token": jwt::generate_token(result), "type": "Bearer" }))
                    .unwrap(),
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
