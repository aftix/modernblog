use crate::sql;
use crate::util::{Ron, User};
use crate::SQLite;
use chrono::NaiveDateTime;
use common::posts::{NewPostResponse, Post, PostResponse};
use diesel::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[options("/post/new")]
pub async fn newpost_opt() -> &'static str {
    ""
}

#[post("/post/new", data = "<req>")]
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

#[options("/post/get")]
pub fn get_opt() -> &'static str {
    ""
}

#[get("/post/get?<published>&<get_images>")]
pub async fn get(
    conn: SQLite,
    published: Option<bool>,
    get_images: Option<bool>,
) -> Ron<Option<Vec<PostResponse>>> {
    use crate::schema::images::dsl::*;
    use crate::schema::posts::dsl::*;

    let result = conn
        .run(move |c| {
            if published.unwrap_or_default() {
                posts.load::<sql::Post>(c)
            } else {
                posts.filter(draft.eq(false)).load::<sql::Post>(c)
            }
        })
        .await;

    if result.is_err() {
        return Ron::new(None);
    }

    let mut my_posts: Vec<_> = result
        .unwrap()
        .into_iter()
        .map(|post| PostResponse {
            id: post.id,
            title: post.title,
            published: !post.draft,
            images: None,
            header: post.header,
        })
        .collect();

    if get_images.unwrap_or_default() {
        for post in &mut my_posts {
            let my_id = post.id;
            let result = conn
                .run(move |c| images.filter(postid.eq(my_id)).load::<sql::Image>(c))
                .await;
            if let Ok(imgs) = result {
                post.images = Some(imgs.into_iter().map(|img| img.name).collect());
            }
        }
    }

    Ron::new(Some(my_posts))
}
