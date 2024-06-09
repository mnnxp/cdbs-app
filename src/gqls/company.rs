use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
use crate::types::UUID;

/// Получение данных для создания компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetCreateCompanyDataOpt;

/// Регистрация новой компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterCompany;

/// Получение данных для настройки компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetCompanySettingDataOpt;

/// Получение данных компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetCompanyData;

/// Получение данных о компании со статусом поставщика (без токена)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetSupplierCompanyData;

/// Обновление данных компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct CompanyUpdate;

/// Изменение типа доступа компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ChangeCompanyAccess;

/// Удаление компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompany;

/// Добавление компании в избранное
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddCompanyFav;

/// Удаление компании из избранного
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompanyFav;

/// Обновление аватара компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadCompanyFavicon;

/// Получение списка компаний
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetCompaniesShortList;

/// Загрузка сертификата компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadCompanyCertificate;

/// Обновление описания сертификата компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UpdateCompanyCertificate;

/// Удаление сертификата компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompanyCertificate;

/// Получение данных для добавления представительства компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetRepresentDataOpt;

/// Добавление представительства компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterCompanyRepresent;

/// Обновление представительства компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UpdateCompanyRepresent;

/// Удаление представительства компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompanyRepresent;

/// Добавление спецификаций компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddCompanySpecs;

/// Удаление спецификаций компании
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/companies.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompanySpecs;
