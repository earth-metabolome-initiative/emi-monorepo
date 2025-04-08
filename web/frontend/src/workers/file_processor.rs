use web_common::api::ApiError;
use yew_agent::prelude::*;

use crate::components::forms::inputs::file_like::FileLike;

#[oneshot(FileProcessor)]
pub async fn file_processor<Data: FileLike>(data: Vec<u8>) -> Result<Data, ApiError> {
    log::info!("Processing file data");
    let result = Data::try_from_bytes(data).await;
    log::info!("File data processed");
    result
}
