use crate::models::response::Response;
use crate::models::user::{LoginInfoDTO, User};
use crate::DbConn;
use chrono::Utc;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::{Header, Validation};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket::serde::json::{from_str, Json};
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

static ONE_DAY: i64 = 60 * 60 * 24; // in seconds

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = status::Custom<Json<Response>>;
    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<Self, status::Custom<Json<Response>>> {
        let conn = request.guard::<DbConn>().await.succeeded();
        if conn.is_some() {
            if let Some(authen_header) = request.headers().get_one("Authorization") {
                let authen_str = authen_header.to_string();
                if authen_str.starts_with("Bearer") {
                    let token = authen_str[6..authen_str.len()].trim();
                    if let Ok(token_data) = decode_token(token.to_string()) {
                        let claims = token_data.claims.clone();
                        if verify_token(token_data, &conn.unwrap()).await {
                            return Outcome::Success(claims);
                        }
                    }
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from("Invalid token, please login again"),
                    data: from_str("").unwrap(),
                }),
            ),
        ))
    }
}

pub fn generate_token(login: LoginInfoDTO) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + ONE_DAY,
        user: login.username,
        login_session: login.login_session,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(include_bytes!("secret.key")),
    )
    .unwrap()
}

fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(include_bytes!("secret.key")),
        &Validation::default(),
    )
}

async fn verify_token(token_data: TokenData<UserToken>, conn: &DbConn) -> bool {
    User::is_valid_login_session(token_data.claims, conn).await
}
