use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
use crate::types::UUID;

/// Получение данных для создания услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetServiceDataOpt;

/// Регистрация новой услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ServiceRequest;

/// Получение данных для обновления услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetUpdateServiceDataOpt;

/// Обновление основной информации о стандарте
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutServiceUpdate;

/// Удаление услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteService;

/// Изменение типа достаупа к стандарту
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ChangeServiceAccess;

/// Создание информации о новых файлах и получение urls для загрузки
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadServiceFiles;

/// Получение информации о файлах услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ServiceFilesList;

/// Получение данных услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetServiceData;

/// Получение файлов услуги с presigned-uls
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ServiceFiles;

/// Получение списка стандартов с краткой информацией
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetServicesShortList;

/// Удалить файлы услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteServiceFile;

/// Изменение значения параметра услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutServiceParams;

/// Получение параметров услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetServiceParams;

/// Удаление параметра услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteServiceParams;

/// Получение ключевых слов (тегов) услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetServiceKeywords;

/// Добавление ключевых слов (тегов) по наименованию
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddServiceKeywordsByNames;

/// Удаление ключевых слов (тегов) услуги
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteServiceKeywords;

/// Добление связи услуги с каталогами
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddServiceSpecs;

/// Удаление связи каталога со стандартом
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/service.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteServiceSpecs;
