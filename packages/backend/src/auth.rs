use crate::util::{Ron, User};
use crate::SessionSecret;
use common::auth::*;
use rocket::State;
use std::time::{SystemTime, UNIX_EPOCH};

#[options("/auth/login")]
pub async fn login_opt() -> &'static str {
    ""
}

#[post("/auth/login", data = "<req>")]
pub async fn login(secret: &State<SessionSecret>, req: String) -> Ron<LoginResponse> {
    let password = std::env::var("WRITER_PASSWORD");
    if password.is_err() {
        error!("WRITER_PASSWORD env variable not set");
        return Ron::new(LoginResponse::Failure);
    }

    if password.unwrap() != req {
        return Ron::new(LoginResponse::Failure);
    }

    let now = SystemTime::now();
    let now: u64 = now
        .duration_since(UNIX_EPOCH)
        .expect("shouldn't happen")
        .as_secs();
    let claim = Claim {
        exp: now + 5 * 60,
        iat: now,
    };
    let jwt = AuthToken::new(&claim, &secret.inner().0);
    if jwt.is_err() {
        return Ron::new(LoginResponse::Failure);
    }

    Ron::new(LoginResponse::Success(jwt.unwrap(), claim))
}

#[options("/auth/renew")]
pub async fn renew_opt() -> &'static str {
    ""
}

#[post("/auth/renew")]
pub async fn renew(secret: &State<SessionSecret>, _user: User) -> Ron<LoginResponse> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time moved backwards")
        .as_secs();
    let claim = Claim {
        exp: now + 5 * 60,
        iat: now,
    };
    let jwt = AuthToken::new(&claim, &secret.inner().0);
    if jwt.is_err() {
        return Ron::new(LoginResponse::Failure);
    }

    Ron::new(LoginResponse::Success(jwt.unwrap(), claim))
}
