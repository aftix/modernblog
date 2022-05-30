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


#[cfg(test)]
mod test {
    use crate::test::{setup, login};
    use common::auth::LoginResponse;
    use rand::Rng;
    use rocket::http::Header;

    #[rocket::async_test]
    async fn login_opt() {
        let client = setup().await;
        let req = client.options("/api/auth/login");
        let resp = req.dispatch().await;

        assert_eq!(resp.status().code, 200);
        let body = resp.into_string().await.expect("No body");
        assert_eq!(body, "");
    }

    #[rocket::async_test]
    async fn login_fails() {
        let client = setup().await;

        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_!@#$%^&*()1234567890,'`\"";
        let mut rng = rand::thread_rng();
        let distr = rand::distributions::Uniform::new_inclusive(1, 100);

        for _ in 0..1000 {
            let num = rng.sample(distr);
            let pass = random_string::generate(num, charset);

            
            let req = client.post("/api/auth/login");
            let req = req.body(pass);
            let resp = req.dispatch().await;

            assert_eq!(resp.status().code, 200);

            let body = resp.into_string().await.expect("No body");
            let resp: Result<LoginResponse, _> = ron::from_str(&body);
            assert!(resp.is_ok());
            assert_eq!(resp.unwrap(), LoginResponse::Failure);
        }
    }

    #[rocket::async_test]
    async fn login_succeeds() {
        let client = setup().await;
        let req = client.post("/api/auth/login");

        let pass = std::env::var("WRITER_PASSWORD").expect("No variable set!");
        let req = req.body(pass);
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);
        
        let resp: Result<LoginResponse, _> = ron::from_str(&resp.into_string().await.expect("No body"));
        assert!(resp.is_ok());
        if let Ok(LoginResponse::Success(_, _)) = resp {

        } else {
            panic!("No response!");
        }
    }

    #[rocket::async_test]
    async fn renew_opt() {
        let client = setup().await;
        let req = client.options("/api/auth/renew");
        let resp = req.dispatch().await;
        assert_eq!(resp.status().code, 200);
        assert_eq!(resp.into_string().await.expect("No body"), "");
    }

    #[rocket::async_test]
    async fn renew_fails() {
        let client = setup().await;

        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_!@#$%^&*()1234567890,'`\"";
        let mut rng = rand::thread_rng();
        let distr = rand::distributions::Uniform::new_inclusive(1, 100);

        for _ in 0..1000 {
            let num = rng.sample(distr);
            let pass = random_string::generate(num, charset);

            
            let mut req = client.post("/api/auth/renew");
            req.add_header(Header::new("Authorization", format!("Bearer {}", pass)));
            let resp = req.dispatch().await;

            assert_eq!(resp.status().code, 401);
        }
    }

    #[rocket::async_test]
    async fn renew_succeeds() {
        let client = setup().await;
        let (token, _) = login(&client).await;

        let mut req = client.post("/api/auth/renew");
        req.add_header(Header::new("Authorization", format!("Bearer {}", token)));
        let resp = req.dispatch().await;

        assert_eq!(resp.status().code, 200);

        let body = resp.into_string().await.expect("No body");

        let resp: Result<LoginResponse, _> = ron::from_str(&body);

        assert!(resp.is_ok());

        if let Ok(LoginResponse::Success(_, _)) = resp {
        } else {
            panic!("Renew failed");
        }
    }

    #[rocket::async_test]
    async fn renew_succeeds_twice() {
        let client = setup().await;
        let (token, _) = login(&client).await;

        let mut req = client.post("/api/auth/renew");
        req.add_header(Header::new("Authorization", format!("Bearer {}", token)));
        let resp = req.dispatch().await;

        assert_eq!(resp.status().code, 200);

        let body = resp.into_string().await.expect("No body");

        let resp: Result<LoginResponse, _> = ron::from_str(&body);

        assert!(resp.is_ok());

        if let Ok(LoginResponse::Success(token, _)) = resp {
            let mut req = client.post("/api/auth/renew");
            req.add_header(Header::new("Authorization", format!("Bearer {}", token)));
            let resp = req.dispatch().await;

            assert_eq!(resp.status().code, 200);

            let body = resp.into_string().await.expect("No body");

            let resp: Result<LoginResponse, _> = ron::from_str(&body);
            assert!(resp.is_ok());

            if let Ok(LoginResponse::Success(_, _)) = resp {
            } else {
               panic!("Failed second time");
            }
        } else {
            panic!("Renew failed");
        }
    }
}