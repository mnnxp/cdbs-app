use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
use crate::types::UUID;

/// Получение данных для создания стандарта
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetStandardDataOpt;

/// Регистрация нового страндарта
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterStandard;

/// Получение данных для обновления стандарта
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetUpdateStandardDataOpt;

/// Обновление основной информации о стандарте
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutStandardUpdate;

/// Удаление стандарта
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteStandard;

/// Изменение типа достаупа к стандарту
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ChangeStandardAccess;

/// Создание информации о новых файлах и получение urls для загрузки
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadStandardFiles;

/// Получение информации о файлах стандарта
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct StandardFilesList;

/// Отправка подтверждения о загрузке файлов в хранилище
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ConfirmUploadCompleted;

/// Получение данных стандарта
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetStandardData;

/// Получение файлов стандарта с presigned-uls
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct StandardFiles;

/// Добавление стандарта в избранное
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddStandardFav;

/// Удаление стандарта из избранного
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/standards.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteStandardFav;
