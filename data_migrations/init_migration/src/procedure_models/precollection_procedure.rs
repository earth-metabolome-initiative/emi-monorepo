use container_categories::ContainerCategory;
use core_structures::{ProcedureModel, TrackableCategory, User};
use diesel_async::AsyncPgConnection;
use nameplate_categories::NameplateCategory;
use tool_categories::ToolCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use super::shared_step_models::{init_aliquoting_step_model, init_box_qrcode_step_model};
use crate::trackable_categories::LIQUID_NITROGEN;

pub(crate) const PRECOLLECTION_PROCEDURE: &str = "Pre-collection procedure";

pub(super) async fn init_precollection_procedure_model(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let dbgi_precollection_procedure: ProcedureModel = ProcedureModel::new()
        .name(PRECOLLECTION_PROCEDURE)?
        .description("Prepare all the material needed to perform a collection procedure")?
        .icon("blender")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let box_nameplate = dbgi_precollection_procedure
        .nameplate(NameplateCategory::SemiPermanent, user, portal_conn)
        .await?;

    let r#box = dbgi_precollection_procedure
        .container(ContainerCategory::ContainerBox, user, portal_conn)
        .await?;

    let eyesight = dbgi_precollection_procedure
        .tool(ToolCategory::EmpiricalMeasurementTool, user, portal_conn)
        .await?;

    init_box_qrcode_step_model(
        user,
        &dbgi_precollection_procedure,
        &box_nameplate,
        &r#box,
        portal_conn,
    )
    .await?;

    let liquid_nitrogen = TrackableCategory::from_name(LIQUID_NITROGEN, portal_conn)
        .await?
        .expect("Liquid nitrogen not found");

    let aliquoting_liquid_nitrogen_photograph = core_structures::create_photograph(
        include_bytes!("../../images/cleaning.jpg"),
        user,
        portal_conn,
    )
    .await?;

    init_aliquoting_step_model(
        user,
        &dbgi_precollection_procedure,
        &r#box, 
        &[&eyesight],
        &liquid_nitrogen,
        2.0,
        "Prepare a liquid nitrogen box by purring liquid 5 to 7 centimeters of liquid nitrogen inside the box",
        &aliquoting_liquid_nitrogen_photograph,
        portal_conn
    ).await?;

    Ok(())
}
