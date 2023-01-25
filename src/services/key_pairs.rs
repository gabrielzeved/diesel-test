use std::error::Error;

use cms::establish_connection;

use crate::gql::key_pairs_schema::{AllKeyPairInput, FindByPairInput};
use crate::models::key_pair::*;
use crate::utils::pagination::Paginate;

pub fn all(input: AllKeyPairInput) -> Result<(Vec<KeyPair>, i32), Box<dyn Error>> {
    match input {
        AllKeyPairInput { pagination } => {
            let connection = &mut establish_connection();

            let mut query = KeyPair::all().paginate(pagination.page as i64);

            if let Some(per_page) = pagination.per_page {
                use std::cmp::min;
                query = query.per_page(min(per_page as i64, 25));
            }

            let (values, total_pages) = query.load_and_count_pages::<KeyPair>(connection)?;
            Ok((values, total_pages as i32))
        }
    }
}

pub fn find_by_pair(input: FindByPairInput) -> Result<(Vec<KeyPair>, i32), Box<dyn Error>> {
    match input {
        FindByPairInput {
            key,
            token,
            pagination,
        } => {
            let connection = &mut establish_connection();

            let mut query = KeyPair::by_pair(&key, &token).paginate(pagination.page as i64);

            if let Some(per_page) = pagination.per_page {
                use std::cmp::min;
                query = query.per_page(min(per_page as i64, 25));
            }

            let (values, total_pages) = query.load_and_count_pages::<KeyPair>(connection)?;
            Ok((values, total_pages as i32))
        }
    }
}
