//! Submodule providing a generic web worker for processing files
//! asynchronously.
use common_traits::try_from_async::TryFromAsync;
use yew_agent::prelude::*;

use crate::components::forms::inputs::file_like::FileLike;

#[oneshot(FileProcessor)]
pub async fn file_processor<Data: FileLike>(
    data: Vec<u8>,
) -> Result<Data, <Data as TryFromAsync<Vec<u8>>>::Error> {
    Data::try_from_bytes(data).await
}
