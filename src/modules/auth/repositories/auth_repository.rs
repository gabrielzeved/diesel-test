use crate::modules::auth::models::key_pair::KeyPair;

pub trait AuthRepository {
  fn read(&self) -> Vec<KeyPair>;
}
