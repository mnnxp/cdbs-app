use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
// use crate::types::UUID;

/// Отправка подтверждения о загрузке файлов в хранилище
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ConfirmUploadCompleted;

/// Получение путей каталогов (спецификаций)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/specs.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetSpecsPaths;

/// Поиск спецификации по наименованию
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/specs.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SearchSpecs;

/// Получение списка лицензий
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetLicenses;

/// Получение списка программ (для которых создаются наборы файлов)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetPrograms;

/// Получение списка параметров (с переводом для заданного языка)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetParams;

/// Добавление нового имени параметра
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterParam;
