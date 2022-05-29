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

#[get("/post/get?<all>&<get_images>")]
pub async fn get(
    conn: SQLite,
    all: Option<bool>,
    get_images: Option<bool>,
) -> Ron<Option<Vec<PostResponse>>> {
    use crate::schema::images::dsl::*;
    use crate::schema::posts::dsl::*;

    let result = conn
        .run(move |c| {
            if all.unwrap_or_default() {
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

#[cfg(test)]
mod test {
    use common::posts::{NewPostResponse, PostResponse};
    use rocket::http::Header;

    // Make sure OPTIONS route passes for /api/post/get
    #[rocket::async_test]
    async fn get_posts_opt() {
        let client = crate::test::setup().await;
        let req = client.options("/api/post/get");
        let resp = req.dispatch().await; 
        assert_eq!(resp.status().code, 200);
        assert_eq!(resp.into_string().await.unwrap(), "");
    }

    // Test GET for /api/post/get
    #[rocket::async_test]
    async fn get_posts() {
        let client = crate::test::setup().await;

        // Getting the default should return 2 published posts, no images
        let req = client.get("/api/post/get");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: None,
        });
        assert_eq!(response[1], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: None,
        });

        // Getting all posts should return 3 posts
        let req = client.get("/api/post/get?all");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 3);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: None,
        });
        assert_eq!(response[1], PostResponse {
            id: 2,
            title: "Post 2".to_string(),
            published: false,
            header: None,
            images: None,
        });
        assert_eq!(response[2], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: None,
        });

        // Getting all posts should return 3 posts
        let req = client.get("/api/post/get?all=true");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 3);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: None,
        });
        assert_eq!(response[1], PostResponse {
            id: 2,
            title: "Post 2".to_string(),
            published: false,
            header: None,
            images: None,
        });
        assert_eq!(response[2], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: None,
        });

        // Specifying all to be false should work
        let req = client.get("/api/post/get?all=false");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: None,
        });
        assert_eq!(response[1], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: None,
        });

        // Specifying all to be invalid should work
        let req = client.get("/api/post/get?all=googoogaga");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: None,
        });
        assert_eq!(response[1], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: None,
        });
    }

    // Test GET for /api/post/get?get_images
    #[rocket::async_test]
    async fn get_posts_and_images() {
        let client = crate::test::setup().await;

        // Getting the default should return 2 published posts, no images
        let req = client.get("/api/post/get?get_images");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: Some(vec!["aeu".to_string()]),
        });
        assert_eq!(response[1], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: Some(vec!["aeuaeu".to_string()]),
        });

        // Getting all posts should return 3 posts
        let req = client.get("/api/post/get?all&get_images");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 3);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: Some(vec!["aeu".to_string()]),
        });
        assert_eq!(response[1], PostResponse {
            id: 2,
            title: "Post 2".to_string(),
            published: false,
            header: None,
            images: Some(vec!["abcd".to_string()]),
        });
        assert_eq!(response[2], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: Some(vec!["aeuaeu".to_string()]),
        });

        // Getting all posts should return 3 posts
        let req = client.get("/api/post/get?all=true&get_images");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 3);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: Some(vec!["aeu".to_string()]),
        });
        assert_eq!(response[1], PostResponse {
            id: 2,
            title: "Post 2".to_string(),
            published: false,
            header: None,
            images: Some(vec!["abcd".to_string()]),
        });
        assert_eq!(response[2], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: Some(vec!["aeuaeu".to_string()]),
        });

        // Specifying all to be false should work
        let req = client.get("/api/post/get?all=false&get_images");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: Some(vec!["aeu".to_string()]),
        });
        assert_eq!(response[1], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: Some(vec!["aeuaeu".to_string()]),
        });

        // Specifying all to be invalid should work
        let req = client.get("/api/post/get?all=googoogaga&get_images");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let text = resp.into_string().await.expect("No body");

        let response: Option<Vec<PostResponse>> = ron::from_str(&text).expect("Can't deserialize");
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], PostResponse {
            id: 1,
            title: "Post 1".to_string(),
            published: true,
            header: None,
            images: Some(vec!["aeu".to_string()]),
        });
        assert_eq!(response[1], PostResponse {
            id: 3,
            title: "Post 3".to_string(),
            published: true,
            header: Some("abcd".to_string()),
            images: Some(vec!["aeuaeu".to_string()]),
        });
    }

    // Test OPTIONS /api/post/new
    #[rocket::async_test]
    async fn new_post_opt() {
        let client = crate::test::setup().await;
        let req = client.options("/api/post/new");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);
        assert_eq!(resp.into_string().await.unwrap(), "");
    }

    // Test that you need a user token to make a new post
    #[rocket::async_test]
    async fn new_post_unauth() {
        let client = crate::test::setup().await;
        let req = client.post("/api/post/new");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 401);
    }

    // Test sending invalid new post
    #[rocket::async_test]
    async fn new_post_invalid() {
        let client = crate::test::setup().await;
        let (token, _) = crate::test::login(&client).await;

        let mut req = client.post("/api/post/new"); 
        let header_content = format!("Bearer {}", token);
        req.add_header(Header::new("Authorization", header_content));
        let resp = req.dispatch().await;

        assert_eq!(resp.status().code, 404);
    }

    // TODO: Test adding new posts

    // TODO: Fuzz inputs
}