use core_structures::{
    PhotographProcedureTemplate, ProcedureTemplate, User,
    tables::insertables::{
        PhotographProcedureTemplateSettable, ProcedureTemplateAssetModelSettable,
        ProcedureTemplateSettable,
    },
};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable, Read};

use crate::procedure_template_asset_models::photograph::photograph_builder;

use crate::procedure_templates::organism_observation_procedure;
use crate::procedure_templates::shared_sub_procedure_templates::part_of_organism_collection;

pub(super) fn cct_and_panel_picture_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let name = "CCT and Panel picture procedure";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    let (_, _organism, phone) = organism_observation_procedure(user, conn)?;
    let (_, cct) = part_of_organism_collection(user, conn)?;

    let cct_and_panel_picture_procedure = PhotographProcedureTemplate::new()
        .name(name)?
        .description(
            "Photograph of the sample collection tube with a visible label together with the organism panel.",
        )?
        .procedure_template_photographed_with_model(&phone)?
        .procedure_template_photographed_asset_model(&cct)?
        .procedure_template_photograph_model(
            photograph_builder(user, conn)?.name("Sample Label and Panel Picture")?,
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    Ok(ProcedureTemplate::read(cct_and_panel_picture_procedure.procedure_template, conn)?)
}
