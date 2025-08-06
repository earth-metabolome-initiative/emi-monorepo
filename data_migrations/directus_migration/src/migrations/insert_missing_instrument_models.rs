//! Submodule for inserting the missing instrument models which
//! are present in the Directus database but not in the Portal database.

use core_structures::{Brand as PortalBrand, CommercialProduct as PortalCommercialProduct};
use diesel::PgConnection;
use web_common_traits::database::{BoundedRead, Insertable, InsertableVariant};

use super::get_user;
use crate::codegen::InstrumentModel as DirectusInstrumentModel;

/// Inserts missing instrument models into the portal database.
///
/// # Arguments
///
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the portal database connection.
///
/// # Errors
///
/// * If the insertion fails, an error of type `error::Error` is returned.
pub(crate) fn insert_missing_instrument_models(
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    let directus_instrument_models =
        DirectusInstrumentModel::bounded_read(0, u16::MAX, directus_conn)?;
    for directus_instrument_model in directus_instrument_models {
        let _portal_product: PortalCommercialProduct = if let Some(portal_product) =
            PortalCommercialProduct::from_name(
                &directus_instrument_model.instrument_model,
                portal_conn,
            )? {
            portal_product
        } else {
            let directus_brand = directus_instrument_model.brand(directus_conn)?;
            let portal_brand = PortalBrand::from_name(&directus_brand.brand, portal_conn)?
                .ok_or_else(|| {
                    crate::error::Error::UnknownBrand(Box::from(directus_brand.clone()))
                })?;
            let directus_created_by =
                directus_instrument_model.user_created(directus_conn)?.ok_or_else(|| {
                    crate::error::Error::InstrumentModelWithMissingUser(Box::from(
                        directus_instrument_model.clone(),
                    ))
                })?;
            let directus_updated_by = directus_instrument_model
                .user_updated(directus_conn)?
                .unwrap_or_else(|| directus_created_by.clone());
            let created_at = directus_instrument_model.date_created.ok_or_else(|| {
                crate::error::Error::MissingDate(
                    "instrument_models".to_owned(),
                    "date_created".to_owned(),
                )
            })?;
            let updated_at = directus_instrument_model.date_updated.unwrap_or(created_at);

            let portal_created_by = get_user(&directus_created_by, directus_conn, portal_conn)?;
            let portal_updated_by = get_user(&directus_updated_by, directus_conn, portal_conn)?;

            // We need to insert the product first
            PortalCommercialProduct::new()
                .name(directus_instrument_model.instrument_model.clone())?
                .brand(portal_brand.id)?
                .created_by(portal_created_by.id)?
                .updated_by(portal_updated_by.id)?
                .updated_at(updated_at)?
                .created_at(created_at)?
                .insert(portal_created_by.id, portal_conn)?
        };

        let _directus_instrument_type = directus_instrument_model.instrument_type(directus_conn)?;

        todo!("Insert missing instrument model: {}", directus_instrument_model.instrument_model);
    }
    Ok(())
}
