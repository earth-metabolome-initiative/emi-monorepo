//! Submodule providing an utility to retrieve or insert an address.

use core_structures::{
    Address as PortalAddress, City as PortalCity, Country as PortalCountry,
    tables::insertables::{AddressSettable, CitySettable},
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::codegen::Address as DirectusAddress;

/// Returns (potentially newly created) address for a Directus address.
pub(crate) fn get_address(
    directus_address: &DirectusAddress,
    portal_conn: &mut PgConnection,
) -> anyhow::Result<PortalAddress> {
    let country = PortalCountry::from_name(&directus_address.country, portal_conn)?;

    let city = match PortalCity::from_name(&directus_address.city, portal_conn)?.pop() {
        Some(city) => city,
        None => {
            // We need to insert the city
            PortalCity::new()
                .name(directus_address.city.clone())?
                .iso(country.iso)?
                .insert(0, portal_conn)?
        }
    };

    if let Some(address) = PortalAddress::from_city_id_and_street_name_and_house_number(
        city.id,
        &directus_address.street,
        &directus_address.street_number,
        portal_conn,
    )
    .optional()?
    {
        Ok(address)
    } else {
        // Otherwise we need to insert the address
        Ok(PortalAddress::new()
            .city(city.id)?
            .street_name(directus_address.street.clone())?
            .house_number(directus_address.street_number.clone())?
            .postal_code(directus_address.postal_code.clone())?
            .geolocation(match directus_address.geolocation.clone() {
                postgis_diesel::types::GeometryContainer::Point(geolocation) => geolocation,
                _ => {
                    unreachable!(
                        "Directus address has invalid geolocation: {:?}",
                        directus_address.geolocation
                    );
                }
            })?
            .insert(0, portal_conn)?)
    }
}
