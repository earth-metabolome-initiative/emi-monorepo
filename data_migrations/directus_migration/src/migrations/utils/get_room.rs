//! Submodule providing an utility to retrieve or insert a room.

use core_structures::{Room as PortalRoom, tables::insertables::RoomSettable};
use diesel::{OptionalExtension, PgConnection};
use directus_codegen::Room as DirectusRoom;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use super::{get_address, get_user};

pub(crate) fn get_room(
    directus_room: &DirectusRoom,
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> anyhow::Result<PortalRoom> {
    if let Some(room) = PortalRoom::from_qrcode(directus_room.qr_code, portal_conn).optional()? {
        return Ok(room);
    }

    let directus_address = directus_room.address(directus_conn)?;
    let address = get_address(&directus_address, portal_conn)?;
    let directus_created_by =
        directus_room.user_created(directus_conn)?.expect("Room must have a creator");
    let portal_created_by = get_user(&directus_created_by, directus_conn, portal_conn)?;
    let directus_updated_by =
        directus_room.user_updated(directus_conn)?.unwrap_or_else(|| directus_created_by.clone());
    let portal_updated_by = get_user(&directus_updated_by, directus_conn, portal_conn)?;
    let created_at = directus_room.date_created.expect("Room must have creation date");
    let updated_at = directus_room.date_updated.unwrap_or(created_at);

    // We need to create the room.
    Ok(PortalRoom::new()
        .name(directus_room.room_name.clone())?
        .description(directus_room.comment.clone())?
        .qrcode(directus_room.qr_code)?
        .addresses(address.id)?
        .geolocation(match directus_room.geolocation.clone() {
            postgis_diesel::types::GeometryContainer::Point(geolocation) => geolocation,
            _ => {
                unreachable!(
                    "Directus room has invalid geolocation: {:?}",
                    directus_room.geolocation
                );
            }
        })?
        .created_by(portal_created_by.id)?
        .updated_by(portal_updated_by.id)?
        .created_at(created_at)?
        .updated_at(updated_at)?
        .insert(portal_created_by.id, portal_conn)?)
}
