//! Submodule gathering the methods used in the DBGI Directus migration

use core_structures::{
    Photograph, PhotographProcedure, ProcedureAsset, Sample, User,
    tables::insertables::{
        AssetSettable, GeolocationProcedureSettable, PhotographProcedureSettable,
        ProcedureAssetSettable, SampleSettable,
    },
};
use guided_procedures::{GuidedProcedure, ProcedureTemplateGraph};
use init_migration::{
    asset_models::{instruments::phone::phone_model, photographs::photograph_model},
    dbgi_plan,
};
use web_common_traits::{
    database::{BoundedRead, DispatchableInsertableVariant},
    prelude::{Builder, Insertable},
};

use crate::FieldDatumWrapper;

impl FieldDatumWrapper {
    pub(crate) fn emi_part_of_organism_wizard_migration(
        &self,
        user: &User,
        conn: &mut diesel::PgConnection,
    ) -> anyhow::Result<()> {
        todo!("implement procedure dispatch for EMI part of organism template")
    }
}
