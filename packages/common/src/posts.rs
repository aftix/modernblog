#[cfg(feature = "guards")]
use rocket::{
    data::{FromData, Outcome, ToByteUnit},
    http::{ContentType, Status},
    Data, Request,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub images: Vec<String>,
    pub published: bool,
    pub header: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum NewPostResponse {
    Success(u32),
    Failure,
}

#[cfg(feature = "guards")]
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
            .unwrap_or_else(|| (100_000_usize).bytes());

        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, ())),
            Err(_) => return Failure((Status::InternalServerError, ())),
        };

        let ret = ron::de::from_str::<Post>(&string);
        if let Ok(ret) = ret {
            Success(ret)
        } else {
            Failure((Status::InternalServerError, ()))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub images: Option<Vec<String>>,
    pub published: bool,
    pub header: Option<String>,
}
