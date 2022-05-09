extern crate chrono;
extern crate jsonwebtoken;
extern crate rocket;

const TOKEN_LIFE: i64 = 900;
const KEY: &[u8] = b"secret";

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    user: String,
    timestamp: i64,
}

impl Token {
    pub fn new(user: String) -> Self {
        Self {
            user,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn is_active(&self) -> bool {
        if chrono::Utc::now().timestamp() - TOKEN_LIFE > self.timestamp {
            return true;
        }
        false
    }

    pub fn encode(self) -> String {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(KEY),
        );
        token.unwrap()
    }

    pub fn refresh(mut self) -> String {
        self.timestamp = chrono::Utc::now().timestamp();
        self.encode()
    }

    pub fn decode(token: String) -> Self {
        jsonwebtoken::decode::<Self>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(KEY),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .unwrap()
        .claims
    }
}
