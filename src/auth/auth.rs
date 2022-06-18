use crate::models::UserModel;

extern crate chrono;
extern crate jsonwebtoken;
extern crate rocket;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

const KEY: &[u8] = b"secret";

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub user: String,
    end_time: Option<i64>,
}

impl Token {
    pub fn new(user: String) -> Self {
        Self { user, end_time: None }
    }

    pub fn is_guest(&self) -> bool {
        self.end_time.is_some()
    }

    pub fn is_active_guest(&self) -> bool {
        self.is_guest() && self.end_time.unwrap() > chrono::Utc::now().timestamp()
    }

    pub fn refresh(&mut self) {
        if self.is_guest() {
            self.set_time();
        }
    }

    pub fn set_time(&mut self) -> &mut Self {
        self.end_time = Some(chrono::Utc::now().timestamp() + 1800);
        return self;
    }

    pub fn encode(self) -> String {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(KEY),
        );
        token.unwrap()
    }

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
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let token_header: Vec<_> = request.headers().get("token").collect();
        if token_header.len() == 1 {
            if let Some(token) = Self::decode(token_header[0].to_owned()) {
                if token.is_guest() && token.is_active_guest() {
                    return Outcome::Success(token);
                } else {
                    use super::models::DBLink;
                    let conn = request.rocket().state::<DBLink>();
                    if conn.is_none() {
                        return Outcome::Failure((Status::ServiceUnavailable, ()));
                    }
                    if UserModel::is_active(conn.unwrap(), &token.user) {
                        return Outcome::Success(token);
                    }
                }
            }
        }
        rocket::request::Outcome::Failure((Status::Forbidden, ()))
    }
}
