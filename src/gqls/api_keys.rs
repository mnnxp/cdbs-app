use graphql_client::GraphQLQuery;
use chrono::DateTime as ChronoDateTime;
use chrono::Utc;

pub type DateTime = ChronoDateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/api_keys.graphql",
    response_derives = "Debug"
)]
pub struct GetApiKeys;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/api_keys.graphql",
    response_derives = "Debug"
)]
pub struct CreateApiKey;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/api_keys.graphql",
    response_derives = "Debug"
)]
pub struct DeleteApiKey;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/api_keys.graphql",
    response_derives = "Debug"
)]
pub struct RotateApiKey;