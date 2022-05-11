use crate::schema::moods;
use crate::schema::moods::dsl::*;
use crate::DbConn;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "moods"]
pub struct Mood {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub begin_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "moods"]
pub struct MoodDTO {
    pub user_id: i32,
    pub game_id: i32,
    pub begin_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PartialMood {
    pub game_id: i32,
    pub begin_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>
}

impl Mood {
    pub async fn create(new_mood: MoodDTO, db: &DbConn) -> bool {
        db.run(move |conn| {
            diesel::insert_into(moods)
                .values(&new_mood)
                .execute(conn)
                .is_ok()
        })
        .await
    }

    pub async fn get_by_user_id(filter_user_id: i32, db: &DbConn) -> Vec<Mood> {
        println!("{:?}", filter_user_id);
        println!("ok");
        let moods_found = db.run(move |conn| moods.filter(user_id.eq(filter_user_id)).get_results::<Mood>(conn)).await;
        println!("Ok");
        if moods_found.is_err() {
            return Vec::new();
        }
        if let Ok(moods_list) = moods_found {
            println!("{:?}", moods_list);
            return moods_list;
        }
        Vec::new()
    }

    pub async fn get_by_user_ids(user_ids: Vec<i32>, db: &DbConn) -> Vec<Mood> {
        let moods_found = db.run(move |conn| moods.filter(user_id.eq_any(user_ids)).get_results::<Mood>(conn)).await;
        if moods_found.is_err() {
            return Vec::new();
        }
        if let Ok(moods_list) = moods_found {
            return moods_list;
        }
        Vec::new()
    }

    pub async fn get_by_game_ids_and_user_ids(game_ids: Vec<i32>, user_ids: Vec<i32>, db: &DbConn) -> Vec<Mood> {
        let moods_found = db.run(move |conn| moods.filter(user_id.eq_any(user_ids)).filter(game_id.eq_any(game_ids)).get_results::<Mood>(conn)).await;
        if moods_found.is_err() {
            return Vec::new();
        }
        if let Ok(moods_list) = moods_found {
            return moods_list;
        }
        Vec::new()
    }
}
