//! Submodule to verify that all Instrument Types defined in the
//! Directus database are also available in the Portal database.

use core_structures::InstrumentCategory as PortalInstrumentCategory;
use diesel_async::AsyncPgConnection;
use web_common_traits::prelude::AsyncBoundedRead;

use crate::codegen::InstrumentType as DirectusInstrumentCategory;

/// Operations to ensure that the Instrument Types defined in the
/// Directus database are also available in the Portal database.
///
/// # Arguments
///
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the Portal database connection.
///
/// # Errors
///
/// * If the Directus database connection fails, an error is returned.
/// * If the Portal database connection fails, an error is returned.
/// * If the Instrument Types in the Directus database do not match those in the
///   Portal database, an error is returned.
pub async fn ensure_instrument_categories_compatibility(
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let instrument_types = DirectusInstrumentCategory::read_all_async(directus_conn).await?;

    for instrument_type in instrument_types {
        let portal_instrument_type = PortalInstrumentCategory::from_name(
            instrument_type.instrument_type.as_ref().ok_or_else(|| {
                crate::error::Error::MissingInstrumentTypeName(Box::from(instrument_type.clone()))
            })?,
            portal_conn,
        )
        .await?;

        if portal_instrument_type.is_none() {
            return Err(crate::error::Error::UnknownInstrumentCategory(Box::from(instrument_type)));
        }
    }

    Ok(())
}
