//! Submodule providing an utility to retrieve or insert an address.

use crate::codegen::Address as DirectusAddress;
use core_structures::{Address as PortalAddress, City as PortalCity, Country as PortalCountry};
use diesel_async::AsyncPgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};
use web_common_traits::prelude::Builder;

/// Returns (potentially newly created) address for a Directus address.
pub async fn get_address(
    directus_address: &DirectusAddress,
    portal_conn: &mut AsyncPgConnection,
) -> Result<PortalAddress, crate::error::Error> {
    let country = PortalCountry::from_name(&directus_address.country, portal_conn)
        .await?
        .ok_or_else(|| crate::error::Error::UnknownCountry(directus_address.country.clone()))?;

    let city = if let Some(city) =
        PortalCity::from_code(&directus_address.city_code, portal_conn).await?
    {
        city
    } else {
        // We need to insert the city
        PortalCity::new()
            .name(directus_address.city.clone())?
            .code(directus_address.city_code.clone())?
            .iso(country.iso.clone())?
            .build()?
            .insert(portal_conn)
            .await?
    };

    if let Some(address) = PortalAddress::from_iso_and_city_code_and_street_name_and_street_number(
        &country.iso,
        &city.code,
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
            .iso(country.iso.clone())?
            .city_code(directus_address.city_code.clone())?
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
            .insert(portal_conn)
            .await?)
    }
}
