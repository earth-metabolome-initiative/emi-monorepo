//! Submodule to insert missing instruments present in the Directus database
//! but not in the Portal database.

use core_structures::{
    CommercialProduct as CommercialProductProduct, Instrument as PortalInstrument,
    InstrumentLocation as PortalInstrumentLocation, InstrumentModel as PortalInstrumentModel,
    InstrumentState as PortalInstrumentState,
};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{AsyncRead, Insertable, InsertableVariant},
    prelude::Builder,
};

use super::{get_room, get_user};
use crate::codegen::Instrument as DirectusInstrument;

/// Inserts missing instruments into the Portal database
/// that are present in the Directus database.
///
/// # Arguments
///
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the Portal database connection.
///
/// # Errors
///
/// * If the database operations fail, an error is returned.
pub(crate) async fn insert_missing_instruments(
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let directus_instruments = DirectusInstrument::load_all(directus_conn).await?;
    for directus_instrument in directus_instruments {
        let user_created =
            directus_instrument.user_created(directus_conn).await?.ok_or_else(|| {
                crate::error::Error::InstrumentWithMissingUser(Box::from(
                    directus_instrument.clone(),
                ))
            })?;
        let user_updated = directus_instrument
            .user_updated(directus_conn)
            .await?
            .unwrap_or_else(|| user_created.clone());

        let created_by = get_user(&user_created, directus_conn, portal_conn).await?;
        let updated_by = get_user(&user_updated, directus_conn, portal_conn).await?;
        let created_at = directus_instrument.date_created.ok_or(
            crate::error::Error::MissingDate("instruments".to_owned(), "date_created".to_owned()),
        )?;
        let updated_at = directus_instrument.date_updated.unwrap_or(created_at);

        let instrument_state =
            PortalInstrumentState::from_name(&directus_instrument.status, portal_conn)
                .await?
                .ok_or_else(|| {
                    crate::error::Error::UnknownInstrumentState(directus_instrument.status.clone())
                })?;
        let directus_instrument_model = directus_instrument.instrument_model(directus_conn).await?;
        let portal_product = CommercialProductProduct::from_name(
            &directus_instrument_model.instrument_model,
            portal_conn,
        )
        .await?
        .ok_or_else(|| {
            crate::error::Error::UnknownInstrumentModel(Box::from(
                directus_instrument_model.clone(),
            ))
        })?;

        let portal_instrument_model =
            PortalInstrumentModel::from_id(portal_conn, &portal_product).await.map_err(|_| {
                crate::error::Error::UnknownInstrumentModel(Box::from(
                    directus_instrument_model.clone(),
                ))
            })?;

        let portal_instrument = PortalInstrument::new()
            .created_by(created_by.id)?
            .updated_by(updated_by.id)?
            .created_at(created_at)?
            .updated_at(updated_at)?
            .instrument_state_id(instrument_state.id)?
            .instrument_model_id(portal_instrument_model.id)?
            .qrcode(directus_instrument.uuid_instrument)?
            .build()?
            .insert(&created_by.id, portal_conn)
            .await?;

        let directus_room = directus_instrument.instrument_location(directus_conn).await?;
        let portal_room = get_room(&directus_room, directus_conn, portal_conn).await?;

        let _instrument_location = PortalInstrumentLocation::new()
            .instrument_id(portal_instrument.id)?
            .room_id(portal_room.id)?
            .created_by(created_by.id)?
            .created_at(created_at)?
            .build()?
            .insert(&created_by.id, portal_conn)
            .await?;
    }
    Ok(())
}
