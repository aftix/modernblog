use crate::sql;
use crate::util::{Ron, User};
use crate::SQLite;
use aftblog_common::posts::{NewPostResponse, Post};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[options("/post/new")]
pub async fn newpost_opt() -> &'static str {
    ""
}

#[post("/auth/new", data = "<req>")]
pub async fn newpost(_user: User, conn: SQLite, req: Post) -> Ron<NewPostResponse> {
    use crate::schema::images::dsl::*;
    use crate::schema::posts::dsl::*;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time moved backwards");
    let new_post = sql::NewPost {
        title: String::from(req.title()),
        body: String::from(req.body()),
        draft: !req.published(),
        time: NaiveDateTime::from_timestamp(
            now.as_secs() as i64,
            (now.as_nanos() - 1_000_000_000 * now.as_secs() as u128) as u32,
        ),
        header: req.header(),
    };

    let my_title = String::from(req.title());

    let result = conn
        .run(move |c| diesel::insert_into(posts).values(&new_post).execute(c))
        .await;

    if result.is_err() {
        return Ron::new(NewPostResponse::Failure);
    }

    let result = conn
        .run(move |c| posts.filter(title.eq(my_title)).load::<sql::Post>(c))
        .await;

    if let Ok(result) = result {
        if result.is_empty() {
            Ron::new(NewPostResponse::Failure)
        } else {
            for image in req.images() {
                let new_image = sql::NewImage {
                    name: image.clone(),
                    postid: result[0].id,
                };
                conn.run(move |c| diesel::insert_into(images).values(&new_image).execute(c))
                    .await
                    .unwrap();
            }
            Ron::new(NewPostResponse::Success(result[0].id as u32))
        }
    } else {
        Ron::new(NewPostResponse::Failure)
    }
}
