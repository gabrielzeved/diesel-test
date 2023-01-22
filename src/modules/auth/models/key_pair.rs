
use diesel::prelude::*;

use super::permission::Permission;
#[derive(Queryable)]
pub struct KeyPair {
  pub id: i32,
  pub key: String,
  pub token: String,
  pub name: String,
  pub permission: Permission
}