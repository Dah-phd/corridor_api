use crate::models::users::User;
extern crate chrono;
extern crate jsonwebtoken;
extern crate rocket;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

const KEY: &[u8] = b"secret";
pub const TOKEN_ID: &str = "gamertag";

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize, Debug)]
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

    pub fn encode(self, header: &jsonwebtoken::Header) -> String {
        let token = jsonwebtoken::encode(header, &self, &jsonwebtoken::EncodingKey::from_secret(KEY));
        match token {
            Ok(jwt) => jwt,
            _ => "".to_owned(),
        }
    }

    pub fn decode(token: String, validator: &jsonwebtoken::Validation) -> Option<Self> {
        match jsonwebtoken::decode::<Self>(&token, &jsonwebtoken::DecodingKey::from_secret(KEY), validator) {
            Ok(v) => Some(v.claims),
            _ => None,
        }
    }
}

pub struct AuthTokenServices {
    pub header: jsonwebtoken::Header,
    pub validator: jsonwebtoken::Validation,
}

impl AuthTokenServices {
    pub fn new() -> Self {
        let mut validator = jsonwebtoken::Validation::default();
        validator.required_spec_claims.clear();
        validator.validate_exp = false;
        Self {
            header: jsonwebtoken::Header::default(),
            validator,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        if let Some(maybe_gamertag) = request.cookies().get(TOKEN_ID) {
            let gamertag = maybe_gamertag.value();
            use super::models::DBLink;
            let token_services = request.rocket().state::<AuthTokenServices>();
            let conn = request.rocket().state::<DBLink>();
            if token_services.is_none() || conn.is_none() {
                return Outcome::Failure((Status::ServiceUnavailable, ()));
            }
            if let Some(token) = Self::decode(gamertag.to_owned(), &token_services.unwrap().validator) {
                if token.is_active_guest() || User::is_active(conn.unwrap(), &token.user) {
                    return Outcome::Success(token);
                }
            }
        }
        Outcome::Failure((Status::Forbidden, ()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn make_token() {
        let token_service = AuthTokenServices::new();
        let name = "ALFA";
        let token = Token::new(name.to_owned());
        let code = token.encode(&token_service.header);
        let decoded = Token::decode(code, &token_service.validator).unwrap();
        assert_eq!(decoded.user, name);
    }
}
