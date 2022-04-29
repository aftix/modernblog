#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

use rocket::{
    fairing::{Fairing, Info, Kind},
    response::Response,
    Data, Request,
};

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

#[database("blog")]
pub struct SQLite(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(SQLite::fairing())
        .mount("/api", routes![])
}
