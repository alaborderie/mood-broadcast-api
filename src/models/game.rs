use crate::schema::games;
use crate::schema::games::dsl::*;
use crate::DbConn;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "games"]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub logo_url: String,
}

#[derive(Insertable)]
#[table_name = "games"]
pub struct GameDTO {
    pub name: String,
    pub logo_url: String,
}

impl Game {
    pub async fn create(game_name: Option<String>, logo_url: Option<String>, db: &DbConn) -> GameDTO {
        let new_game = GameDTO {
            user_id: user.id,
            login_session: String::from(login_session_str),
            login_timestamp: Utc::now(),
        };
        db.run(move |conn| {
            diesel::insert_into(game)
                .values(&new_game)
                .execute(conn)
        })
        .await
    }
}
