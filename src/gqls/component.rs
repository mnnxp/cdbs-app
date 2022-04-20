use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
use crate::types::UUID;

/// Получение данных для создания компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentDataOpt;

/// Создание нового компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterComponent;

/// Получение данных для обновления компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetUpdateComponentDataOpt;

/// Обновление основной информации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutComponentUpdate;

/// Удаление компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponent;

/// Изменение типа доступа компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ChangeComponentAccess;

/// Получение списка файлов компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComponentFilesList;

/// Создание информации о новых файлах и получение urls для загрузки
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadComponentFiles;

/// Отправка подтверждения о загрузке файлов в хранилище
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/relate.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ConfirmUploadCompleted;

/// Получение файлов компонента с presigned-uls
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComponentFiles;

/// Получение основных данных компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentData;

/// Добавление компонента в избранное
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddComponentFav;

/// Удаление компонента из избранного
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponentFav;
