use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::users;
use crate::schema::users::dsl::*;
use uuid::Uuid;
use crate::models::auth::Auth;
use crate::jwt::UserToken;
use crate::DbConn;

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
        }).await
    }

    pub fn login(login: LoginDTO, db: DbConn) -> Option<LoginInfoDTO> {
        let user_to_verify = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(db)
            .unwrap();
        if !user_to_verify.password.is_empty()
            && verify(&login.password, &user_to_verify.password).unwrap()
        {
            if let Some(auth) = Auth::create(&user_to_verify.username, conn) {
                if !Auth::save_auth(auth, conn) {
                    return None;
                }
                let login_session_str = User::generate_login_session();
                User::update_login_session_to_db(&user_to_verify.username, &login_session_str, conn);
                Some(LoginInfoDTO {
                    username: user_to_verify.username,
                    login_session: login_session_str,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &DbConn) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_username(un: &str, conn: &DbConn) -> Option<User> {
        let result_user = users.filter(username.eq(un)).get_result::<User>(conn);
        if let Ok(user) = result_user {
            Some(user)
        } else {
            None
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn update_login_session_to_db(un: &str, login_session_str: &str, conn: &DbConn) -> bool {
        if let Some(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
            .set(login_session.eq(login_session_str.to_string()))
            .execute(conn)
            .is_ok()
        } else {
            false
        }
    }
}
