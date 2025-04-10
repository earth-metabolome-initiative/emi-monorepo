//! Submodule providing an utility to retrieve or insert a room.

use core_structures::Room as PortalRoom;
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{Insertable, InsertableVariant},
    prelude::Builder,
};

use super::{get_address, get_user};
use crate::codegen::Room as DirectusRoom;

pub(crate) async fn get_room(
    directus_room: &DirectusRoom,
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<PortalRoom, crate::error::Error> {
    if let Some(room) = PortalRoom::from_qrcode(&directus_room.qr_code, portal_conn).await? {
        Ok(room)
    } else {
        let directus_address = directus_room.address(directus_conn).await?;
        let address = get_address(&directus_address, portal_conn).await?;
        let directus_created_by =
            directus_room.user_created(directus_conn).await?.ok_or_else(|| {
                crate::error::Error::RoomWithMissingUser(Box::from(directus_room.clone()))
            })?;
        let portal_created_by = get_user(&directus_created_by, directus_conn, portal_conn).await?;
        let directus_updated_by = directus_room
            .user_updated(directus_conn)
            .await?
            .unwrap_or_else(|| directus_created_by.clone());
        let portal_updated_by = get_user(&directus_updated_by, directus_conn, portal_conn).await?;
        let created_at = directus_room.date_created.ok_or_else(|| {
            crate::error::Error::MissingDate("Rooms".to_owned(), "date_created".to_owned())
        })?;
        let updated_at = directus_room.date_updated.unwrap_or(created_at);

        // We need to create the room.
        Ok(PortalRoom::new()
            .name(directus_room.room_name.clone())?
            .description(directus_room.comment.clone())?
            .qrcode(directus_room.qr_code.clone())?
            .addresses_id(address.id)?
            .geolocation(match directus_room.geolocation.clone() {
                postgis_diesel::types::GeometryContainer::Point(geolocation) => geolocation,
                _ => {
                    return Err(crate::error::Error::InvalidGeolocation(
                        directus_room.geolocation.clone(),
                    ));
                }
            })?
            .created_by(portal_created_by.id)?
            .updated_by(portal_updated_by.id)?
            .created_at(created_at)?
            .updated_at(updated_at)?
            .build()?
            .insert(&portal_created_by.id, portal_conn)
            .await?)
    }
}
