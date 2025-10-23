//! Model representing administrable role authorizations in PostgreSQL.

use diesel::{Queryable, QueryableByName, Selectable};

#[derive(Queryable, QueryableByName, Selectable, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::administrable_role_authorizations::administrable_role_authorizations)]
#[diesel(primary_key(grantee, role_name))]
/// Represents a row in PostgreSQL's `administrable_role_authorizations` view.
///
/// Each row describes a **role authorization** that is administrable by the
/// current user. This view shows role grants where the current user has the
/// authority to grant or revoke the role membership. Role authorizations
/// represent the granting of one role to another role or user, establishing a
/// membership relationship in PostgreSQL's role-based access control system.
///
/// Fields are nullable because PostgreSQL's information schema views can
/// contain missing or context-dependent values.
///
/// For more details:
/// - PostgreSQL Catalog: [`pg_auth_members`](https://www.postgresql.org/docs/current/catalog-pg-auth-members.html)
/// - Information Schema View: [`administrable_role_authorizations`](https://www.postgresql.org/docs/current/infoschema-administrable-role-authorizations.html)
pub struct AdministrableRoleAuthorizations {
    /// Name of the role that was granted the privilege (the grantee).
    /// This is the role or user that received the role membership.
    pub grantee: Option<String>,

    /// Name of the role that was granted to the grantee.
    /// This is the role whose privileges and permissions are being granted.
    pub role_name: Option<String>,

    /// Indicates whether the grantee has the option to grant this role to
    /// others. "YES" if the grantee can grant the role to other users/roles
    /// (WITH GRANT OPTION), "NO" if the grantee cannot grant the role to
    /// others.
    pub is_grantable: Option<String>,
}
