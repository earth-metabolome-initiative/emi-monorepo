//! Submodule for inserting the missing instrument models which
//! are present in the Directus database but not in the Portal database.

use core_structures::{
    Brand as PortalBrand, CommercialProduct as PortalCommercialProduct,
    InstrumentCategory as PortalInstrumentCategory, InstrumentModel as PortalInstrumentModel,
};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{AsyncRead, Insertable, InsertableVariant},
    prelude::Builder,
};

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
pub(crate) async fn insert_missing_instrument_models(
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let directus_instrument_models = DirectusInstrumentModel::load_all(directus_conn).await?;
    for directus_instrument_model in directus_instrument_models {
        let portal_product: PortalCommercialProduct = if let Some(portal_product) =
            PortalCommercialProduct::from_name(
                &directus_instrument_model.instrument_model,
                portal_conn,
            )
            .await?
        {
            portal_product
        } else {
            let directus_brand = directus_instrument_model.brand(directus_conn).await?;
            let portal_brand =
                PortalBrand::from_name(&directus_brand.brand, portal_conn).await?.ok_or_else(
                    || crate::error::Error::UnknownBrand(Box::from(directus_brand.clone())),
                )?;
            let directus_created_by =
                directus_instrument_model.user_created(directus_conn).await?.ok_or_else(|| {
                    crate::error::Error::InstrumentModelWithMissingUser(Box::from(
                        directus_instrument_model.clone(),
                    ))
                })?;
            let directus_updated_by = directus_instrument_model
                .user_updated(directus_conn)
                .await?
                .unwrap_or_else(|| directus_created_by.clone());
            let created_at = directus_instrument_model.date_created.ok_or_else(|| {
                crate::error::Error::MissingDate(
                    "instrument_models".to_owned(),
                    "date_created".to_owned(),
                )
            })?;
            let updated_at = directus_instrument_model.date_updated.unwrap_or(created_at);

            let portal_created_by =
                get_user(&directus_created_by, directus_conn, portal_conn).await?;
            let portal_updated_by =
                get_user(&directus_updated_by, directus_conn, portal_conn).await?;

            // We need to insert the product first
            PortalCommercialProduct::new()
                .name(directus_instrument_model.instrument_model.clone())?
                .brand_id(portal_brand.id)?
                .created_by(portal_created_by.id)?
                .updated_by(portal_updated_by.id)?
                .updated_at(updated_at)?
                .created_at(created_at)?
                .build()?
                .insert(&portal_created_by.id, portal_conn)
                .await?
        };

        let directus_instrument_type =
            directus_instrument_model.instrument_type(directus_conn).await?;
        let portal_instrument_category = PortalInstrumentCategory::from_name(
            directus_instrument_type.instrument_type.as_ref().ok_or_else(|| {
                crate::error::Error::MissingInstrumentTypeName(Box::from(
                    directus_instrument_type.clone(),
                ))
            })?,
            portal_conn,
        )
        .await?
        .ok_or_else(|| {
            crate::error::Error::UnknownInstrumentCategory(Box::from(
                directus_instrument_type.clone(),
            ))
        })?;

        todo!("Register the instrument model categories!");

        let _portal_instrument_model = PortalInstrumentModel::new()
            .id(portal_product.id)?
            .created_by(portal_product.created_by)?
            .updated_by(portal_product.updated_by)?
            .updated_at(portal_product.updated_at)?
            .created_at(portal_product.created_at)?
            .build()?
            .insert(&portal_product.created_by, portal_conn)
            .await?;
    }
    Ok(())
}
