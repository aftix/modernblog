#![no_main]
use libfuzzer_sys::fuzz_target;

use backend::rocket;
use rocket::local::blocking::Client;
use rocket::http::Header;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ROCKET: Mutex<Client> = {
        Mutex::new(Client::tracked(rocket()).expect("valid rocket instance"))
    };
}

fuzz_target!(|data: &[u8]| {
    let client = ROCKET.lock().expect("Failed to lock");
    let s = String::from_utf8_lossy(data).to_string();
    let req = client.post("/api/auth/renew");
    let req = req.header(Header::new("Authorization", s));
    let resp = req.dispatch();
    if resp.status().code != 401 && resp.status().code != 400 {
        panic!("failed response");
    }
});
