//! Submodule providing method to check whether a user is authorized to perform a given operation
//! on the provided table and Uuid.

use crate::DieselConn;
use diesel::prelude::*;
use web_common::api::ws::messages;

/// Check whether a user is a website admin.
///
/// # Implementation details
/// The website administrators are defined as the user that are part of the
/// team with the name "Administrators". The team is defined in the teams table,
/// the name is stored in the describable table, and the membership is defined in
/// the user_authorizations table. The user must have Admin role in the team to be
/// considered an admin. An admin role is defined as a role with the name "admin",
/// and the name is stored in the describable table. The role is associated with
/// the user and the team by the user_authorizations table.
pub(crate) fn is_admin(
    conn: &mut DieselConn,
    user_id: uuid::Uuid,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::describables;
    use crate::schema::roles;
    use crate::schema::teams;
    use crate::schema::user_authorizations;

    let admin_team_id: uuid::Uuid = teams::dsl::teams
        .inner_join(describables::dsl::describables)
        .filter(describables::dsl::name.eq("Administrators"))
        .select(teams::dsl::id)
        .get_result(conn)?;

    let admin_role_id: uuid::Uuid = roles::dsl::roles
        .inner_join(describables::dsl::describables)
        .filter(describables::dsl::name.eq("admin"))
        .select(roles::dsl::id)
        .get_result(conn)?;

    let is_admin: bool = user_authorizations::dsl::user_authorizations
        .filter(user_authorizations::dsl::user_id.eq(user_id))
        .filter(user_authorizations::dsl::editable_id.eq(admin_team_id))
        .filter(user_authorizations::dsl::role_id.eq(admin_role_id))
        .select(diesel::dsl::sql::<diesel::sql_types::Bool>("true"))
        .get_result(conn)?;

    Ok(is_admin)
}

/// Check whether a user is authorized to perform a given operation on the provided table and Uuid.
///
/// # Arguments
/// * `conn` - A connection to the database.
/// * `user_id` - The user id to check authorization for.
/// * `table` - The table to check authorization for.
/// * `id` - The id of the row to check authorization for.
pub(crate) fn is_authorized(
    conn: &mut DieselConn,
    user_id: uuid::Uuid,
    table: messages::Table,
    id: uuid::Uuid,
    roles: Vec<messages::Role>,
) -> Result<bool, diesel::result::Error> {

    use diesel::dsl::sql;
    // Preliminarily, we check whether the provided table is a valid table.
    debug_assert!(diesel::select(sql::<diesel::sql_types::Bool>(&format!(
        "EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public' AND table_name = '{}'
        )",
        table
    )))
    .get_result(conn)?);

    // First, we check whether the provided ID exists in the provided table.
    // We only do this check in debug mode, as it is not necessary in release mode
    // since this method is only called from within this crate and it would be
    // a bug to call it with an invalid table or id.
    debug_assert!(diesel::select(sql::<diesel::sql_types::Bool>(&format!(
        "EXISTS (
            SELECT 1
            FROM {} WHERE id = '{}'
        )",
        table, id
    )))
    .get_result(conn)?);

    if table.is_users() {
        // If the table is the users table, we check whether the provided user ID is the
        // same as the ID of the user we are checking authorization for. If it is, we
        // return true, as a user is always authorized to perform operations on itself.
        if user_id == id {
            return Ok(true);
        }

        // If one of the requested roles is the admin role, we check whether the user is
        // within the global admin team.
        return Ok(roles.contains(&messages::Role::Admin) && is_admin(conn, user_id)?);
    }

    // We convert the provided Roles into the associated Uuids by querying the database.
    // The roles name is stored in the describable table, so we need to join the roles table
    // with the describable table to get the Uuid of the roles.
    let roles = {
        use crate::schema::describables;
        use crate::schema::roles;

        let role_names = roles
            .iter()
            .map(|role| role.to_string())
            .collect::<Vec<String>>();

        let roles: Vec<uuid::Uuid> = roles::dsl::roles
            // We join the roles and the describables tables on the id column.
            .inner_join(describables::dsl::describables)
            .filter(describables::dsl::name.eq_any(role_names))
            .select(describables::dsl::id)
            .get_results(conn)?;

        roles
    };

    // Now that we know that the table and id are valid, we can check whether the user is authorized
    // to perform the given operation on the given table and id. The information is stored in the
    // user_authorizations table. We check whether the user has any of the roles requested.

    {
        use crate::schema::user_authorizations;

        let authorized: bool = user_authorizations::dsl::user_authorizations
            .filter(user_authorizations::dsl::user_id.eq(user_id))
            .filter(user_authorizations::dsl::editable_id.eq(id))
            .filter(user_authorizations::dsl::role_id.eq_any(&roles))
            .select(sql::<diesel::sql_types::Bool>("true"))
            .get_result(conn)?;

        if authorized {
            return Ok(true);
        }
    }

    // Afterwards, if the user is not authorized, we check whether any of the teams where
    // the user is a member are authorized to perform the given operation on the given table and id.

    // We first need to get the ids of the teams where the user is a member. Teams are defined in the
    // teams table, and the membership is defined in the user_authorizations table. We join the teams
    // and the user_authorizations tables on the editables_id column, and we filter the user_authorizations
    // table on the user_id column and the role_id column. We then select the id column of the teams table.
    let team_ids = {
        use crate::schema::teams;
        use crate::schema::user_authorizations;

        let team_ids: Vec<uuid::Uuid> = teams::dsl::teams
            // We join the teams and the user_authorizations tables on respectively the id and the editable_id columns.
            .inner_join(
                user_authorizations::dsl::user_authorizations
                    .on(teams::dsl::id.eq(user_authorizations::dsl::editable_id)),
            )
            .filter(user_authorizations::dsl::user_id.eq(user_id))
            .filter(user_authorizations::dsl::role_id.eq_any(&roles))
            .select(teams::dsl::id)
            .get_results(conn)?;

        team_ids
    };

    // Now having the set of team ids, we can check whether any of these teams have the appropriate
    // role to to green light this operation. We do this by querying the team_authorizations table.

    {
        use crate::schema::team_authorizations;

        let authorized: bool = team_authorizations::dsl::team_authorizations
            .filter(team_authorizations::dsl::team_id.eq_any(team_ids))
            .filter(team_authorizations::dsl::editable_id.eq(id))
            .filter(team_authorizations::dsl::role_id.eq_any(&roles))
            .select(sql::<diesel::sql_types::Bool>("true"))
            .get_result(conn)?;

        if authorized {
            return Ok(true);
        }
    }

    // Afterwards, if the user is still not authorized, we check whether any of the organizations
    // where the user is a member are authorized to perform the given operation on the given table and id.

    // First, we need to get the ids of the organizations where the user is a member. Organizations are
    // defined in the organizations table, and the membership is defined in the user_authorizations table.
    // We join the organizations and the user_authorizations tables on the editables_id column, and we filter
    // the user_authorizations table on the user_id column and the role_id column. We then select the id column
    // of the organizations table.
    let organization_ids = {
        use crate::schema::organizations;
        use crate::schema::user_authorizations;

        let organization_ids: Vec<uuid::Uuid> = organizations::dsl::organizations
            // We join the organizations and the user_authorizations tables on respectively the id and the editable_id columns.
            .inner_join(
                user_authorizations::dsl::user_authorizations
                    .on(organizations::dsl::id.eq(user_authorizations::dsl::editable_id)),
            )
            .filter(user_authorizations::dsl::user_id.eq(user_id))
            .filter(user_authorizations::dsl::role_id.eq_any(&roles))
            .select(organizations::dsl::id)
            .get_results(conn)?;

        organization_ids
    };

    // Now having the set of organization ids, we can check whether any of these organizations have the appropriate
    // role to to green light this operation. We do this by querying the organization_authorizations table.

    {
        use crate::schema::organization_authorizations;

        let authorized: bool = organization_authorizations::dsl::organization_authorizations
            .filter(organization_authorizations::dsl::organization_id.eq_any(organization_ids))
            .filter(organization_authorizations::dsl::editable_id.eq(id))
            .filter(organization_authorizations::dsl::role_id.eq_any(&roles))
            .select(sql::<diesel::sql_types::Bool>("true"))
            .get_result(conn)?;

        if authorized {
            return Ok(true);
        }
    }

    Ok(false)
}