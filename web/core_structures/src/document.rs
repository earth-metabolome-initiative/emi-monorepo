//! Submodule providing utilities to create and manage documents.
use diesel::connection::LoadConnection;
use web_common_traits::database::{InsertError, Insertable, InsertableVariant};

use crate::{
    Photograph, codegen::structs_codegen::tables::insertables::AssetSettable,
    tables::insertables::PhotographAttribute,
};

/// Returns the newly created photograph.
pub fn create_photograph<C: LoadConnection>(
    photograph: &[u8],
    user: &crate::User,
    conn: &mut C,
) -> Result<Photograph, InsertError<PhotographAttribute>>
where
    <Photograph as Insertable>::InsertableBuilder:
        InsertableVariant<C, Attribute = PhotographAttribute, Row = Photograph, UserId = i32>,
{
    let info = infer::get(photograph).expect("Failed to infer document type");

    // TODO: add validation for the photograph.

    // We begin a transaction where we insert the document and write it to the
    // database
    let document: Photograph = conn
        .transaction::<Photograph, InsertError<PhotographAttribute>, _>(|conn| {
            let document: Photograph =
                crate::Photograph::new().created_by(user.id)?.insert(user.id, conn)?;

            // TODO: actually write the document to the file system
            // Using `document.id` as the file name

            Ok(document)
        })?;

    Ok(document)
}
