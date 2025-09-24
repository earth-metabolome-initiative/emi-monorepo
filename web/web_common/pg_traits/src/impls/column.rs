//! Submodule implementing traits for the [`Column`](pg_diesel::models::Column)
//! model.

use pg_diesel::models::Column;

use crate::AssociatedType;

impl AssociatedType for Column {
    fn associated_type<'required_crate>(
        &self,
        crates: &'required_crate [crate::RequiredCrate],
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<&'required_crate crate::RequiredType>, diesel::result::Error> {
        self.pg_type(conn)?.associated_type(crates, conn)
    }
}
