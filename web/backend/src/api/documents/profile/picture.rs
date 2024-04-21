//! Submodule for profile picture related endpoints.

use std::fs::File;

use actix_web::{get, HttpResponse, Responder};
use std::io::Read;
use uuid::Uuid;
use web_common::custom_validators::ImageSize;

use crate::{models::Document, nested_models::NestedDocument};

#[get("/picture/{user_id}/{image_size}.png")]
pub async fn user_picture_handler(
    path: actix_web::web::Path<(Uuid, ImageSize)>,
    pool: actix_web::web::Data<crate::DBPool>,
) -> impl Responder {
    let (user_id, image_size) = path.into_inner();

    // We get the database pool connection
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    // Secondarily, we build the expected path
    let path = format!(
        "{}/{}/{}.png",
        web_common::api::documents::profile::picture::FULL_ENDPOINT,
        user_id,
        image_size
    );
    // Then, we find the document corresponding to the requested path
    let document = match Document::from_path(&path, &mut conn) {
        Ok(document) => document,
        Err(_) => return actix_web::HttpResponse::NotFound().finish(),
    };
    // Now with the document, we can obtain the complete DocumentView
    let document = match NestedDocument::get(document.id, &mut conn) {
        Ok(document) => document,
        Err(_) => return actix_web::HttpResponse::InternalServerError().finish(),
    };
    // From the document view, we obtain the path to the image.
    let image_path = document.internal_path();

    // Finally, we return the image
    if let Ok(mut file) = File::open(image_path) {
        // Read the image data into a buffer
        let mut buffer = Vec::new();
        if let Ok(_) = file.read_to_end(&mut buffer) {
            // Set the content type to image/jpeg
            HttpResponse::Ok()
                .content_type(document.format.mime_type)
                // Stream the image data as the response body
                .body(buffer)
        } else {
            // Error reading the image data
            HttpResponse::InternalServerError().finish()
        }
    } else {
        // Error opening the image file
        HttpResponse::NotFound().finish()
    }
}
