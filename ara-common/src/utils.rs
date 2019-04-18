use failure::Error;
use ring::digest;

#[allow(dead_code)]
pub fn rand_str(len: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    rng.sample_iter(&Alphanumeric).take(len).collect()
}

#[inline(always)]
pub fn new_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

#[allow(dead_code)]
pub fn sha512(input: &[u8]) -> Vec<u8> {
    let digest = digest::digest(&digest::SHA512, input);
    (&digest.as_ref()).to_vec()
}

#[allow(dead_code)]
pub fn sha256(input: &[u8]) -> Vec<u8> {
    let digest = digest::digest(&digest::SHA256, input);
    (&digest.as_ref()).to_vec()
}

pub fn sha256_hex(input: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA256, input);
    hex::encode(&digest.as_ref())
}

pub fn argon2_hash(password: &[u8], secret: &[u8]) -> Result<String, Error> {
    use argonautica::Hasher;
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_secret_key(secret)
        .hash()?; //does not implement std Error

    Ok(hash)
}

pub fn argon2_verify(password: &[u8], secret: &[u8], hash: &str) -> Result<bool, Error> {
    use argonautica::Verifier;

    let mut verifier = Verifier::default();
    let valid = verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(secret)
        .verify()?; //does not implement std Error

    Ok(valid)
}
