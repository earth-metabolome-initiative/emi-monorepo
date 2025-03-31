mod codegen;
mod error;
use codegen::diesel_codegen::tables::directus_sessions;
use codegen::{Brand as DirectusBrand, DirectusUser};
use core_structures::{Brand as PortalBrand, BrandState as PortalBrandState, User, UserEmail};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use web_common_traits::database::{Insertable, InsertableBuilder, InsertableVariant, Loadable};
use web_common_traits::prelude::Builder;

const DIRECTUS_DATABASE_NAME: &str = "directus";
const DIRECTUS_DATABASE_PASSWORD: &str = "directus_dbgi";
const DIRECTUS_DATABASE_USER: &str = "directus";
const DIRECTUS_DATABASE_PORT: u16 = 5434;
const DIRECTUS_HOSTNAME: &str = "134.21.20.118";
const DIRECTUS_DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DIRECTUS_DATABASE_USER}:{DIRECTUS_DATABASE_PASSWORD}@{DIRECTUS_HOSTNAME}:{DIRECTUS_DATABASE_PORT}/{DIRECTUS_DATABASE_NAME}",
);

const PORTAL_DATABASE_NAME: &str = "development.db";
const PORTAL_DATABASE_PASSWORD: &str = "password";
const PORTAL_DATABASE_USER: &str = "user";
const PORTAL_DATABASE_PORT: u16 = 15032;
const PORTAL_DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{PORTAL_DATABASE_USER}:{PORTAL_DATABASE_PASSWORD}@localhost:{PORTAL_DATABASE_PORT}/{PORTAL_DATABASE_NAME}",
);

async fn get_user(
    directus_user: &DirectusUser,
    portal_conn: &mut AsyncPgConnection,
) -> Result<User, error::Error> {
    let email: &str =
        directus_user.email.as_deref().ok_or(error::Error::MissingEmail(directus_user.id))?;
    if let Some(stored_email) = UserEmail::from_email(email, portal_conn).await? {
        return Ok(stored_email.created_by(portal_conn).await?);
    }

    let new_user = User::new()
        .first_name(
            directus_user
                .first_name
                .clone()
                .ok_or(error::Error::MissingFirstName(directus_user.id))?,
        )?
        .last_name(
            directus_user
                .last_name
                .clone()
                .ok_or(error::Error::MissingLastName(directus_user.id))?,
        )?
        .build()?
        .insert(portal_conn)
        .await?;

    let _new_email = UserEmail::new()
        .created_by(new_user.id)?
        .email(directus_user.email.clone().ok_or(error::Error::MissingEmail(directus_user.id))?)?
        .build()?
        .insert(portal_conn)
        .await?;

    Ok(new_user)
}

/// Inserts missing brands into the portal database.
///
/// # Arguments
///
/// * `directus_brands` - A slice of `DirectusBrand` objects to be inserted.
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the portal database connection.
///
/// # Errors
///
/// * If the insertion fails, an error of type `error::Error` is returned.
///
async fn insert_missing_brands(
    directus_brands: &[DirectusBrand],
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), error::Error> {
    for directus_brand in directus_brands {
        if PortalBrand::from_name(&directus_brand.brand, portal_conn).await?.is_some() {
            continue;
        }

        let brand_state = PortalBrandState::from_name(&directus_brand.status, portal_conn)
            .await?
            .ok_or(error::Error::UnknownBrandStatus(directus_brand.status.clone()))?;
        let directus_created_by =
            directus_brand.user_created(directus_conn).await?.ok_or(error::Error::MissingUser)?;
        let portal_created_by = get_user(&directus_created_by, portal_conn).await?;
        let directus_updated_by =
            directus_brand.user_updated(directus_conn).await?.ok_or(error::Error::MissingUser)?;
        let portal_updated_by = get_user(&directus_updated_by, portal_conn).await?;

        let _portal_brand = PortalBrand::new()
            .brand_state_id(brand_state.id)?
            .created_by(portal_created_by.id)?
            .created_at(directus_brand.date_created.ok_or(error::Error::MissingDate)?)?
            .updated_at(directus_brand.date_updated.ok_or(error::Error::MissingDate)?)?
            .updated_by(portal_updated_by.id)?
            .name(directus_brand.brand.clone())?
            .build()?
            .insert(portal_conn).await?;
    }

    let portal_brands = PortalBrand::load_all(portal_conn).await?;

    assert!(portal_brands.len() >= directus_brands.len());

    Ok(())
}

/// Executes the data migration from the Directus database to the portal database.
async fn transact_migration(
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), error::Error> {
    let directus_users = DirectusUser::load_all(directus_conn).await?;
    for directus_user in directus_users {
        let _portal_user = get_user(&directus_user, portal_conn).await?;
    }

    let directus_brands = DirectusBrand::load_all(directus_conn).await?;
    insert_missing_brands(&directus_brands, directus_conn, portal_conn).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let mut directus_conn = AsyncPgConnection::establish(DIRECTUS_DATABASE_URL).await?;
    let mut portal_conn = AsyncPgConnection::establish(PORTAL_DATABASE_URL).await?;

    portal_conn
        .transaction(|portal_conn| {
            Box::pin(async move { transact_migration(&mut directus_conn, portal_conn).await })
        })
        .await?;

    Ok(())
}
