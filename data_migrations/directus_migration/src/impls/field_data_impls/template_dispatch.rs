//! Submodule to provide a template dispatch for `FieldDatum` related methods.

use core_structures::User;
use diesel::PgConnection;
use directus_codegen::FieldDatum;
use init_migration::dbgi_plan;
use core_structures::ProcedureTemplate;

use crate::{directus_template::DirectusTemplate, structs::FieldDatumWrapper};

impl FieldDatumWrapper {

    pub(crate) fn directus_template_dispatch(&self) -> DirectusTemplate {
        if self.as_ref().qfield_project.as_deref() == Some(
            "heloise_bachelor_FiBL",
        ) {
            return DirectusTemplate::FIBL;
        }
        todo!("implement directus template dispatch for FieldDatum {self:#?}");
    }

    pub(crate) fn procedure_dispatch(&self, user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
        match self.directus_template_dispatch() {
            DirectusTemplate::DBGI => {
                self.dbgi_wizard_migration(user, conn)
            }
            DirectusTemplate::VineshPartOfOrganism => {
                todo!("implement procedure dispatch for vinesh part of organism template")
            }
            DirectusTemplate::VineshWholeOrganism => {
                todo!("implement procedure dispatch for vinesh whole organism template")
            }
            DirectusTemplate::FIBL => {
                todo!("implement procedure dispatch for FIBL template")
            }
        }
    }
}
