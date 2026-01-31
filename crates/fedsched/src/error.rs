use thiserror::Error;

#[derive(Error, Debug)]
pub enum FedschedError {
    #[error("Table '{table}' has duplicate field: '{field}'")]
    DuplicateField { table: String, field: String },
    #[error("Configuration contains a table with an empty name")]
    EmptyTableName,
    #[error("Missing required constraint for field '{0}'")]
    InvalidConstraint(String),

    #[error("Template rendering failed: {0}")]
    Template(#[from] askama::Error),
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),
    #[error("DB error: {0}")]
    Db(#[from] surrealdb::Error),
    #[error("Can't serialize into JSON: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Can't build GQL schema: {0}")]
    GqlSchema(#[from] async_graphql::dynamic::SchemaError),
}
pub type FedschedResult<T> = Result<T, FedschedError>;
