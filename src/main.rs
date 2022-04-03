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
}
