use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
use crate::types::UUID;

/// Получение данных для создания нового пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/register.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterOpt;

/// Регистрация нового пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/register.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegUser;

/// Получение краткой информации об авторизированном пользователе
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetMySelf;

/// Деактивация активного токена пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct Logout;

/// Получение уведомлений авторизированного пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetNotifications;

/// Изменение статуса уведомления на "прочитанного"
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SetReadNotifications;

/// Удаление уведомления пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteNotifications;

/// Добавление пользователя в избранное
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddUserFav;

/// Удаление пользователя из избранного
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteUserFav;

/// Получение данных профиля авторизированного пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetSelfData;

/// Получение данных профиля
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetUserData;

/// Получение данных для обновления информации профиля
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetSettingDataOpt;

/// Обновление основной информации профиля
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UserUpdate;

/// Обновление пароля пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutUpdatePassword;

/// Изменение типа доступа к профилю пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ChangeTypeAccessUser;

/// Удаление данных профиля
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteUserData;

/// Обновление аватара пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadUserFavicon;

/// Получение списка пользователей (краткая информация)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetUsersShortList;

/// Добавление сертификата пользователя и получение presigned-url для загрузки файла
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadUserCertificate;

/// Обновление описания сертификата пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UpdateUserCertificate;

/// Удаление сертификата пользователя
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteUserCertificate;
