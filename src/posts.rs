#[cfg(feature = "gaurds")]
use rocket::http::{ContentType, Status};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Post {
    title: String,
    body: String,
    images: Vec<String>,
    published: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
pub enum NewPostResponse {
    Success(u32),
    Failure,
}

#[cfg(feature = "gaurds")]
#[rocket::async_trait]
impl<'r> FromData<'r> for Post {
    type Error = ();

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;

        let ct = ContentType::new("application", "x-new-post");
        if req.content_type() != Some(&ct) {
            return Forward(data);
        }

        let limit = req
            .limits()
            .get("x-new-post")
            .unwrap_or((100000 as usize).bytes());

        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, ())),
            Err(e) => return Failure((Status::InternalServerError, ())),
        };

        let ret = ron::de::from_str::<Post>(&string);
        if let Some(ret) = ret {
            Success(ret)
        } else {
            Failure((Status::InternalServerError, ()))
        }
    }
}
