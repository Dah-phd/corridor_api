extern crate chrono;
extern crate jsonwebtoken;
extern crate rocket;

const KEY: &[u8] = b"secret";

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub user: String,
    // timestamp: i64,
}

impl Token {
    pub fn new(user: String) -> Self {
        Self {
            user,
            // timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn is_active(&self) -> bool {
        return true;
        // if chrono::Utc::now().timestamp() - TOKEN_LIFE > self.timestamp {
        //     return true;
        // }
        // false
    }

    pub fn encode(self) -> String {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(KEY),
        );
        token.unwrap()
    }

    // pub fn refresh(mut self) -> String {
    //     self.timestamp = chrono::Utc::now().timestamp();
    //     self.encode()
    // }

    pub fn decode(token: String) -> Option<Self> {
        match jsonwebtoken::decode::<Self>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(KEY),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
        ) {
            Ok(v) => Some(v.claims),
            _ => None,
        }
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, ()> {
        let token_header: Vec<_> = request.headers().get("token").collect();
        if token_header.len() != 1 {
            return rocket::request::Outcome::Failure((rocket::http::Status::BadRequest, ()));
        }
        let token = token_header[0];
        let decoded_token = Self::decode(token.to_owned());
        match decoded_token {
            Some(v) => rocket::request::Outcome::Success(v),
            None => rocket::request::Outcome::Failure((rocket::http::Status::BadRequest, ())),
        }
    }
}
