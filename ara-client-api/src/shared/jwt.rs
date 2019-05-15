use ara_common::utils::new_uuid;
use ara_model::core::User;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtClaim {
    exp: DateTime<Utc>,
    iat: DateTime<Utc>,
    jti: String,
    first_name: String,
    last_name: String,
    email: String,
}

pub(crate) fn new_token(user: &User) -> Result<String, failure::Error> {
    let expire_date = Utc::now() + Duration::days(30);
    let claim = JwtClaim {
        exp: expire_date,
        iat: Utc::now(),
        jti: new_uuid(),
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        email: user.email.clone(),
    };
    encode_jwt(&claim)
}

fn encode_jwt(claim: &JwtClaim) -> Result<String, failure::Error> {
    let key = "secret";

    let header = Header::default();

    let token = encode(&header, &claim, key.as_ref())?;

    Ok(token)
}

#[allow(dead_code)]
pub(crate) fn decode_jwt(token: &str) -> Result<JwtClaim, failure::Error> {
    let key = "secret";

    let token_data = decode::<JwtClaim>(&token, key.as_ref(), &Validation::new(Algorithm::HS256))?;
    Ok(token_data.claims)
}
