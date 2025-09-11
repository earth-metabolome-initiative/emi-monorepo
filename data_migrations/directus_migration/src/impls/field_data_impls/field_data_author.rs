//! Submodule providing helpers methods to work with the FieldDatum author.

use core_structures::{User};

use crate::codegen::FieldDatum;

mod lysandre_journiac;
use lysandre_journiac::lysandre_journiac;

impl FieldDatum {
    /// Returns the author of the field datum if it exists.
    pub fn author(&self, portal: &mut diesel::PgConnection) -> anyhow::Result<User> {
        if let Some(collector_fullname) = &self.collector_fullname {
            return dispatch_user_from_name(collector_fullname, portal);
        }
        todo!("dispatch author retrieval to field_data_author module")
    }
}

fn dispatch_user_from_name(name: &str, portal: &mut diesel::PgConnection) -> anyhow::Result<User> {
    match name {
        "Lysandre Journiac" => lysandre_journiac(portal),
        _ => todo!("implement user dispatch from name: {name}"),
    }
}

