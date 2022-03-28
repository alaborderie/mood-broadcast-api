use crate::controllers::auth_controller::*;
use diesel::pg::PgConnection;
use rocket::fairing::AdHoc;
use rocket::Rocket;

embed_migrations!();

#[database("postgres_database")]
pub struct DbConn(PgConnection);

pub fn rocket() -> (Rocket, Option<DbConn>) {
    let rocket = rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", |rocket| {
            let conn = DbConn::get_one(&rocket).expect("database connection");
            match embedded_migrations::run(&*conn) {
                Ok(()) => Ok(rocket),
                Err(e) => {
                    error!("Failed to run database migrations: {:?}", e);
                    Err(rocket)
                }
            }
        }))
        .mount("/api/v1/auth", routes![login, signup]);

    let conn = if cfg!(test) {
        DbConn::get_one(&rocket)
    } else {
        None
    };

    (rocket, conn)
}
