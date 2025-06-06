//! Submodule to initialize the `reagents` in the database.

use core_structures::{Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub const BOTTLE: &str = "Bottle";
pub const BOX: &str = "Box";
pub const SPRAYER: &str = "Sprayer";
pub const POLYSTYRENE_BOX: &str = "Polystyrene Box";

pub(crate) fn init_containers(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    let container = core_structures::Trackable::new()
        .name(Some("Container".to_owned()))
        .unwrap()
        .description(Some("Containers used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _bottle = Trackable::new()
        .name(Some(BOTTLE.to_owned()))
        .unwrap()
        .description(Some("Bottle, a common container for liquids".to_owned()))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let r#box = Trackable::new()
        .name(Some(BOX.to_owned()))
        .unwrap()
        .description(Some("Box, a common container for solids or liquids".to_owned()))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _polystyrene_box = Trackable::new()
        .name(Some(POLYSTYRENE_BOX.to_owned()))
        .unwrap()
        .description(Some(
            "Polystyrene box, a container typically used for liquid nitrogen".to_owned(),
        ))
        .unwrap()
        .parent_id(Some(r#box.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _sprayer = Trackable::new()
        .name(Some(SPRAYER.to_owned()))
        .unwrap()
        .description(Some(
            "Sprayer, a container for liquids, usually Ethanol, used in laboratory or field procedures".to_owned(),
        ))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    Ok(())
}
