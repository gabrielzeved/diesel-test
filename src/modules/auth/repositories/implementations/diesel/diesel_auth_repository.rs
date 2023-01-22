use cms::establish_connection;
use diesel::prelude::*;

use crate::modules::auth::{
  models::key_pair::KeyPair,
  repositories::auth_repository::AuthRepository
};

pub struct DieselAuthRepository {}

impl AuthRepository for DieselAuthRepository {
  fn read(&self) -> Vec<KeyPair> {
    use crate::schema::key_pairs::dsl::*;
  
    let connection = &mut establish_connection();  
    let results = key_pairs
    .filter(name.eq("ci-cd"))
    .load::<KeyPair>(connection)
    .expect("Error finding key pair");
    
    return results;
  } 
}
