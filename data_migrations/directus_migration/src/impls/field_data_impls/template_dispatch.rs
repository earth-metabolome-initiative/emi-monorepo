//! Submodule to provide a template dispatch for `FieldDatum` related methods.

use core_structures::{ProcedureTemplate, User};
use diesel::PgConnection;
use directus_codegen::FieldDatum;
use init_migration::dbgi_plan;

use crate::{directus_template::DirectusTemplate, structs::FieldDatumWrapper};

impl FieldDatumWrapper {
    pub(crate) fn directus_template_dispatch(&self) -> DirectusTemplate {
        match self.as_ref().qfield_project.as_str().trim() {
            "heloise_bachelor_FiBL" => DirectusTemplate::FIBL,
            "jbuf" | "jbn" | "jbc" | "kew-botanical-gardens" | "jbp" => DirectusTemplate::DBGI,
            "artemisia_absinthium"
            | "sbl_20004"
            | "emi"
            | "chemical_ecology"
            | "champex_gradient"
            | "medicin_alps"
            | "bioblitz_vinesch" => {
                if self.as_ref().observation_subject.as_deref() == Some("Whole organism")
                    || self.as_ref().taxon_name.as_deref() == Some("Insecta")
                {
                    DirectusTemplate::EMIWholeOrganism
                } else {
                    DirectusTemplate::EMIPartOfOrganism
                }
            }
            _ => todo!("implement directus template dispatch for FieldDatum {self:#?}"),
        }
    }

    pub(crate) fn procedure_dispatch(
        &self,
        user: &User,
        conn: &mut PgConnection,
    ) -> anyhow::Result<()> {
        match self.directus_template_dispatch() {
            DirectusTemplate::DBGI => self.dbgi_wizard_migration(user, conn),
            DirectusTemplate::EMIPartOfOrganism => {
                self.emi_part_of_organism_wizard_migration(user, conn)
            }
            DirectusTemplate::EMIWholeOrganism => {
                self.emi_whole_organism_wizard_migration(user, conn)
            }
            DirectusTemplate::FIBL => self.fibl_wizard_migration(user, conn),
        }
    }
}
