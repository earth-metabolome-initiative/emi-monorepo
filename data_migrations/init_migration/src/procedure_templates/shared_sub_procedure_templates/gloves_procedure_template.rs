//! Submodule defining the Full Organism collection procedure.

use core_structures::{
    CleaningProcedureTemplate, PpeReminderProcedureTemplate, ProcedureTemplate, User,
    tables::insertables::{
        CleaningProcedureTemplateSettable, PpeReminderProcedureTemplateSettable,
        ProcedureTemplateSettable,
    },
    traits::AppendProcedureTemplate,
};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use crate::{
    procedure_template_asset_models::ppe::glove_model_builder,
    procedure_templates::collection_preparation_procedures::ethanol_70_percent_procedure,
};

/// Initializes the "Wear and sterilize gloves" procedure template.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure template.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure template.
/// * If the procedure template building fails.
pub fn gloves_procedure_template(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let name = "Wear and sterilize gloves";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    let (_, bottle) = ethanol_70_percent_procedure(user, conn)?;

    let gloves_procedure = ProcedureTemplate::new()
        .name(name)?
        .description(
            "Procedure template for wearing and sterilizing gloves to ensure safety and hygiene.",
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    // Remind the user to wear gloves
    let gloves_reminder = PpeReminderProcedureTemplate::new()
        .procedure_template_ppe_asset_model(glove_model_builder(user, conn)?)?
        .name("Wear gloves")?
        .description("Please wear gloves to avoid contamination and protect yourself.")?
        .created_by(user)?
        .insert(user.id, conn)?;
    let gloves_model = gloves_reminder.procedure_template_ppe_asset_model(conn)?;

    // Remind the user to sterilize / clean the gloves with ethanol 70
    // percent
    let glove_cleaning_step = CleaningProcedureTemplate::new()
        .name("Sterilize gloves")?
        .description("Please sterilize the gloves with ethanol 70 percent to avoid contamination.")?
        .procedure_template_cleaned_model(&gloves_model)?
        .procedure_template_cleaned_with_model(&bottle)?
        .liters(0.05)?
        .created_by(user)?
        .insert(user.id, conn)?;

    gloves_procedure.extend(&[gloves_reminder.into(), glove_cleaning_step.into()], user, conn)?;

    Ok(gloves_procedure)
}
