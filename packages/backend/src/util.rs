use crate::SessionSecret;
use core::fmt::Debug;
use rocket::{
    http::{ContentType, Status},
    request::{self, FromRequest},
    response::{Responder, Response, Result},
    Request,
};
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct Ron<T: Serialize + Debug> {
    inner: T,
}

impl<T: Serialize + Debug> Ron<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<'r, 'o: 'r, T: Serialize + Debug> Responder<'r, 'o> for Ron<T> {
    fn respond_to(self, _req: &'r Request<'_>) -> Result<'o> {
        let text = ron::to_string(&self.inner);

        if text.is_err() {
            return Err(Status::UnprocessableEntity);
        }
        let text = text.unwrap();
        Ok(Response::build()
            .header(ContentType::Plain)
            .sized_body(text.len(), Cursor::new(text))
            .finalize())
    }
}

pub struct User {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth = req.headers().get_one("Authorization");
        if auth.is_none() {
            return request::Outcome::Failure((Status::Unauthorized, ()));
        }
        let auth = auth.unwrap();

        let parts: Vec<&str> = auth.split(' ').collect();
        if parts.len() != 2 {
            return request::Outcome::Failure((Status::BadRequest, ()));
        }
        if parts[0] != "Bearer" {
            return request::Outcome::Failure((Status::BadRequest, ()));
        }

        let jwt = parts[1];
        let token = common::auth::AuthToken::from_jwt(jwt);
        let secret = req.rocket().state::<SessionSecret>();
        if secret.is_none() {
            return request::Outcome::Failure((Status::Unauthorized, ()));
        }
        let secret = secret.unwrap();
        let claim = token.authenticate(&secret.0);
        if claim.is_none() {
            return request::Outcome::Failure((Status::Unauthorized, ()));
        }

        request::Outcome::Success(Self {})
    }
}
