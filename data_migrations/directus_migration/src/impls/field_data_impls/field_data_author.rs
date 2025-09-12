//! Submodule providing helpers methods to work with the FieldDatum author.

use core_structures::{User};
use diesel::PgConnection;

use crate::codegen::FieldDatum;

use web_common_traits::database::Insertable;
use core_structures::tables::insertables::UserSettable;

use web_common_traits::database::InsertableVariant;

impl FieldDatum {
    /// Returns the author of the field datum if it exists.
    pub fn author(&self, portal: &mut PgConnection) -> anyhow::Result<User> {
        if let Some(collector_fullname) = &self.collector_fullname {
            return dispatch_user_from_name(collector_fullname, portal);
        }
        todo!("dispatch author retrieval to field_data_author module")
    }
}

fn dispatch_user_from_name(name: &str, portal: &mut PgConnection) -> anyhow::Result<User> {
    match name {
        "Lysandre Journiac" => get_or_insert_user("Lysandre", "Journiac", portal),
        _ => todo!("implement user dispatch from name: {name}"),
    }
}


fn get_or_insert_user(first_name: &str, last_name: &str, portal: &mut PgConnection) -> anyhow::Result<User> {

    let users = User::from_first_name_and_last_name(first_name, last_name, portal)?;

    if !users.is_empty() {
        assert_eq!(users.len(), 1, "Expected exactly one user with name {first_name} {last_name}, found {}", users.len());
        return Ok(users.into_iter().next().unwrap());
    }

    let user: User = User::new().last_name(last_name)?.first_name(first_name)?.insert(0, portal)?;

    Ok(user)
}
