use crate::jwt::UserToken;
use crate::models::auth::Auth;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::DbConn;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub async fn signup(user: UserDTO, db: DbConn) -> bool {
        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let user = UserDTO {
            password: hashed_pwd,
            ..user
        };
        db.run(move |conn| {
            diesel::insert_into(users)
                .values(&user)
                .execute(conn)
                .is_ok()
        })
        .await
    }

    pub async fn login(login: LoginDTO, db: DbConn) -> Option<LoginInfoDTO> {
        let user_to_verify = db
            .run(move |conn| {
                users
                    .filter(username.eq(&login.username_or_email))
                    .or_filter(email.eq(&login.username_or_email))
                    .get_result::<User>(conn)
            })
            .await;
        if user_to_verify.is_err() {
            return None;
        }
        let user = user_to_verify.unwrap();
        if !user.password.is_empty() && verify(&login.password, &user.password).unwrap() {
            if let Some(auth) = Auth::create(&user.username, &db).await {
                if !Auth::save_auth(auth, &db).await {
                    return None;
                }
                let login_session_str = User::generate_login_session();
                let un: String = user.username.to_string();
                let session: String = login_session_str.to_string();
                User::update_login_session_to_db(un, session, &db).await;
                Some(LoginInfoDTO {
                    username: user.username.to_string(),
                    login_session: login_session_str,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn is_valid_login_session(user_token: UserToken, db: &DbConn) -> bool {
        db.run(move |conn| {
            users
                .filter(username.eq(&user_token.user))
                .filter(login_session.eq(&user_token.login_session))
                .get_result::<User>(conn)
                .is_ok()
        })
        .await
    }

    pub async fn find_user_by_username(un: String, db: &DbConn) -> Option<User> {
        db.run(move |conn| {
            let result_user = users.filter(username.eq(un)).get_result::<User>(conn);
            if let Ok(user) = result_user {
                Some(user)
            } else {
                None
            }
        })
        .await
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub async fn update_login_session_to_db(
        un: String,
        login_session_str: String,
        db: &DbConn,
    ) -> bool {
        if let Some(user) = User::find_user_by_username(un, db).await {
            db.run(move |conn| {
                diesel::update(users.find(user.id))
                    .set(login_session.eq(login_session_str))
                    .execute(conn)
                    .is_ok()
            })
            .await
        } else {
            false
        }
    }
}
