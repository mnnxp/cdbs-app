use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
use crate::types::UUID;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/discussions.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterDiscussionComment;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/discussions.graphql",
    response_derives = "Debug"
)]
pub(crate) struct EditComment;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/discussions.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComment;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/discussions.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetDiscussions;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/discussions.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetDiscussionComments;
