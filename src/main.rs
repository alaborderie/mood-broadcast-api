#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate uuid;

pub mod controllers;
pub mod jwt;
pub mod models;
pub mod schema;
pub mod services;

use crate::controllers::auth_controller::*;
use crate::controllers::game_controller::*;
use crate::controllers::mood_controller::*;
use diesel_migrations::embed_migrations;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

#[database("postgres_database")]
pub struct DbConn(diesel::PgConnection);

embed_migrations!();

async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db_conn = DbConn::get_one(&rocket).await.expect("database connection");
    db_conn
        .run(|conn| match embedded_migrations::run(&*conn) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                error!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::try_on_ignite("Database Migrations", migrate))
        .mount("/api/v1/auth", routes![login, signup])
        .mount(
            "/api/v1/games",
            routes![create_game, find_game_by_id, list_game],
        )
        .mount(
            "/api/v1/moods",
            routes![create_mood, list_user_moods, get_moods_by_friend_id],
        )
}

#[cfg(test)]
mod integration_test {
    use super::rocket;
    use crate::models::response::Response;
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;

    fn get_jwt(client: &Client) -> String {
        client
            .post("/api/v1/auth/signup")
            .json(&json!({ "username": "test", "email": "user@test.com", "password": "user_test"}))
            .dispatch();
        let response = client
            .post("/api/v1/auth/login")
            .json(&json!({ "username_or_email": "test", "password": "user_test" }))
            .dispatch();
        return String::from(
            response
                .into_json::<Response>()
                .unwrap()
                .data
                .get("token")
                .unwrap()
                .to_string()
                .replace("\"", ""),
        );
    }

    #[test]
    fn run_integration_tests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        subscribe_and_login(&client);
        let jwt = get_jwt(&client);
        game_routes(&client, String::from(&jwt));
        // TODO: friend_routes(&client, String::from(&jwt));
        mood_routes(&client, String::from(&jwt));
    }

    fn subscribe_and_login(client: &Client) {
        let mut response = client.post("/api/v1/auth/signup").json(&json!({ "username": "alaborderie", "email": "antoine.laborderie@gmail.com", "password": "PassW0rd!" })).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "{\"message\":\"Signup was successfull.\",\"data\":{}}"
        );
        response = client.post("/api/v1/auth/login").json(&json!({ "username_or_email": "antoine.laborderie@gmail.com", "password": "PassW0rd!" })).dispatch();
        assert_eq!(response.status(), Status::Ok);
        response = client
            .post("/api/v1/auth/login")
            .json(&json!({ "username_or_email": "failed@test.com", "password": "TotalFailur3!" }))
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    fn game_routes(client: &Client, jwt: String) {
        let mut response = client
            .post("/api/v1/games")
            .json(&json!({"name": "League of Legends"}))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        response = client
            .post("/api/v1/games")
            .header(Header::new("Authorization", format!("Bearer {}", jwt)))
            .json(&json!({"name": "League of Legends"}))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        response = client.get("/api/v1/games").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        response = client
            .get("/api/v1/games")
            .header(Header::new("Authorization", format!("Bearer {}", jwt)))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response
                .into_json::<Response>()
                .unwrap()
                .data
                .as_array()
                .unwrap()
                .first(),
            Some(&json!({"id": 1, "logo_url": "", "name": "League of Legends"}))
        );
        response = client.get("/api/v1/games/1").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        response = client
            .get("/api/v1/games/1")
            .header(Header::new("Authorization", format!("Bearer {}", jwt)))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_json::<Response>(),
            Some(Response {
                message: String::from("Ok"),
                data: json!({"id": 1, "logo_url": "", "name": "League of Legends"})
            })
        );
    }

    fn mood_routes(client: &Client, jwt: String) {
        let mut response = client
            .post("/api/v1/moods")
            .json(&json!({"game_id": 1, "begin_timestamp": "1970-01-01T00:00:00Z", "end_timestamp": "1970-01-01T00:00:00Z"}))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        response = client
            .post("/api/v1/moods")
            .header(Header::new("Authorization", format!("Bearer {}", jwt)))
            .json(&json!({"game_id": 1, "begin_timestamp": "1970-01-01T00:00:00Z", "end_timestamp": "1970-01-01T00:00:00Z"}))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        response = client.get("/api/v1/moods").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        response = client
            .get("/api/v1/moods")
            .header(Header::new("Authorization", format!("Bearer {}", jwt)))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response
                .into_json::<Response>()
                .unwrap()
                .data
                .as_array()
                .unwrap()
                .first(),
            Some(
                &json!({"id": 1, "game_id": 1, "begin_timestamp": "1970-01-01T00:00:00Z", "end_timestamp": "1970-01-01T00:00:00Z", "user_id": 2})
            )
        );
        response = client.get("/api/v1/moods/friends/1").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        response = client
            .get("/api/v1/moods/friends/1")
            .header(Header::new("Authorization", format!("Bearer {}", jwt)))
            .dispatch();
        assert_eq!(response.status(), Status::Forbidden);
        // TODO: Uncomment
        /*
        response = client
            .get("/api/v1/moods/friends/2")
            .header(Header::new("Authorization", format!("Bearer {}", jwt)))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        */
    }
}
