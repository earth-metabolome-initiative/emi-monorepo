//! Submodule providing an utility to retrieve or insert an address.

use core_structures::{Address as PortalAddress, City as PortalCity, Country as PortalCountry};
use diesel_async::AsyncPgConnection;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use crate::codegen::Address as DirectusAddress;

/// Returns (potentially newly created) address for a Directus address.
pub(crate) async fn get_address(
    directus_address: &DirectusAddress,
    portal_conn: &mut AsyncPgConnection,
) -> Result<PortalAddress, crate::error::Error> {
    let country = PortalCountry::from_name(&directus_address.country, portal_conn)
        .await?
        .ok_or_else(|| crate::error::Error::UnknownCountry(directus_address.country.clone()))?;

    let city = match PortalCity::from_name(&directus_address.city, portal_conn).await?.pop() {
        Some(city) => city,
        None => {
            // We need to insert the city
            PortalCity::new()
                .name(directus_address.city.clone())?
                .iso(country.iso)?
                .build()?
                .backend_insert(portal_conn)
                .await?
        }
    };

    if let Some(address) = PortalAddress::from_city_id_and_street_name_and_street_number(
        &city.id,
        &directus_address.street,
        &directus_address.street_number,
        portal_conn,
    )
    .await?
    {
        Ok(address)
    } else {
        // Otherwise we need to insert the address
        Ok(PortalAddress::new()
            .city_id(city.id)?
            .street_name(directus_address.street.clone())?
            .street_number(directus_address.street_number.clone())?
            .postal_code(directus_address.postal_code.clone())?
            .geolocation(match directus_address.geolocation.clone() {
                postgis_diesel::types::GeometryContainer::Point(geolocation) => geolocation,
                _ => {
                    return Err(crate::error::Error::InvalidGeolocation(
                        directus_address.geolocation.clone(),
                    ));
                }
            })?
            .build()?
            .backend_insert(portal_conn)
            .await?)
    }
}
