use crate::schema::friends;
use crate::schema::friends::dsl::*;
use crate::DbConn;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "friends"]
pub struct Friend {
    pub id: i32,
    pub user_from_id: i32,
    pub user_to_id: i32,
    pub status: String,
    pub update_timestamp: DateTime<Utc>
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "friends"]
pub struct FriendDTO {
    pub user_from_id: i32,
    pub user_to_id: i32,
    pub status: String,
    pub update_timestamp: DateTime<Utc>
}

impl Friend {

}
