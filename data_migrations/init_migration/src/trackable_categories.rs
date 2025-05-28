//! Submodule initializing trackable categories.

use core_structures::{TrackableCategory, User};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub const ETHANOL_95: &str = "Absolute Ethanol, >= 95%";
pub const METHANOL_HPLC: &str = "Methanol, >= 99.8%";
pub const FORMIC_ACID: &str = "Formic acid, 98+%";
pub const DISTILLED_WATER: &str = "Distilled water";
pub const LIQUID_NITROGEN: &str = "Liquid nitrogen";

/// Initializes the trackable categories for the user.
///
/// # Arguments
///
/// * `user` - The user for whom the trackable categories are being initialized.
/// * `portal_conn` - The database connection.
///
/// # Errors
///
/// * If the connection to the database fails.
pub(super) async fn init_trackable_categories(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let _ethanol = TrackableCategory::new()
        .name(ETHANOL_95)?
        .description("Absolute Ethanol, >= 95%, with 5% isopropanol")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _methanol = TrackableCategory::new()
        .name(METHANOL_HPLC)?
        .description("Methanol, >= 99.8%, HPLC grade")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _formic_acid = TrackableCategory::new()
        .name(FORMIC_ACID)?
        .description("Formic acid, 98+%, pure")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _distilled_water = TrackableCategory::new()
        .name(DISTILLED_WATER)?
        .description("Distilled water, pure")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _liquid_nitrogen = TrackableCategory::new()
        .name(LIQUID_NITROGEN)?
        .description("Liquid nitrogen, pure")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
