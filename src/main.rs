extern crate bitflags;
extern crate juniper;

mod gql;
mod models;
mod schema;
mod services;
mod utils;

use std::sync::Arc;

use crate::gql::key_pairs_schema::Schema;
use actix_cors::Cors;
use actix_web::{
    get,
    middleware::Logger,
    route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use gql::key_pairs_schema::create_schema;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let response = data.execute(&st, &()).await;
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(logger)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
