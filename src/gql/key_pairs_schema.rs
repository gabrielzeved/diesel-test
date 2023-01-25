use juniper::{EmptySubscription, FieldResult, GraphQLInputObject, GraphQLObject, RootNode};

#[derive(GraphQLInputObject)]
#[graphql(description = "Struct used for any kind of pagination ( page starts from 0 )")]
pub struct Pagination {
    pub page: i32,
    pub per_page: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Input for querying all key pairs")]
pub struct AllKeyPairInput {
    pub pagination: Pagination,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "")]
pub struct FindByPairInput {
    pub key: String,
    pub token: String,
    pub pagination: Pagination,
}

#[derive(GraphQLObject)]
#[graphql(description = "A key pair values")]
pub struct KeyPair {
    pub key: String,
    pub token: String,
    pub name: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "List of key pairs")]
pub struct KeyPairList {
    pub data: Vec<KeyPair>,
    pub total_pages: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn find_all_key_pairs(all_key_pair: AllKeyPairInput) -> FieldResult<KeyPairList> {
        use crate::services::key_pairs::all;

        let result = all(all_key_pair).expect("");

        let data = result
            .0
            .iter()
            .map(|x| KeyPair {
                key: x.key.to_string(),
                name: x.name.to_string(),
                token: x.token.to_string(),
            })
            .collect();

        Ok(KeyPairList {
            data,
            total_pages: result.1,
        })
    }

    fn find_by_pair_key_pair(by_pair: FindByPairInput) -> FieldResult<KeyPairList> {
        use crate::services::key_pairs::find_by_pair;

        let result = find_by_pair(by_pair).expect("");

        let data = result
            .0
            .iter()
            .map(|x| KeyPair {
                key: x.key.to_string(),
                name: x.name.to_string(),
                token: x.token.to_string(),
            })
            .collect();

        Ok(KeyPairList {
            data,
            total_pages: result.1,
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn health_check() -> FieldResult<bool> {
        Ok(true)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
