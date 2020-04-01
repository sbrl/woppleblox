use argonautica::{ Hasher, Verifier };

use crate::db::User;

fn password_hash(secret: &str, password: &str) -> String {
    let mut hash = Hasher::default()
        .with_password(password)
        .with_secret_key(secret)
        .hash()
        .unwrap();
    
    hash
}

fn password_verify(secret: &str, hash: &str, password: &str) -> bool {
    Verifier::default()
        .with_secret_key(secret)
        .with_hash(hash)
        .with_password(password)
        .verify()
        .unwrap()
}

pub trait HasPassword {
    fn password_update(&mut self, new_password: &str);
    fn password_verify(&self, password: &str) -> bool;
}

impl HasPassword for User {
    fn password_update(&mut self, secret: &str, new_password: &str) {
        self.password = password_hash(secret, new_password);
    }
    
    fn password_verify(&self, secret: &str, password: &str) -> bool {
        password_verify(secret, &self.password, password)
    }
}
