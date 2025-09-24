//! Submodule implementing traits for the [`PgType`](pg_diesel::models::PgType)
//! model.

use pg_diesel::models::PgType;

use crate::AssociatedType;

impl AssociatedType for PgType {
    fn associated_type<'required_crate>(
        &self,
        crates: &'required_crate [crate::RequiredCrate],
        _conn: &mut diesel::PgConnection,
    ) -> Result<Option<&'required_crate crate::RequiredType>, diesel::result::Error> {
        for required_crate in crates {
            if let Some(compatible_type) = required_crate.compatible_type(&self.typname) {
                return Ok(Some(compatible_type));
            }
        }
        Ok(None)
    }
}
