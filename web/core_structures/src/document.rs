//! Submodule providing utilities to create and manage documents.
use diesel::connection::LoadConnection;
use web_common_traits::database::{InsertError, Insertable, InsertableVariant};

use crate::{
    Document, codegen::structs_codegen::tables::insertables::DocumentBuildable,
    tables::insertables::InsertableDocumentAttributes,
};

/// Returns the newly created photograph.
pub fn create_photograph<C: LoadConnection>(
    photograph: &[u8],
    user: &crate::User,
    conn: &mut C,
) -> Result<Document, InsertError<InsertableDocumentAttributes>>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <Document as diesel::associations::HasTable>::Table,
        <<Document as Insertable>::InsertableVariant as diesel::Insertable<
            <Document as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, Document>,
{
    let info = infer::get(photograph).expect("Failed to infer document type");

    // TODO: add validation for the photograph.

    // We begin a transaction where we insert the document and write it to the
    // database
    let document: Document = conn
        .transaction::<Document, InsertError<InsertableDocumentAttributes>, _>(|conn| {
            let document: Document = crate::Document::new()
                .mime_type(info.mime_type())?
                .created_by(user.id)?
                .insert(user.id, conn)?;

            // TODO: actually write the document to the file system
            // Using `document.id` as the file name

            Ok(document)
        })?;

    Ok(document)
}
