use chrono::Utc;
use hex;
use ring::hmac;

pub fn millis() -> u64 {
    Utc::now().timestamp_millis() as u64
}

pub fn sign(secret: &str, msg: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&key, msg.as_bytes());
    hex::encode(tag.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_millseconds() {
        assert!(millis() > 0);
    }

    #[test]
    fn test_sign() {
        assert_eq!(
            sign("secret", "message"),
            String::from("8b5f48702995c1598c573db1e21866a9b825d4a794d169d7060a03605796360b")
        );
    }
}
