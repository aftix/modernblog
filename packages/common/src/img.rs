#[cfg(feature = "guards")]
use rocket::{
    data::{FromData, Outcome, ToByteUnit},
    http::{ContentType, Status},
    Data, Request,
};

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ImageRequest {
    pub data: Vec<u8>,
    pub extension: String,
}

#[cfg(feature = "guards")]
#[rocket::async_trait]
impl<'r> FromData<'r> for ImageRequest {
    type Error = String;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;

        let ct = ContentType::new("application", "x-new-image");
        if req.content_type() != Some(&ct) {
            return Forward(data);
        }

        let limit = req
            .limits()
            .get("x-new-image")
            .unwrap_or_else(|| (4_000_000_usize).bytes());

        let bytes = match data.open(limit).into_bytes().await {
            Ok(bytes) if bytes.is_complete() => bytes.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, String::new())),
            Err(err) => return Failure((Status::InternalServerError, err.to_string())),
        };

        let ret = bson::from_slice(&bytes[..]);
        if let Ok(ret) = ret {
            Success(ret)
        } else {
            Failure((Status::InternalServerError, ret.err().unwrap().to_string()))
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ImageResponse {
    Success(String),
    Failure,
}
