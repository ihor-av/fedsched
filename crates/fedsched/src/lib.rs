use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router, extract::Extension, http::header::CONTENT_TYPE, response::IntoResponse, routing::post,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(SimpleObject)]
struct Event {
    id: ID,
    name: String,
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn event(&self, id: ID) -> Event {
        let name = format!("Demo event with id: {id:?}");
        Event { id, name }
    }
}

pub async fn setup_scheduler() -> Router {
    let schema = Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription).finish();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors)
}

async fn graphql_handler(
    Extension(schema): Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> impl IntoResponse {
    let inner_req = req.into_inner();
    let resp = schema.execute(inner_req).await;
    GraphQLResponse::from(resp)
}
