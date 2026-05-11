use graphql_client::GraphQLQuery;
use chrono::NaiveDateTime;
use crate::types::UUID;

/// Get permission list access
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetPermissions;

// ============================================================================
// COMPONENT ACCESS QUERIES
// ============================================================================

/// Get access list for component (users and companies)
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetComponentAccessList;

/// Search users for access management
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SearchUsersForAccess;

/// Search companies for access management
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SearchCompaniesForAccess;

/// Set user access on component
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SetUserAccessComponent;

/// Delete user access from component
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteUserAccessComponent;

/// Set company access on component
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct SetCompanyAccessComponent;

/// Delete company access from component
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompanyAccessComponent;

// ============================================================================
// COMPANY MEMBERS QUERIES
// ============================================================================

/// Get company members list
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetCompanyMembers;

/// Get company roles list
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct GetCompanyRoles;

/// Add member to company
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddCompanyMember;

/// Change member role in company
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ChangeRoleMember;

/// Remove member from company
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompanyMember;

/// Create role for member company
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]

/// Change name role
pub(crate) struct RegisterCompanyRole;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct ChangeNameRoleCompany;

/// Create role from company
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteCompanyRole;

/// Add access to role
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct AddAccessRole;

/// Remove access from role
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/rbac.graphql",
    response_derives = "Debug"
)]
pub(crate) struct DeleteAccessRole;