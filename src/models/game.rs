use crate::schema::games;
use crate::schema::games::dsl::*;
use crate::DbConn;
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

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "games"]
pub struct GameDTO {
    pub name: String,
    pub logo_url: Option<String>,
}

impl Game {
    pub async fn create(new_game: GameDTO, db: &DbConn) -> bool {
        db.run(move |conn| {
            diesel::insert_into(games)
                .values(&new_game)
                .execute(conn)
                .is_ok()
        })
        .await
    }

    pub async fn get_one(game_id: i32, db: &DbConn) -> Option<Game> {
        let game_found = db
            .run(move |conn| games.filter(id.eq(game_id)).get_result::<Game>(conn))
            .await;
        if game_found.is_err() {
            return None;
        }
        if let Ok(game) = game_found {
            return Some(game);
        }
        None
    }

    pub async fn get_list(db: &DbConn) -> Vec<Game> {
        let games_found = db.run(move |conn| games.get_results::<Game>(conn)).await;
        if games_found.is_err() {
            return Vec::new();
        }
        if let Ok(games_list) = games_found {
            return games_list;
        }
        Vec::new()
    }
}
