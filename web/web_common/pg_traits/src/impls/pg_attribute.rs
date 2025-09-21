//! Submodule implementing traits for the [`PgAttribute`](pg_diesel::models::PgAttribute) model.

use pg_diesel::models::{PgAttribute, PgType};

use crate::Supports;

impl<TraitMarker> Supports<TraitMarker> for PgAttribute
where
    PgType: Supports<TraitMarker>,
{
    type Error = PgType::Error;

    fn supports(&self, conn: &mut PgConnection) -> Result<bool, Self::Error> {
        self.pg_type(conn)?.supports(conn)
    }
}
