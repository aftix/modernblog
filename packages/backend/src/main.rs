use backend::{auth, posts, SessionSecret, CORS, SQLite, rocket};
use rocket::{launch, routes};

#[launch]
pub fn backend() -> _ {
    rocket()
}