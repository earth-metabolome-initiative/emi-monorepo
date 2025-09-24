//! Submodule implementing traits for the [`Table`](pg_diesel::models::Table)
//! model.

use pg_diesel::models::Table;

use crate::{Supports, Trait};

impl Supports for Table {
    fn supports(
        &self,
        supported_trait: Trait,
        crates: &[crate::RequiredCrate],
        conn: &mut diesel::PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        for column in self.columns(conn)? {
            if !column.supports(supported_trait, crates, conn)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
