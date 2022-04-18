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
}

#[cfg(test)]
mod integration_test {
    use super::rocket;
    use crate::models::response::Response;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;

    #[test]
    fn subscribe_and_login() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
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

    #[test]
    fn game_routes() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/api/v1/games")
            .json(&json!({"name": "League of Legends"}))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        response = client.get("/api/v1/games").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_json::<Response>(),
            Some(Response {
                message: String::from("Ok"),
                data: json!([{"id": 1, "logo_url": "", "name": "League of Legends"}])
            })
        );
        response = client.get("/api/v1/games/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_json::<Response>(),
            Some(Response {
                message: String::from("Ok"),
                data: json!({"id": 1, "logo_url": "", "name": "League of Legends"})
            })
        );
    }
}
