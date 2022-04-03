use crate::models::user::User;
use crate::schema::auth;
use crate::schema::auth::dsl::*;
use crate::DbConn;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

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
    pub async fn create(username: &str, conn: &DbConn) -> Option<AuthDTO> {
        if let Some(user) = User::find_user_by_username(username.to_string(), conn).await {
            Some(AuthDTO {
                user_id: user.id,
                login_timestamp: Utc::now(),
            })
        } else {
            None
        }
    }

    pub async fn save_auth(insert_record: AuthDTO, db: &DbConn) -> bool {
        db.run(move |conn| {
            diesel::insert_into(auth)
                .values(&insert_record)
                .execute(conn)
                .is_ok()
        })
        .await
    }
}
