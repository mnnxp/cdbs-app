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

/// Получение списка компонентов
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentsShortList;

/// Удаление файла компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponentFile;

/// Получить ключевые слова (теги) компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentKeywords;

/// Добавить ключевые слова по наименованию
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddComponentKeywordsByNames;

/// Удаление ключевых слов компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponentKeywords;

/// Добавление лицензии к компоненту
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddComponentLicense;

/// Получение лицензии компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentLicenses;

/// Удаление лицензии компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponentLicense;

/// Получение списка файлов модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComponentModificationFilesList;

/// Создание информации о новых файлах и получение urls для загрузки
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadModificationFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComponentModificationFiles;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteModificationFile;

/// Получение файлов из набора файлов модификации компонента с presigned-uls
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComModFilesetFiles;

/// Удаление файлов из набора файлов модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteFilesFromFileset;

/// Получение информации о файлах из набора модификации
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComModFilesOfFileset;

/// Добавление набора файлов для модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterModificationFileset;

/// Удаление набра файлов из модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteModificationFileset;

/// Создание информации о новых файлах в наборе файлов и получение urls для загрузки
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadFilesToFileset;

/// Добавление модификации для компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterComponentModification;

/// Добавление модификаций с параметрами для компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct RegisterComponentModificationsBulk;

/// Обновление информации о модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutComponentModificationUpdate;

/// Удаление модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponentModification;

/// Получение модификаций компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentModifications;

/// Получение наборов файлов для модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComponentModificationFilesets;

/// Доступные статусы для компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComponentActualStatuses;

/// Обновление параметров модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutModificationParams;

/// Удаление параметра модификации компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteModificationParams;

/// Изменение значения параметра компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct PutComponentParams;

/// Получение параметров компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentParams;

/// Удаление параметра компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponentParams;

/// Добавление спецификаций для компонента (связь с каталогами)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddComponentSpecs;

/// Удаление спецификаций компонента (связи с каталогами)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteComponentSpecs;

/// Добавление стандарта к компоненту
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddStandardToComponent;

/// Получение стандартов связанных с компонентом
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentStandards;

/// Удаление связи стандарта с компонентом
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteStandardsComponent;

/// Указать основного поставщика
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SetCompanyOwnerSupplier;

/// Добавить поставщика компонента (для базовых компонентов)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddComponentSupplier;

/// Получить список поставщиков компонента
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ComponentSuppliers;

/// Удалить поставщиков компонента (по Uuid)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteSuppliersComponent;

/// Обновить основное изображение компонента (показывается в списке компонентов)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct UploadComponentFavicon;

/// Поиск компонентов по параметрам
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/components.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SearchByComponents;