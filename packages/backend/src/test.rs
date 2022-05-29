use diesel::RunQueryDsl;
use rocket::local::asynchronous::Client;
use super::{rocket, SQLite};
use crate::sql::{NewImage, NewPost};
use diesel_migrations::embed_migrations;
use chrono::NaiveDateTime;
use std::time::{SystemTime, UNIX_EPOCH};
use common::auth::{AuthToken, Claim, LoginResponse};

embed_migrations!();


// Get a rocket instance for tests with mock db
#[doc(hidden)]
pub async fn setup() -> Client {
    use crate::schema::posts::dsl::*;
    use crate::schema::images::dsl::*;

    let client = Client::tracked(rocket()).await.expect("Valid rocket instance");

    {
        let sql = SQLite::get_one(client.rocket()).await.expect("Couldn't get a connection");
        sql.run(|c| {
            diesel::sql_query("PRAGMA foreign_keys = ON").execute(c).expect("Foregin Keys Failed");
            embedded_migrations::run(c).expect("Failed to run migrations");
        }).await;

        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time moved backwards");
        let stamp = NaiveDateTime::from_timestamp(now.as_secs() as i64, (now.as_nanos() - 1_000_000_000 * now.as_secs() as u128) as u32);
        let new_posts = vec![
            NewPost {
                title: "Post 1".to_string(),
                body: "This is post 1.".to_string(),
                draft: false,
                time: stamp.clone(),
                header: None,
            },
            NewPost {
                title: "Post 2".to_string(),
                body: "This is post 2.".to_string(),
                draft: true,
                time: stamp.clone(),
                header: None,
            },
            NewPost {
                title: "Post 3".to_string(),
                body: "This is post 3.".to_string(),
                draft: false,
                time: stamp.clone(),
                header: Some("abcd".to_string()),
            },
        ];
        let new_images = vec![
            NewImage {
                name: "abcd".to_string(),
                postid: 2,
            },
            NewImage {
                name: "aeu".to_string(),
                postid: 1,
            },
            NewImage {
                name: "aeuaeu".to_string(),
                postid: 3,
            },
        ];

        sql.run(move |c| {
            diesel::insert_into(posts).values(new_posts).execute(c).expect("failed to enter");
            diesel::insert_into(images).values(new_images).execute(c).expect("failed to enter images");
        }).await;
    }

    client
}

#[doc(hidden)]
pub async fn login(client: &Client) -> (AuthToken, Claim) {
    let req = client.post("/api/auth/login");
    let req = req.body(std::env::var("WRITER_PASSWORD").unwrap_or_else(|_| String::new()));
    let resp = req.dispatch().await;

    assert_eq!(resp.status().code, 200);

    let body = resp.into_string().await;
    assert!(body.is_some());

    let response: Result<LoginResponse, _> = ron::from_str(body.as_ref().unwrap());
    if let Ok(LoginResponse::Success(token, claim)) = response {
        (token, claim)
    } else {
        panic!("Couldn't log in");
    }
}