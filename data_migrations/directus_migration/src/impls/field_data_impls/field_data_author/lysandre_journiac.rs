//! Submodule handling the retrieval or creation of the user "Lysandre Journiac"

use core_structures::User;
use diesel::PgConnection;
use web_common_traits::{database::InsertableVariant, prelude::Insertable};
use core_structures::tables::insertables::UserSettable;

pub fn lysandre_journiac(portal: &mut PgConnection) -> anyhow::Result<User> {
    todo!("implement lysandre journiac user creation/retrieval");

    let user: User = User::new().last_name("Journiac")?.first_name("Lysandre")?.insert(0, portal)?;

    Ok(user)
}
