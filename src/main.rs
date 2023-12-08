use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, MergedObject, Object, Schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use schema::home::file_resolver::FileMutation;
use tower_http::cors::CorsLayer;
pub mod schema;
async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}
#[derive(Default, MergedObject)]
pub struct Mutation(FileMutation);
async fn graphiql() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let schema = Schema::build(Query, Mutation::default(), EmptySubscription).finish();
    let router = Router::new()
        .route("/", get(hello_world))
        .route(
            "/graphql",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]),
        );
    Ok(router.into())
}
