use bcrypt::{hash, DEFAULT_COST};
use magic_crypt::{MagicCrypt, new_magic_crypt};

struct Users {
    db: sled::Db,
}

impl Users {
    pub fn init() -> Result<Users, sled::Error> {
        let db = sled::open("users")?;
        Ok(Self { db })
    }

    pub fn get_token(&self, username: &str, password: &str) -> Option<String> {
        if self.verify_user(username, password) {
            Self::tokenize(username, password)
        } else {
            None
        }
    }

    fn tokenize(username: &str, password: &str) -> Option<String> {
        todo!()
    }

    pub fn new_user(&self, username: &str, password: &str) -> Option<String> {
        if let Ok(user_exists) = self.db.contains_key(username) {
            if user_exists {
                return None;
            }
            if let Ok(hashed) = hash(password, DEFAULT_COST) {
                if let Ok(maybe_record) = self.db.insert(username, hashed.as_bytes()) {
                    if maybe_record.is_some() {
                        return Self::tokenize(username, password);
                    }
                }
            }
        }
        None
    }

    pub fn verify_user(&self, username: &str, password: &str) -> bool {
        todo!()
    }
}
