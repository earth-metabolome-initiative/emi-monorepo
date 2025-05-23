//! Submodule providing utilities to create and manage documents.
use common_traits::prelude::Builder;
use diesel_async::AsyncConnection;
use web_common_traits::database::{InsertError, Insertable, InsertableVariant};

use crate::{Document, tables::insertables::InsertableDocumentAttributes};

#[cfg(feature = "postgres")]
/// Returns the newly created photograph.
pub async fn create_photograph(
    photograph: &[u8],
    user: &crate::User,
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<Document, InsertError<InsertableDocumentAttributes>> {
    let info = infer::get(photograph).expect("Failed to infer document type");

    // TODO: add validation for the photograph.

    // We begin a transaction where we insert the document and write it to the
    // database
    let document: Document = conn
        .transaction::<Document, InsertError<InsertableDocumentAttributes>, _>(|conn| {
            Box::pin(async move {
                let document: Document = crate::Document::new()
                    .mime_type(info.mime_type())?
                    .created_by(user.id)?
                    .build()?
                    .insert(&user.id, conn)
                    .await?;

                // TODO: actually write the document to the file system
                // Using `document.id` as the file name

                Ok(document)
            })
        })
        .await?;

    Ok(document)
}
