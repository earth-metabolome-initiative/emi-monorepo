//! Submodule implementing traits for the
//! [`PgAttribute`](pg_diesel::models::PgAttribute) model.

use pg_diesel::models::PgAttribute;

use crate::AssociatedType;

impl AssociatedType for PgAttribute {
    fn associated_type<'required_crate>(
        &self,
        crates: &'required_crate [crate::RequiredCrate],
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<&'required_crate crate::RequiredType>, diesel::result::Error> {
        self.pg_type(conn)?.associated_type(crates, conn)
    }
}
