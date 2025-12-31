mod math;
mod ms365;
mod salesforce;
mod sap;

use actix_web::{App, HttpResponse, HttpServer, guard, web};
use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_actix_web::GraphQL;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  
    Ok(())
}
