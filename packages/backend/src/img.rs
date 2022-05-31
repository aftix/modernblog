use crate::{ImageSet, SQLite, sql, IMG_DIR};
use rocket::State;
use crate::util::{Ron, User};
use common::img::{ImageResponse, ImageRequest};
use rand::Rng;
use diesel::prelude::*;
use std::fs::{create_dir, write};
use std::path::Path;
use std::sync::RwLock;

#[options("/img/new")]
pub fn new_image_opt() -> &'static str {
    ""
}

#[post("/img/new", data = "<req>")]
pub async fn new_image(_user: User, imgset: &State<RwLock<ImageSet>>, conn: SQLite, req: ImageRequest) -> Ron<ImageResponse> {
    use crate::schema::images::dsl::*;

    let read_in = imgset.read().expect("Failed to get read lock").1;
    
    // Populate the ImageSet HashSet if it has not been done
    if !read_in {
        let imgs = conn.run(|c| images.filter(name.ne("")).load::<sql::Image>(c)).await;

        let mut writer = imgset.write().expect("Failed to get write lock");
        writer.1 = true;


        if let Ok(imgs) = imgs {
            imgs.into_iter().for_each(|img| {
                writer.0.insert(img.name);
            }); 
        }
    }

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890";
    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(5, 10);

    let img_uri = loop {
        let reader = imgset.read().expect("Failed to get read lock");
        let len = rng.sample(distr);
        let img_uri = random_string::generate(len, charset);
        let img_uri = format!("{}{}.{}", IMG_DIR, img_uri, req.extension);
        if !reader.0.contains(&img_uri) {
            break img_uri;
        }
    };

    {
        let mut writer = imgset.write().expect("Failed to get write lock");
        writer.0.insert(img_uri.clone());
    }

    if !Path::new("img/").exists() {
        create_dir("img/").expect("Couldn't create directory");
    }
    println!("{}", img_uri);
    let file = write(&img_uri, &req.data[..]);
    if file.is_ok() {
        Ron::new(ImageResponse::Success(img_uri))
    } else {
        println!("{:?}", file.err());
        Ron::new(ImageResponse::Failure)
    }
}

#[cfg(test)]
mod test {
    use crate::test::{login, setup};
    use rocket::http::{ContentType, Header};
    use common::img::{ImageRequest, ImageResponse};

    #[rocket::async_test]
    async fn new_img_opt() {
        let client = setup().await;
        let req = client.options("/api/img/new");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);
        assert_eq!(resp.into_string().await.expect("No body"), "");
    }

    #[rocket::async_test]
    async fn new_img_fails() {
        let client = setup().await;
        let req = client.post("/api/img/new");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 401);
    }

    #[rocket::async_test]
    async fn new_img_bad_request() {
        let client = setup().await;
        let (token, _) = login(&client).await;
        let mut req = client.post("/api/img/new");
        req.add_header(ContentType::new("application", "x-new-image"));
        req.add_header(Header::new("Authorization", format!("Bearer {}", token)));
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 500);
    }

    #[rocket::async_test]
    async fn new_img() {
        let client = setup().await;
        let (token, _) = login(&client).await;
        let mut req = client.post("/api/img/new");
        req.add_header(ContentType::new("application", "x-new-image"));
        req.add_header(Header::new("Authorization", format!("Bearer {}", token)));
        let req = req.body(ron::to_string(&ImageRequest {
            data: vec![],
            extension: "png".to_string(),
        }).expect("Failed to serialize"));

        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);

        let body = resp.into_string().await.expect("No body");

        let resp: Result<ImageResponse, _> = ron::from_str(&body);
        if let Ok(ImageResponse::Success(_)) = resp {

        } else {
            panic!("Bad Response");
        }
    }
}