use yew::prelude::*;
mod jpeg_like;
use std::{fmt::Debug, rc::Rc};

use web_common::file_formats::GenericFileFormat;

/// Returns a human-readable string representing the size of a file.
pub fn human_readable_file_size(size: usize) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.0} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.0} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Trait defining an object which is File-like
pub trait FileLike:
    Clone + Debug + PartialEq + Unpin + for<'de> serde::Deserialize<'de> + 'static
{
    const FORMATS: &'static [GenericFileFormat];

    /// Returns the file size in bytes.
    fn file_size(&self) -> u64;

    fn preview(&self) -> Html;

    fn try_from_bytes(
        bytes: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<Self, ApiError>> + Send;

    /// Returns the file size in a human readable format.
    fn human_readable_file_size(&self) -> String {
        human_readable_file_size(self.file_size() as usize)
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FilePreviewProp<Data>
where
    Data: FileLike,
{
    pub file: Rc<Data>,
    pub delete: Callback<()>,
}

#[function_component(FilePreview)]
/// A component to display a preview of the file that have been selected.
pub fn file_preview<Data: FileLike>(props: &FilePreviewProp<Data>) -> Html {
    let on_click = {
        let on_delete = props.delete.clone();
        Callback::from(move |click: MouseEvent| {
            click.stop_immediate_propagation();
            click.prevent_default();
            on_delete.emit(());
        })
    };

    let class = format!("file-preview-item");

    html! {
        <div class={class}>
            {props.file.preview()}
            <div class="delete">
                <i class="fas fa-times" onclick={on_click}></i>
            </div>
        </div>
    }
}

// #[derive(Clone, PartialEq, Properties)]
// pub struct FilePreviewItemProp<Data>
// where
//     Data: 'static + Clone + PartialEq,
// {
//     pub file: Data,
//     #[prop_or_default]
//     pub on_delete: Callback<()>,
//     pub large: bool,
//     #[prop_or_default]
//     pub hiding: bool,
// }

// impl<Data> FilePreviewItemProp<Data>
// where
//     Data: 'static + Clone + PartialEq,
// {
//     pub fn path(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(file) => Some(file.name()),
//             FileLike::Url(_) => None,
//         }
//     }

//     pub fn name(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(file) =>
// Some(file.name().split('/').last().unwrap().to_string()),
// FileLike::Url(_) => None,         }
//     }

//     pub fn extension(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(file) => {
//                 let path = file.name();
//                 let extension = path.split('.').last();
//                 extension.map(|s| s.to_string())
//             }
//             FileLike::Url(url) => {
//                 let extension = url.split('.').last();
//                 extension.map(|s| s.to_string())
//             }
//         }
//     }

//     pub fn size(&self) -> Option<u64> {
//         match &self.file {
//             FileLike::File(file) => Some(file.size() as u64),
//             FileLike::Url(_) => None,
//         }
//     }

//     pub fn is_image(&self) -> bool {
//         match &self.file {
//             FileLike::File(file) => file.type_().starts_with("image"),
//             FileLike::Url(url) => {
//                 url.ends_with(".png")
//                     || url.ends_with(".jpg")
//                     || url.ends_with(".jpeg")
//                     || url.ends_with(".gif")
//                     || url.ends_with(".webp")
//             }
//         }
//     }

//     pub fn is_pdf(&self) -> bool {
//         match &self.file {
//             FileLike::File(file) =>
// file.type_().starts_with("application/pdf"),             FileLike::Url(url)
// => url.ends_with(".pdf"),         }
//     }

//     pub fn last_modified(&self) -> Option<u64> {
//         match &self.file {
//             FileLike::File(file) => Some(file.last_modified() as u64),
//             FileLike::Url(_) => None,
//         }
//     }

//     pub fn metadata_hash(&self) -> u64 {
//         let mut hasher = DefaultHasher::new();
//         match &self.file {
//             FileLike::File(file) => {
//                 file.name().hash(&mut hasher);
//             }
//             FileLike::Url(url) => {
//                 url.hash(&mut hasher);
//             }
//         }
//         self.size().hash(&mut hasher);
//         self.last_modified().hash(&mut hasher);
//         hasher.finish()
//     }

//     pub fn human_readable_size(&self) -> Option<String> {
//         self.size().map(human_readable_size)
//     }

//     pub fn last_modified_date(&self) -> Option<String> {
//         self.last_modified().map(|last_modified| {
//             let date = js_sys::Date::new(&JsValue::from_f64(last_modified as
// f64));             let date = date.to_locale_string("en-US",
// &JsValue::undefined());             date.as_string().unwrap()
//         })
//     }

//     /// Returns a human-readable string representing the time elapsed since
// the file was last modified.     pub fn human_readable_modified_delta(&self)
// -> Option<String> {         let date = self.last_modified()?;
//         let date = js_sys::Date::new(&JsValue::from_f64(date as f64));
//         let now = js_sys::Date::new_0();
//         let delta = now.get_time() - date.get_time();
//         let delta = (delta as f64 / 1000.0) as u64;
//         Some(if delta < 60 {
//             format!("{} seconds ago", delta)
//         } else if delta < 60 * 60 {
//             format!("{} minutes ago", delta / 60)
//         } else if delta < 60 * 60 * 24 {
//             format!("{} hours ago", delta / (60 * 60))
//         } else if delta < 60 * 60 * 24 * 30 {
//             format!("{} days ago", delta / (60 * 60 * 24))
//         } else if delta < 60 * 60 * 24 * 30 * 12 {
//             format!("{} months ago", delta / (60 * 60 * 24 * 30))
//         } else {
//             format!("{} years ago", delta / (60 * 60 * 24 * 30 * 12))
//         })
//     }

//     /// Returns the file metadata when the variant is a File.
//     pub fn file_metadata(&self) -> Option<String> {
//         match &self.file {
//             FileLike::File(_) => Some(format!(
//                 "{}, {}, from {}",
//                 self.name()?,
//                 self.human_readable_size()?,
//                 self.human_readable_modified_delta()?
//             )),
//             FileLike::Url(_) => None,
//         }
//     }
// }

// pub struct ImagePreview {}

// impl Component for ImagePreview {
//     type Message = ();
//     type Properties = FilePreviewItemProp;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // We add an hash obtained from the file name and the associated
// informations, such         // as the file size and the last modified date, to
// the URL to make sure that the URL         // changes when the file changes,
// so that the image is reloaded when the file changes.

//         let hash = ctx.props().metadata_hash();

//         match &ctx.props().file {
//             FileLike::File(file) => {
//                 let url =
// web_sys::Url::create_object_url_with_blob(&file).unwrap();
// let url = format!("{}#{}", url, hash);                 html! {
//                     <img src={url} class="preview" />
//                 }
//             }
//             FileLike::Url(url) => {
//                 let url = format!("{}#{}", url, hash);
//                 html! {
//                     <img src={url} class="preview" />
//                 }
//             }
//         }
//     }
// }

// pub struct PDFPreview {}

// impl Component for PDFPreview {
//     type Message = ();
//     type Properties = FilePreviewItemProp;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // We add an hash obtained from the file name and the associated
// informations, such         // as the file size and the last modified date, to
// the URL to make sure that the URL         // changes when the file changes,
// so that the image is reloaded when the file changes.

//         let hash = ctx.props().metadata_hash();

//         match &ctx.props().file {
//             FileLike::File(file) => {
//                 let url =
// web_sys::Url::create_object_url_with_blob(&file).unwrap();
// let url = format!("{}#{}", url, hash);                 html! {
//                     <iframe src={url} class="preview"></iframe>
//                 }
//             }
//             FileLike::Url(url) => {
//                 let url = format!("{}#{}", url, hash);
//                 html! {
//                     <iframe src={url} class="preview"></iframe>
//                 }
//             }
//         }
//     }
// }

// pub struct TextualFilePreview {
//     text: String,
// }

// #[derive(Clone, PartialEq, Properties)]
// pub struct TextualFilePreviewProps {
//     pub file_props: FilePreviewItemProp,
//     #[prop_or(20)]
//     pub number_of_lines: usize,
// }

// pub enum TextualFilePreviewMessage {
//     Load(String),
// }

// impl Component for TextualFilePreview {
//     type Message = TextualFilePreviewMessage;
//     type Properties = TextualFilePreviewProps;

//     fn create(ctx: &Context<Self>) -> Self {
//         let file = match ctx.props().file_props.file.clone() {
//             FileLike::File(file) => file,
//             FileLike::Url(_) => {
//                 unreachable!("TextualFilePreview should only be used with
// files, not URLs.")             }
//         };
//         let reader = web_sys::FileReader::new().unwrap();
//         let link = ctx.link().clone();
//         // We read the first few lines of the file to display a preview of
// the file.

//         const CHUNK_SIZE: usize = 1024; // Adjust the chunk size as needed

//         let number_of_lines = ctx.props().number_of_lines;

//         let on_load = Closure::wrap(Box::new(move |event:
// web_sys::ProgressEvent| {             let mut lines_read = 0;
//             let mut text = String::new();
//             let reader = event
//                 .target()
//                 .unwrap()
//                 .dyn_into::<web_sys::FileReader>()
//                 .unwrap();
//             let result = reader.result().unwrap();
//             let data = js_sys::Uint8Array::new(&result);

//             let mut chunk = Vec::with_capacity(CHUNK_SIZE);
//             for i in 0..data.length() {
//                 chunk.push(data.get_index(i));
//                 if chunk.len() >= CHUNK_SIZE {
//                     let chunk_text = String::from_utf8_lossy(&chunk);

//                     for line in chunk_text.lines() {
//                         text.push_str(line);
//                         text.push('\n');
//                         lines_read += 1;
//                         if lines_read >= number_of_lines {
//                             break;
//                         }
//                     }

//                     if lines_read >= number_of_lines {
//                         break;
//                     }

//                     chunk.clear();
//                 }
//             }

//             if lines_read < number_of_lines {
//                 let remaining_chunk_text = String::from_utf8_lossy(&chunk);
//                 text.push_str(&remaining_chunk_text);
//                 lines_read += remaining_chunk_text.lines().count();
//             }

//             link.send_message(TextualFilePreviewMessage::Load(text));
//         }) as Box<dyn FnMut(_)>);

//         reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
//         on_load.forget();

//         reader.read_as_array_buffer(&file).unwrap();

//         Self {
//             text: String::new(),
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             TextualFilePreviewMessage::Load(text) => {
//                 self.text = text;
//                 true
//             }
//         }
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <div class="file-preview-item-thumbnail">
//                 <p>{&self.text}</p>
//             </div>
//         }
//     }
// }

// #[function_component(FilePreviewItem)]
// /// A component to display a preview of a single file of a list of files.
// pub fn file_preview_item<Data>(props: &FilePreviewItemProp<Data>) -> Html
// where
//     Data: 'static + Clone + PartialEq,
// {
//     let on_delete = props.on_delete.clone();
//     let hiding = use_state(|| false);

//     let on_click = {
//         let hiding = hiding.clone();
//         Callback::from(move |click: MouseEvent| {
//             click.stop_immediate_propagation();
//             click.prevent_default();
//             hiding.set(true);
//             on_delete.emit(());
//         })
//     };

//     let thumbnail: Html = {
//         if props.is_image() {
//             html! { <ImagePreview file={props.file.clone()}
// large={props.large} /> }         } else if props.is_pdf() {
//             html! { <PDFPreview file={props.file.clone()} large={props.large}
// /> }         } else {
//             html! { <TextualFilePreview file_props={props.clone()} /> }
//         }
//     };

//     let class = format!(
//         "file-preview-item{}",
//         if props.hiding || *hiding {
//             " hiding"
//         } else {
//             ""
//         }
//     );

//     let wrapped = html! {
//         <>
//             {thumbnail}
//             {if let Some(metadata) = props.file_metadata() {
//                 html!{
//                     <>
//                     <button class="delete" onclick={on_click}><i class="fas
// fa-times"></i></button>                     <p
// class="metadata">{metadata}</p>                     </>
//                 }
//             } else {
//                 html!{
//                     <></>
//                 }
//             }}
//         </>
//     };

//     if props.large {
//         html! {
//             <div class={class} title={props.path()}>
//                 {wrapped}
//             </div>
//         }
//     } else {
//         html! {
//             <li class={class} title={props.path()}>
//                 {wrapped}
//             </li>
//         }
//     }
// }
