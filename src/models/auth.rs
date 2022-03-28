use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use crate::schema::auth;
use crate::schema::auth::dsl::*;
use crate::models::user::User;

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(User)]
#[table_name = "auth"]
pub struct Auth {
    pub id: i32,
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct AuthDTO {
    pub user_id: i32,
    pub login_timestamp: DateTime<Utc>,
}

impl Auth {
    pub fn create(username: &str, conn: &PgConnection) -> Option<AuthDTO> {
        if let Some(user) = User::find_user_by_username(username, conn) {
            Some(AuthDTO {
                user_id: user.id,
                login_timestamp: Utc::now(),
            })
        } else {
            None
        }
    }

    pub fn save_auth(insert_record: AuthDTO, conn: &PgConnection) -> bool {
        diesel::insert_into(auth)
            .values(&insert_record)
            .execute(conn)
            .is_ok()
    }
}
