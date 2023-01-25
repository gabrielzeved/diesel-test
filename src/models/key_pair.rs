use crate::schema::*;
use diesel::prelude::*;

use super::permission::Permission;

type AllColumns = (
    key_pairs::id,
    key_pairs::token,
    key_pairs::key,
    key_pairs::name,
    key_pairs::permission,
);

pub const ALL_COLUMNS: AllColumns = (
    key_pairs::id,
    key_pairs::token,
    key_pairs::key,
    key_pairs::name,
    key_pairs::permission,
);

type All = diesel::dsl::Select<key_pairs::table, AllColumns>;

type WithKey<'a> = diesel::dsl::Eq<key_pairs::key, &'a str>;
type WithToken<'a> = diesel::dsl::Eq<key_pairs::token, &'a str>;
type WithPair<'a> = diesel::dsl::And<WithKey<'a>, WithToken<'a>>;
type ByPair<'a> = diesel::dsl::Filter<All, WithPair<'a>>;

#[derive(Queryable, Selectable)]
pub struct KeyPair {
    pub id: i32,
    pub key: String,
    pub token: String,
    pub name: String,
    pub permission: Permission,
}

impl KeyPair {
    pub fn all() -> All {
      key_pairs::table.select(ALL_COLUMNS)
    }

    pub fn by_pair<'a>(key: &'a str, token: &'a str) -> ByPair<'a> {
        let predicate = key_pairs::key.eq(key).and(key_pairs::token.eq(token));
        KeyPair::all().filter(predicate)
    }

    // pub fn by_pair(key: &str, token: &str) -> ByPair<'_> {
    //   let predicate = key_pairs::key.eq(key).and(key_pairs::token.eq(token));
    //   KeyPair::all().filter(predicate)
    // }
}

#[derive(Insertable)]
#[diesel(table_name = key_pairs)]
pub struct NewKeyPair<'a> {
    pub name: &'a str,
    pub permission: &'a Permission,
}
