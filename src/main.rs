mod analytics;
mod appstate;
mod graphql;
mod math;
mod mcp;
mod ms365;
mod salesforce;
mod sap;

use actix_cors::Cors;
use actix_web::web::resource;
use actix_web::{App, HttpServer, guard, web};
use appstate::AppState;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;
use dotenv::from_path;
use graphql::{Query, graphiql_index};
use mcp::Mcp;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp_actix_web::transport::StreamableHttpService;
use std::sync::RwLock;
use std::{clone::Clone, sync::Arc, time::Duration};

use crate::graphql::graphql_ws;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    from_path(".env.local").unwrap_or(());
    let service = StreamableHttpService::builder()
        .service_factory(Arc::new(|| Ok(Mcp::new())))
        .session_manager(Arc::new(LocalSessionManager::default()))
        .stateful_mode(true)
        .sse_keep_alive(Duration::from_secs(30))
        .build();

    let appstate = AppState::default();
    // let client = web::Data::new(reqwest::Client::default());

    HttpServer::new(move || {
        let schema = Schema::build(
            Query::default(),
            EmptyMutation::default(),
            EmptySubscription::default(),
        )
        .data(appstate.clone())
        .enable_federation()
        .finish();
        App::new()
            .wrap(Cors::permissive())
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema.clone())),
            )
            .service(
                resource("/api/v1/graphql")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(graphql_ws),
            )
            .service(resource("/").guard(guard::Get()).to(graphiql_index))
            .service(web::scope("/api/v1/mcp").service(service.clone().scope()))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
