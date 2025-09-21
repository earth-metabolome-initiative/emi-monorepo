//! Submodule providing the `PGSetting` struct representing a row of the
//! `pg_settings` table in `PostgreSQL`.

use cached::proc_macro::cached;
use diesel::{
    ExpressionMethods, PgConnection, QueryDsl, Queryable, QueryableByName, RunQueryDsl, Selectable,
};

#[cached(result = true, key = "String", convert = r#"{ "TimeZone".to_owned() }"#)]
fn time_zone(conn: &mut PgConnection) -> Result<PgSetting, diesel::result::Error> {
    use crate::schema::pg_settings;
    pg_settings::table.filter(pg_settings::dsl::name.eq("TimeZone")).first::<PgSetting>(conn)
}

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_settings)]
/// The view `pg_settings` provides access to run-time parameters of the server.
/// It is essentially an alternative interface to the SHOW and SET commands.
/// It also provides access to some facts about each parameter that are not
/// directly available from SHOW, such as minimum and maximum values.
pub struct PgSetting {
    /// Run-time configuration parameter name.
    pub name: String,
    /// Current value of the parameter.
    pub setting: String,
    /// Implicit unit of the parameter.
    pub unit: Option<String>,
    /// Logical group of the parameter
    pub category: String,
    /// A brief description of the parameter
    pub short_desc: String,
    /// Additional, more detailed, description of the parameter.
    pub extra_desc: Option<String>,
    /// Context required to set the parameter's value (see below)
    pub context: String,
    /// Parameter type (bool, enum, integer, real, or string)
    pub vartype: String,
    /// Source of the current parameter value
    pub source: String,
    /// Minimum allowed value of the parameter (null for non-numeric values)
    pub min_val: Option<String>,
    /// Maximum allowed value of the parameter (null for non-numeric values)
    pub max_val: Option<String>,
    /// Allowed values of an enum parameter (null for non-enum values)
    pub enumvals: Option<Vec<String>>,
    /// Parameter value assumed at server startup if the parameter is not
    /// otherwise set
    pub boot_val: Option<String>,
    /// Value that RESET would reset the parameter to in the current session
    pub reset_val: Option<String>,
    /// Configuration file the current value was set in (null for values set
    /// from sources other than configuration files, or when examined by a user
    /// who neither is a superuser nor has privileges of
    /// `pg_read_all_settings`); helpful when using include directives in
    /// configuration files
    pub sourcefile: Option<String>,
    /// Line number within the configuration file the current value was set at
    /// (null for values set from sources other than configuration files, or
    /// when examined by a user who neither is a superuser nor has privileges of
    /// `pg_read_all_settings`).
    pub sourceline: Option<i32>,
    /// true if the value has been changed in the configuration file but needs a
    /// restart; or false otherwise.
    pub pending_restart: Option<bool>,
}

impl PgSetting {
    /// Returns the TIME ZONE setting for the provided `PostgreSQL` connection.
    ///
    /// # Arguments
    ///
    /// * `conn` - A reference to a `PostgreSQL` connection.
    ///
    /// # Errors
    ///
    /// This function will return an error if the query to fetch the TIME ZONE
    /// setting fails.
    pub fn time_zone(conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        time_zone(conn)
    }
}
