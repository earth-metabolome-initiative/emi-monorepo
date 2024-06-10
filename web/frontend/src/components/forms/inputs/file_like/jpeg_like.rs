use super::*;
use base64::engine::general_purpose;
use base64::Engine;
use web_common::types::JPEG;

impl FileLike for JPEG {
    const FORMATS: &'static [GenericFileFormat] = &[GenericFileFormat::JPEG];

    fn file_size(&self) -> u64 {
        let data: &[u8] = self.as_ref();
        data.len() as u64
    }

    fn preview(&self) -> Html {
        let data: &[u8] = self.as_ref();

        let url = format!(
            "data:image/jpeg;base64,{}",
            general_purpose::STANDARD.encode(data)
        );
        html! {
            <img src={url} class="preview" />
        }
    }
}

impl super::TryFromCallback<web_sys::File> for JPEG {
    fn try_from_callback<C>(
        file: web_sys::File,
        callback: C,
    ) -> Result<(), web_common::api::ApiError>
    where
        C: FnOnce(Result<Self, web_common::api::ApiError>) + 'static,
    {
        use wasm_bindgen::JsCast;

        // Create a reader
        let reader = web_sys::FileReader::new()
            .map_err(|_| vec!["Unable to open File Reader".to_string()])?;

        let on_load =
            wasm_bindgen::closure::Closure::once_into_js(move |event: web_sys::ProgressEvent| {
                let reader = event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::FileReader>()
                    .unwrap();

                let result = reader.result().unwrap();
                let data = js_sys::Uint8Array::new(&result);
                let data = data.to_vec();

                callback(Ok(data.into()));
            });

        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));

        // Read file contents
        reader
            .read_as_array_buffer(&file)
            .map_err(|_| vec!["Unable to read file.".to_string()])?;

        Ok(())
    }
}
