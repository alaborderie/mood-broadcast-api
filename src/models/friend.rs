use crate::schema::friends;
use crate::schema::friends::dsl::*;
use crate::DbConn;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "friends"]
pub struct Friend {
    pub id: i32,
    pub user_from_id: i32,
    pub user_to_id: i32,
    pub status: String,
    pub update_timestamp: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "friends"]
pub struct FriendDTO {
    pub user_from_id: i32,
    pub user_to_id: i32,
    pub status: String,
    pub update_timestamp: DateTime<Utc>,
}

impl Friend {
    pub async fn is_friend_with(user_id: i32, friend_id: i32, db: &DbConn) -> bool {
        let result_friendship_id = db
            .run(move |conn| {
                friends
                    .select(friends::id)
                    .filter(user_from_id.eq(user_id).or(user_from_id.eq(friend_id)))
                    .filter(user_to_id.eq(friend_id).or(user_to_id.eq(user_id)))
                    .get_result::<i32>(conn)
            })
            .await;
        matches!(result_friendship_id, Ok(_friendship_id))
    }
}
