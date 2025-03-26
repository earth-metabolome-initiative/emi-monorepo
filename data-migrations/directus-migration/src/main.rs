mod codegen;
mod error;
use codegen::DirectusUser;
use core_structures::{User, UserEmail};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use web_common_traits::database::{Insertable, InsertableBuilder, InsertableVariant, Loadable};
use web_common_traits::prelude::Builder;

const DATABASE_NAME: &str = "directus";
const DATABASE_PASSWORD: &str = "directus_dbgi";
const DATABASE_USER: &str = "directus";
const DATABASE_PORT: u16 = 5434;
const HOSTNAME: &str = "134.21.20.118";
const DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@{HOSTNAME}:{DATABASE_PORT}/{DATABASE_NAME}",
);

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let mut conn = AsyncPgConnection::establish(DATABASE_URL).await?;
    let users = DirectusUser::load_all(&mut conn).await?;
    println!("We have loaded {} users", users.len());
    for user in users {
        let new_user = User::new()
            .first_name(user.first_name.ok_or(error::Error::MissingFirstName(user.id))?)?
            .last_name(user.last_name.ok_or(error::Error::MissingLastName(user.id))?)?
            .build()?
            .insert(&mut conn).await?;
    }
    Ok(())
}
