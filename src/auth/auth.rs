extern crate chrono;
extern crate jsonwebtoken;
extern crate rocket;

const KEY: &[u8] = b"secret";

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub user: String,
}

impl Token {
    pub fn new(user: String) -> Self {
        Self { user }
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
impl<'r> rocket::request::FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, ()> {
        let token_header: Vec<_> = request.headers().get("token").collect();
        if token_header.len() == 1 {
            if let Some(token) = Self::decode(token_header[0].to_owned()) {
                return rocket::request::Outcome::Success(token);
            }
        }
        rocket::request::Outcome::Failure((rocket::http::Status::Forbidden, ()))
    }
}
