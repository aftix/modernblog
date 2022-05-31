
#![allow(clippy::extra_unused_lifetimes)]

include!(concat!(env!("OUT_DIR"), "/api.rs"));

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[cfg(any(test,fuzzing))]
#[macro_use]
extern crate diesel_migrations;

use rocket::{
    fairing::{Fairing, Info, Kind},
    response::Response,
    Data, Request, Rocket, Build
};
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::sync::RwLock;

pub mod auth;
pub mod posts;
pub mod img;
mod schema;
mod sql;
mod util;
#[doc(hidden)]
#[cfg(any(test, fuzzing))]
pub mod test;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Header",
            kind: Kind::Response,
        }
    }

    async fn on_request(&self, _: &mut Request<'_>, _: &mut Data<'_>) {}

    async fn on_response<'r>(&self, _: &'r Request<'_>, resp: &mut Response<'r>) {
        resp.set_raw_header("Access-Control-Allow-Origin", FRONTEND_PATH);
        resp.set_raw_header(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        );
        resp.set_raw_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
    }
}

pub struct SessionSecret(pub String);
pub struct ImageSet(HashSet<String>, bool);

#[cfg(not(any(test, fuzzing)))]
#[database("blog")]
pub struct SQLite(pub diesel::SqliteConnection);

#[cfg(any(test, fuzzing))]
#[database("mock_db")]
pub struct SQLite(pub diesel::SqliteConnection);

pub fn rocket() -> Rocket<Build> {
    dotenvy::dotenv().ok();

    let mut rng = rand_chacha::ChaChaRng::from_entropy();
    let secret = SessionSecret(Rng::gen::<u128>(&mut rng).to_string());
    let images = RwLock::new(ImageSet(HashSet::new(), false));
    rocket::build()
        .manage(secret)
        .manage(images)
        .attach(CORS)
        .attach(SQLite::fairing())
        .mount(
            "/api",
            routes![
                auth::login_opt,
                auth::login,
                auth::renew_opt,
                auth::renew,
                posts::newpost_opt,
                posts::newpost,
                posts::get_opt,
                posts::get,
                img::new_image_opt,
                img::new_image,
            ],
        )
}