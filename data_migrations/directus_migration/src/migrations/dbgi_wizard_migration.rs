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
    dbgi_procedure_template,
};
use web_common_traits::{
    database::{BoundedRead, DispatchableInsertableVariant},
    prelude::{Builder, Insertable},
};

use crate::FieldDatumWrapper;

impl FieldDatumWrapper {
    pub(crate) fn dbgi_wizard_migration(
        &self,
        user: &User,
        conn: &mut diesel::PgConnection,
    ) -> anyhow::Result<()> {
        let procedure_template = dbgi_procedure_template(user, conn)?;
        let procedure_graph = ProcedureTemplateGraph::new(&procedure_template, conn)?;

        let photograph_model = photograph_model(user, conn)?;
        let phone_model = phone_model(user, conn)?;

        let sample_source = self.sample_source(&user, conn)?;
        let sample_source_model = self.sample_source_model(&user, conn)?;

        let mut sample_builder = Sample::new()
            .name(self.sample_id()?)
            .expect(&format!("Got error while processing sample_id: {self:?}"))
            .sample_source(sample_source.as_ref())?
            .created_by(&user)?;

        sample_builder = SampleSettable::model(sample_builder, self.sample_model(&user, conn)?)?;

        let sample: Sample = sample_builder.insert(user.id, conn).unwrap();

        let organism_observation_procedure = GuidedProcedure::new()
            .author(&user)
            .graph(&procedure_graph)
            .connection(conn)
            .build()?;

        organism_observation_procedure
        .and_then::<core_structures::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure, anyhow::Error>(|mut builder, conn| {
            let photograph = Photograph::new()
                .model(&photograph_model)?
                .created_by(&user)?
                .insert(user.id, conn)?;

            builder = if let Some(sample_source) = &sample_source {
               builder.procedure_photographed_asset(ProcedureAsset::new().asset(sample_source)?)?
            } else {
               builder.procedure_photographed_asset(ProcedureAsset::new().asset_model(sample_source_model)?)?
            };

            builder = builder
                .procedure_photographed_with(ProcedureAsset::new().asset_model(phone_model)?)?
                .procedure_photograph(ProcedureAsset::new().asset(photograph)?)?;
            Ok(builder)
        }).expect(&format!("Failed to build photograph procedure for FieldDatum {self:#?}"))
        .and_then::<core_structures::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure, anyhow::Error>(|mut builder, conn| {
            let photograph = Photograph::new()
                .model(&photograph_model)?
                .created_by(&user)?
                .insert(user.id, conn)?;
            builder = builder.procedure_photograph(ProcedureAsset::new().asset(photograph)?)?;
            Ok(builder)
        })?
        .and_then::<core_structures::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure, anyhow::Error>(|mut builder, conn| {
            let photograph = Photograph::new()
                .model(&photograph_model)?
                .created_by(&user)?
                .insert(user.id, conn)?;
            builder = builder.procedure_photograph(ProcedureAsset::new().asset(photograph)?)?;
            Ok(builder)
        })?
        .and_then::<core_structures::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure, anyhow::Error>(|mut builder, conn| {
            let photograph = Photograph::new()
                .model(&photograph_model)?
                .created_by(&user)?
                .insert(user.id, conn)?;
            builder = builder.procedure_photograph(ProcedureAsset::new().asset(photograph)?)?;
            Ok(builder)
        })?
        .and_then::<core_structures::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure, anyhow::Error>(|mut builder, conn| {
            let photograph = Photograph::new()
                .model(&photograph_model)?
                .created_by(&user)?
                .insert(user.id, conn)?;
            builder = builder.procedure_photograph(ProcedureAsset::new().asset(photograph)?)?;
            Ok(builder)
        })?
        .and_then::<core_structures::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure, anyhow::Error>(|mut builder, conn| {
            builder = builder.location(self.geolocation())?;
            Ok(builder)
        })?
        .finish()?;
        Ok(())
    }
}
