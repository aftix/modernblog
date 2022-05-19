#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate diesel;

use rand::{Rng, SeedableRng};
use rocket::{
    fairing::{Fairing, Info, Kind},
    response::Response,
    Data, Request,
};

pub mod auth;
pub mod util;

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
        resp.set_raw_header("Access-Control-Allow-Origin", "*");
        resp.set_raw_header(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        );
        resp.set_raw_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
    }
}

pub struct SessionSecret(String);

#[database("blog")]
pub struct SQLite(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    let mut rng = rand_chacha::ChaChaRng::from_entropy();
    let secret: SessionSecret = SessionSecret(Rng::gen::<u128>(&mut rng).to_string());
    rocket::build()
        .manage(secret)
        .attach(CORS)
        .attach(SQLite::fairing())
        .mount(
            "/api",
            routes![auth::login_opt, auth::login, auth::renew_opt, auth::renew],
        )
}
