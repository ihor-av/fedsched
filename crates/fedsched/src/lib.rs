mod error;
mod graphql;
mod surql_templates;
use crate::{
    config::{FieldConfig, mandatory_event_table_fields, smart_config_merge},
    error::FedschedResult,
    graphql::{SchedulerContext, build_schema},
};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Extension, Router,
    response::{Html, IntoResponse},
    routing::get,
};
pub mod config;

pub async fn setup_scheduler(cfgs: Vec<FieldConfig>) -> FedschedResult<Router> {
    let mandatory_cfgs = mandatory_event_table_fields();
    let cfgs = smart_config_merge(cfgs, mandatory_cfgs);
    let ctx = SchedulerContext::build_ctx_from_cfgs(&cfgs).await?;
    let schema = build_schema(ctx, cfgs)?;
    Ok(Router::new()
        .route("/", get(playground_handler).post(graphql_handler))
        .layer(Extension(schema)))
}

async fn playground_handler() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(
    schema: Extension<async_graphql::dynamic::Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub mod prelude {
    pub use crate::{
        config::{FieldConfig, FieldConstraint},
        error::{FedschedError, FedschedResult},
        setup_scheduler,
    };
}
