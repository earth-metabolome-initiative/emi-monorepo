//! Submodule implementing traits for the [`Table`](pg_diesel::models::Table) model.

use pg_diesel::models::{Table, Column};

use crate::Supports;

impl<TraitMarker> Supports<TraitMarker> for Table
where
    Column: Supports<TraitMarker>,
{
    type Error = Column::Error;

    fn supports(&self, conn: &mut PgConnection) -> Result<bool, Self::Error> {
        for column in self.columns(conn)? {
			if !column.supports::<TraitMarker>(conn)? {
				return Ok(false);
			}
		}
		Ok(true)
    }
}
