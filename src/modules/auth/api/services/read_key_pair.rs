use crate::modules::auth::repositories::auth_repository::AuthRepository;

pub fn read_key_pair(repository: &impl AuthRepository) {
  repository.read();
}