use backend::rocket;
use rocket::launch;

#[launch]
pub fn backend() -> _ {
    rocket()
}