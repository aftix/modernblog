use jsonwebtoken::{
    decode, encode, errors::Result, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use std::fmt::{self, Display, Formatter};

// Claim for JWT
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Claim {
    pub iat: u64,
    pub exp: u64,
}

// Session Token, JWT
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthToken {
    jwt: String,
}

impl Display for AuthToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.jwt)
    }
}

impl AuthToken {
    pub fn new(claim: &Claim, secret: &str) -> Result<Self> {
        let jwt = encode(
            &Header::new(Algorithm::HS512),
            claim,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        Ok(Self { jwt })
    }

    pub fn from_jwt(jwt: &str) -> Self {
        Self {
            jwt: jwt.to_owned(),
        }
    }

    pub fn authenticate(self, secret: &str) -> Option<Claim> {
        let claim = decode::<Claim>(
            &self.jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS512),
        );

        if let Ok(claim) = claim {
            Some(claim.claims)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum LoginResponse {
    Success(AuthToken, Claim),
    Failure,
}
