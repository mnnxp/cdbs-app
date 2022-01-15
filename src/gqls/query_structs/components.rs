use graphql_client::GraphQLQuery; 
use crate::types::{UUID};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub struct ComponentFiles;



