//! Submodule providing the file input component for the frontend.

use std::collections::HashSet;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use super::InputErrors;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use validator::Validate;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_common::api::form_traits::TryFromCallback;
use web_common::api::ws::messages::*;
use web_common::file_formats::GenericFileFormat;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

pub fn human_readable_size(size: u64) -> String {
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

#[derive(Clone, PartialEq, Properties)]
pub struct FileInputProp {
    pub label: String,
    #[prop_or(false)]
    pub optional: bool,
    #[prop_or(false)]
    pub multiple: bool,
    #[prop_or_default]
    pub allowed_formats: Vec<GenericFileFormat>,
    #[prop_or_default]
    pub urls: Vec<String>,
    #[prop_or_default]
    pub maximal_size: Option<u64>,
}

impl FileInputProp {
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn normalized_label(&self) -> String {
        self.label.to_lowercase().replace(" ", "_")
    }

    pub fn human_readable_allowed_formats(&self) -> String {
        let mut formats = self
            .allowed_formats
            .iter()
            .flat_map(|f| f.extensions())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if formats.len() == 1 {
            formats[0].to_string()
        } else if formats.len() == 2 {
            format!("{} and {}", formats[0], formats[1])
        } else {
            let last = formats.pop().unwrap();
            let first = formats.join(", ");
            format!("{}, and {}", first, last)
        }
    }
}

pub struct FileInput<Data> {
    _websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    errors: HashSet<String>,
    is_valid: Option<bool>,
    validation_timeout: Option<Timeout>,
    files: Vec<web_sys::File>,
    show_drop_area: bool,
    show_drop_area_timeout: Option<Timeout>,
    hide_drop_area_timeout: Option<Timeout>,
    dragging: bool,
    _phantom: std::marker::PhantomData<Data>,
}

pub enum InputMessage {
    Backend(BackendMessage),
    RemoveError(String),
    RemoveErrors,
    Validate(Result<web_sys::File, Vec<String>>),
    Files(web_sys::FileList),
    FilesRemoved(usize),
    SetTimeoutDropAreaVisibility(bool),
    SetDropAreaVisibility(bool),
    SetDragging(bool),
}

impl<Data> Component for FileInput<Data>
where
    Data: 'static + Clone + Validate + TryFromCallback<web_sys::File>,
{
    type Message = InputMessage;
    type Properties = FileInputProp;

    fn create(ctx: &Context<Self>) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let link = ctx.link().clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Handle the dragover event here
            // For example, prevent the default behavior to allow dropping elements
            event.prevent_default();
            link.send_message(InputMessage::SetDragging(true));
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("dragover", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget(); // Prevent the closure from being dropped immediately

        let link = ctx.link().clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Handle the dragend event here
            // For example, prevent the default behavior to allow dropping elements
            event.prevent_default();
            link.send_message(InputMessage::SetDragging(false));
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("dragend", closure.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("dragleave", closure.as_ref().unchecked_ref())
            .unwrap();
        document
            .add_event_listener_with_callback("drop", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget(); // Prevent the closure from being dropped immediately

        Self {
            _websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(InputMessage::Backend(message));
                }
            })),
            errors: HashSet::new(),
            is_valid: None,
            validation_timeout: None,
            files: Vec::new(),
            show_drop_area: ctx.props().urls.is_empty(),
            show_drop_area_timeout: None,
            hide_drop_area_timeout: None,
            dragging: false,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMessage::Backend(_bm) => false,
            InputMessage::SetDragging(dragging) => {
                if self.dragging != dragging {
                    self.dragging = dragging;
                    true
                } else {
                    false
                }
            }
            InputMessage::RemoveErrors => {
                let mut changes = false;

                if !self.errors.is_empty() {
                    self.errors.clear();
                    changes = true;
                }

                if self.is_valid.is_some() {
                    self.is_valid = None;
                    changes = true;
                }

                changes
            }
            InputMessage::Files(files) => {
                if files.length() == 0 {
                    return false;
                }

                let mut change = false;

                if !self.errors.is_empty() {
                    self.errors.clear();
                    change = true;
                }

                // If the properties require the file to be singular, we only keep the first file
                // and replace eventual previous files.
                if !ctx.props().multiple {
                    self.files.clear();
                    change = true;
                }

                let number_of_files = if ctx.props().multiple {
                    files.length()
                } else {
                    files.length().min(1)
                };

                let allowed_formats = &ctx.props().allowed_formats;

                for i in 0..number_of_files {
                    let file = files.get(i).unwrap();

                    if let Some(maximal_size) = ctx.props().maximal_size {
                        if file.size() as u64 > maximal_size {
                            self.errors.insert(format!(
                                "The file {} is too large. The maximal size is {}, but the file is {}.",
                                file.name(),
                                human_readable_size(maximal_size),
                                human_readable_size(file.size() as u64)
                            ));
                            change = true;
                            continue;
                        }
                    }

                    if !allowed_formats.is_empty() {
                        match GenericFileFormat::try_from_mime(&file.type_()) {
                            Ok(format) => {
                                if !allowed_formats.iter().any(|f| f == &format) {
                                    self.errors.insert(format!(
                                        concat!(
                                            "The file {} is not of an allowed format. ",
                                            "The allowed formats are: {}."
                                        ),
                                        file.type_(),
                                        ctx.props().human_readable_allowed_formats()
                                    ));
                                    change = true;
                                    continue;
                                }
                            }
                            Err(error) => {
                                let errors: Vec<String> = error.into();
                                self.errors.extend(errors.iter().cloned());
                                change = true;
                                continue;
                            }
                        }
                    }

                    if !self.files.iter().any(|f| {
                        f.name() == file.name()
                            && f.size() == file.size()
                            && f.last_modified() == file.last_modified()
                    }) {
                        let link = ctx.link().clone();
                        if let Err(error) = Data::try_from_callback(file.clone(), move |result| {
                            link.send_message(InputMessage::Validate(result.map(|_| file)));
                        }) {
                            ctx.link().send_message(InputMessage::Validate(Err(error)));
                        }
                    }
                }

                change
            }
            InputMessage::RemoveError(error) => {
                self.errors.remove(&error);
                true
            }
            InputMessage::Validate(data) => {
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }

                match data {
                    Ok(file) => {
                        self.files.push(file);
                        self.is_valid = Some(true);
                    }
                    Err(errors) => {
                        for error in errors {
                            self.errors.insert(error);
                        }
                        self.is_valid = Some(false);
                    }
                }

                ctx.link()
                    .send_message(InputMessage::SetTimeoutDropAreaVisibility(
                        self.files.is_empty() && ctx.props().urls.is_empty(),
                    ));

                true
            }
            InputMessage::FilesRemoved(index) => {
                self.files.remove(index);
                ctx.link()
                    .send_message(InputMessage::SetTimeoutDropAreaVisibility(
                        self.files.is_empty() && ctx.props().urls.is_empty(),
                    ));
                true
            }
            InputMessage::SetDropAreaVisibility(visibility) => {
                if let Some(timeout) = self.hide_drop_area_timeout.take() {
                    timeout.cancel();
                }
                if let Some(timeout) = self.show_drop_area_timeout.take() {
                    timeout.cancel();
                }
                if self.show_drop_area != visibility {
                    self.show_drop_area = visibility;
                    true
                } else {
                    false
                }
            }
            InputMessage::SetTimeoutDropAreaVisibility(visibility) => {
                if let Some(timeout) = self.hide_drop_area_timeout.take() {
                    timeout.cancel();
                }
                if let Some(timeout) = self.show_drop_area_timeout.take() {
                    timeout.cancel();
                }
                let link = ctx.link().clone();

                // If the target visibility is already the current visibility, we do not need to
                // do anything.
                if visibility == self.show_drop_area {
                    return false;
                }

                if visibility {
                    self.show_drop_area_timeout = Some(Timeout::new(200, move || {
                        link.send_message(InputMessage::SetDropAreaVisibility(true));
                    }));
                } else {
                    self.hide_drop_area_timeout = Some(Timeout::new(200, move || {
                        link.send_message(InputMessage::SetDropAreaVisibility(false));
                    }));
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let classes = match self.is_valid {
            Some(true) => "input-group file input-group-valid",
            Some(false) => "input-group file input-group-invalid",
            None => "input-group file",
        };

        let uuid = uuid::Uuid::new_v4().to_string();
        // First, we handle that on click on the div, the file input is triggered and
        // the user can select files.
        let on_click = {
            let uuid = uuid.clone();
            Callback::from(move |_: MouseEvent| {
                // This click event may come from any of the children of the div, so we
                // need to make sure that we retrieve the correct input field. We can use
                // the uuid to do so, as as to minimize the risk of conflicts.
                let input = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&uuid)
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>();
                input.click();
            })
        };

        // Then, we handle the input event, which is triggered when the user selects files.
        let on_input = {
            let link = ctx.link().clone();
            Callback::from(move |input_event: InputEvent| {
                input_event.prevent_default();

                let files = input_event
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>()
                    .files()
                    .unwrap();

                link.send_message(InputMessage::Files(files));
            })
        };

        let on_delete = {
            let link = ctx.link().clone();
            Callback::from(move |error: String| {
                link.send_message(InputMessage::RemoveError(error));
            })
        };

        let on_drop = {
            let link = ctx.link().clone();
            Callback::from(move |drop_event: DragEvent| {
                drop_event.prevent_default();
                let files = drop_event.data_transfer().unwrap().files().unwrap();
                link.send_message(InputMessage::Files(files));
            })
        };

        let on_file_delete = {
            let link = ctx.link().clone();
            Callback::from(move |index: usize| {
                link.send_message(InputMessage::FilesRemoved(index));
            })
        };

        let no_files = self.files.is_empty() && ctx.props().urls.is_empty();

        let ondragover = {
            let link = ctx.link().clone();
            Callback::from(move |event: DragEvent| {
                event.prevent_default();
                link.send_message(InputMessage::SetTimeoutDropAreaVisibility(true));
            })
        };

        let ondragleave = {
            let link = ctx.link().clone();
            Callback::from(move |event: DragEvent| {
                event.prevent_default();
                link.send_message(InputMessage::SetTimeoutDropAreaVisibility(no_files));
            })
        };

        let droparea_classes = format!(
            "droparea{}{}",
            if self.dragging { " dragging" } else { "" },
            if self.hide_drop_area_timeout.is_some() {
                " hiding"
            } else {
                ""
            }
        );

        html! {
            <div class={classes} onclick={on_click} ondrop={on_drop} ondragover={ondragover} ondragleave={ondragleave.clone()} ondragend={ondragleave}>
                <label for={props.normalized_label()}>{format!("{}:", props.label())}</label>
                <input
                    type="file"
                    id={uuid}
                    class="input-control"
                    name={props.normalized_label()}
                    multiple={props.multiple}
                    // value={input_value}
                    oninput={on_input}
                    // onblur={on_blur}
                />
                {if self.show_drop_area {
                    html! {
                        <div class={droparea_classes}>
                            <div class="droparea-icon"><i class="fas fa-file-upload"></i></div>
                            <p>{"Drag & Drop files here or click to select"}</p>
                        </div>
                    }
                } else {
                    html! {
                        <FilePreview
                            hiding={self.show_drop_area_timeout.is_some()}
                            files={self.files.clone()}
                            on_delete={on_file_delete}
                            urls={if props.multiple || self.files.is_empty() {props.urls.clone()} else {vec![]}}
                        />
                    }
                }}
                <InputErrors errors={self.errors.clone()} on_delete={on_delete} />
            </div>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FilePreviewProp {
    pub files: Vec<web_sys::File>,
    pub on_delete: Callback<usize>,
    #[prop_or_default]
    pub hiding: bool,
    #[prop_or_default]
    pub urls: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum FileLike {
    File(web_sys::File),
    Url(String),
}

impl FilePreviewProp {
    pub fn number_of_files(&self) -> usize {
        self.files.len() + self.urls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.urls.is_empty()
    }

    pub fn one_file(&self) -> Option<FileLike> {
        if self.number_of_files() == 1 {
            if let Some(file) = self.files.first() {
                return Some(FileLike::File(file.clone()));
            }
            if let Some(url) = self.urls.first() {
                return Some(FileLike::Url(url.clone()));
            }
        }
        None
    }

    pub fn iter(&self) -> impl Iterator<Item = FileLike> + '_ {
        self.files
            .iter()
            .map(|f| FileLike::File(f.clone()))
            .chain(self.urls.iter().map(|u| FileLike::Url(u.clone())))
    }
}

#[function_component(FilePreview)]
/// A component to display a preview of the files that have been selected.
///
/// # Implementative details
/// Depending on whether the provided files are images or not, the component will display
/// either the name of the file with an adequate icon, or a preview of the image. When the
/// document appear to be human-readable, the component will display a preview of the first
/// few lines of the document. Furthermore, the component will display the size of the file,
/// and the date at which it was last modified,
/// and a button to remove the file from the list of selected files.
///
/// # Properties
/// The component receives a list of web_sys::File objects, which it then displays. Since it
/// also needs to be able to communicate with the parent component, it receives a callback to
/// send messages to the parent component to delete the file from the list of selected files.
pub fn file_preview(props: &FilePreviewProp) -> Html {
    if props.is_empty() {
        return html! {
            <></>
        };
    }

    if let Some(file_like) = props.one_file() {
        match file_like {
            FileLike::File(file) => {
                let on_delete = {
                    let props = props.clone();
                    Callback::from(move |_| {
                        let props = props.clone();
                        let timeout = Timeout::new(200, move || {
                            props.on_delete.emit(0);
                        });
                        timeout.forget();
                    })
                };

                return html! {
                    <FilePreviewItem
                        hiding={props.hiding}
                        large={true}
                        file={FileLike::File(file)}
                        on_delete={on_delete}
                    />
                };
            }
            FileLike::Url(url) => {
                return html! {
                    <FilePreviewItem
                        hiding={props.hiding}
                        large={true}
                        file={FileLike::Url(url)}
                        on_delete={Callback::noop()}
                    />
                };
            }
        }
    }

    let mut class = format!(
        "file-preview-list {}",
        if props.hiding { "hiding" } else { "" }
    );

    if props.number_of_files() == 2 {
        class += " two-files";
    }

    if props.number_of_files() == 3 {
        class += " three-files";
    }

    html! {
        <ul class={class}>
            { for props.iter().enumerate().map(|(i, file)|{
                let props = props.clone();
                let on_delete = Callback::from(move |_| {
                    let props = props.clone();
                    let timeout = Timeout::new(200, move || {
                        props.on_delete.emit(i);
                    });
                    timeout.forget();
                });

                html! {
                <FilePreviewItem large={false} file={file.clone()} on_delete={on_delete} />
            }})}
        </ul>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FilePreviewItemProp {
    pub file: FileLike,
    #[prop_or_default]
    pub on_delete: Callback<()>,
    pub large: bool,
    #[prop_or_default]
    pub hiding: bool,
}

impl FilePreviewItemProp {
    pub fn path(&self) -> Option<String> {
        match &self.file {
            FileLike::File(file) => Some(file.name()),
            FileLike::Url(_) => None,
        }
    }

    pub fn name(&self) -> Option<String> {
        match &self.file {
            FileLike::File(file) => Some(file.name().split('/').last().unwrap().to_string()),
            FileLike::Url(_) => None,
        }
    }

    pub fn extension(&self) -> Option<String> {
        match &self.file {
            FileLike::File(file) => {
                let path = file.name();
                let extension = path.split('.').last();
                extension.map(|s| s.to_string())
            }
            FileLike::Url(url) => {
                let extension = url.split('.').last();
                extension.map(|s| s.to_string())
            }
        }
    }

    pub fn size(&self) -> Option<u64> {
        match &self.file {
            FileLike::File(file) => Some(file.size() as u64),
            FileLike::Url(_) => None,
        }
    }

    pub fn is_image(&self) -> bool {
        match &self.file {
            FileLike::File(file) => file.type_().starts_with("image"),
            FileLike::Url(url) => {
                url.ends_with(".png")
                    || url.ends_with(".jpg")
                    || url.ends_with(".jpeg")
                    || url.ends_with(".gif")
                    || url.ends_with(".webp")
            }
        }
    }

    pub fn is_pdf(&self) -> bool {
        match &self.file {
            FileLike::File(file) => file.type_().starts_with("application/pdf"),
            FileLike::Url(url) => url.ends_with(".pdf"),
        }
    }

    pub fn last_modified(&self) -> Option<u64> {
        match &self.file {
            FileLike::File(file) => Some(file.last_modified() as u64),
            FileLike::Url(_) => None,
        }
    }

    pub fn metadata_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        match &self.file {
            FileLike::File(file) => {
                file.name().hash(&mut hasher);
            }
            FileLike::Url(url) => {
                url.hash(&mut hasher);
            }
        }
        self.size().hash(&mut hasher);
        self.last_modified().hash(&mut hasher);
        hasher.finish()
    }

    pub fn human_readable_size(&self) -> Option<String> {
        self.size().map(human_readable_size)
    }

    pub fn last_modified_date(&self) -> Option<String> {
        self.last_modified().map(|last_modified| {
            let date = js_sys::Date::new(&JsValue::from_f64(last_modified as f64));
            let date = date.to_locale_string("en-US", &JsValue::undefined());
            date.as_string().unwrap()
        })
    }

    /// Returns a human-readable string representing the time elapsed since the file was last modified.
    pub fn human_readable_modified_delta(&self) -> Option<String> {
        let date = self.last_modified()?;
        let date = js_sys::Date::new(&JsValue::from_f64(date as f64));
        let now = js_sys::Date::new_0();
        let delta = now.get_time() - date.get_time();
        let delta = (delta as f64 / 1000.0) as u64;
        Some(if delta < 60 {
            format!("{} seconds ago", delta)
        } else if delta < 60 * 60 {
            format!("{} minutes ago", delta / 60)
        } else if delta < 60 * 60 * 24 {
            format!("{} hours ago", delta / (60 * 60))
        } else if delta < 60 * 60 * 24 * 30 {
            format!("{} days ago", delta / (60 * 60 * 24))
        } else if delta < 60 * 60 * 24 * 30 * 12 {
            format!("{} months ago", delta / (60 * 60 * 24 * 30))
        } else {
            format!("{} years ago", delta / (60 * 60 * 24 * 30 * 12))
        })
    }

    /// Returns the file metadata when the variant is a File.
    pub fn file_metadata(&self) -> Option<String> {
        match &self.file {
            FileLike::File(_) => Some(format!(
                "{}, {}, from {}",
                self.name()?,
                self.human_readable_size()?,
                self.human_readable_modified_delta()?
            )),
            FileLike::Url(_) => None,
        }
    }
}

pub struct ImagePreview {}

impl Component for ImagePreview {
    type Message = ();
    type Properties = FilePreviewItemProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // We add an hash obtained from the file name and the associated informations, such
        // as the file size and the last modified date, to the URL to make sure that the URL
        // changes when the file changes, so that the image is reloaded when the file changes.

        let hash = ctx.props().metadata_hash();

        match &ctx.props().file {
            FileLike::File(file) => {
                let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                let url = format!("{}#{}", url, hash);
                html! {
                    <img src={url} class="preview" />
                }
            }
            FileLike::Url(url) => {
                let url = format!("{}#{}", url, hash);
                html! {
                    <img src={url} class="preview" />
                }
            }
        }
    }
}

pub struct PDFPreview {}

impl Component for PDFPreview {
    type Message = ();
    type Properties = FilePreviewItemProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // We add an hash obtained from the file name and the associated informations, such
        // as the file size and the last modified date, to the URL to make sure that the URL
        // changes when the file changes, so that the image is reloaded when the file changes.

        let hash = ctx.props().metadata_hash();

        match &ctx.props().file {
            FileLike::File(file) => {
                let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                let url = format!("{}#{}", url, hash);
                html! {
                    <iframe src={url} class="preview"></iframe>
                }
            }
            FileLike::Url(url) => {
                let url = format!("{}#{}", url, hash);
                html! {
                    <iframe src={url} class="preview"></iframe>
                }
            }
        }
    }
}

pub struct TextualFilePreview {
    text: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextualFilePreviewProps {
    pub file_props: FilePreviewItemProp,
    #[prop_or(20)]
    pub number_of_lines: usize,
}

pub enum TextualFilePreviewMessage {
    Load(String),
}

impl Component for TextualFilePreview {
    type Message = TextualFilePreviewMessage;
    type Properties = TextualFilePreviewProps;

    fn create(ctx: &Context<Self>) -> Self {
        let file = match ctx.props().file_props.file.clone() {
            FileLike::File(file) => file,
            FileLike::Url(_) => {
                unreachable!("TextualFilePreview should only be used with files, not URLs.")
            }
        };
        let reader = web_sys::FileReader::new().unwrap();
        let link = ctx.link().clone();
        // We read the first few lines of the file to display a preview of the file.

        const CHUNK_SIZE: usize = 1024; // Adjust the chunk size as needed

        let number_of_lines = ctx.props().number_of_lines;

        let on_load = Closure::wrap(Box::new(move |event: web_sys::ProgressEvent| {
            let mut lines_read = 0;
            let mut text = String::new();
            let reader = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::FileReader>()
                .unwrap();
            let result = reader.result().unwrap();
            let data = js_sys::Uint8Array::new(&result);

            let mut chunk = Vec::with_capacity(CHUNK_SIZE);
            for i in 0..data.length() {
                chunk.push(data.get_index(i));
                if chunk.len() >= CHUNK_SIZE {
                    let chunk_text = String::from_utf8_lossy(&chunk);

                    for line in chunk_text.lines() {
                        text.push_str(line);
                        text.push('\n');
                        lines_read += 1;
                        if lines_read >= number_of_lines {
                            break;
                        }
                    }

                    if lines_read >= number_of_lines {
                        break;
                    }

                    chunk.clear();
                }
            }

            if lines_read < number_of_lines {
                let remaining_chunk_text = String::from_utf8_lossy(&chunk);
                text.push_str(&remaining_chunk_text);
                lines_read += remaining_chunk_text.lines().count();
            }

            link.send_message(TextualFilePreviewMessage::Load(text));
        }) as Box<dyn FnMut(_)>);

        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
        on_load.forget();

        reader.read_as_array_buffer(&file).unwrap();

        Self {
            text: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextualFilePreviewMessage::Load(text) => {
                self.text = text;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="file-preview-item-thumbnail">
                <p>{&self.text}</p>
            </div>
        }
    }
}

#[function_component(FilePreviewItem)]
/// A component to display a preview of a single file of a list of files.
pub fn file_preview_item(props: &FilePreviewItemProp) -> Html {
    let on_delete = props.on_delete.clone();
    let hiding = use_state(|| false);

    let on_click = {
        let hiding = hiding.clone();
        Callback::from(move |click: MouseEvent| {
            click.stop_immediate_propagation();
            click.prevent_default();
            hiding.set(true);
            on_delete.emit(());
        })
    };

    let thumbnail: Html = {
        if props.is_image() {
            html! { <ImagePreview file={props.file.clone()} large={props.large} /> }
        } else if props.is_pdf() {
            html! { <PDFPreview file={props.file.clone()} large={props.large} /> }
        } else {
            html! { <TextualFilePreview file_props={props.clone()} /> }
        }
    };

    let class = format!(
        "file-preview-item{}",
        if props.hiding || *hiding {
            " hiding"
        } else {
            ""
        }
    );

    let wrapped = html! {
        <>
            {thumbnail}
            {if let Some(metadata) = props.file_metadata() {
                html!{
                    <>
                    <button class="delete" onclick={on_click}><i class="fas fa-times"></i></button>
                    <p class="metadata">{metadata}</p>
                    </>
                }
            } else {
                html!{
                    <></>
                }
            }}
        </>
    };

    if props.large {
        html! {
            <div class={class} title={props.path()}>
                {wrapped}
            </div>
        }
    } else {
        html! {
            <li class={class} title={props.path()}>
                {wrapped}
            </li>
        }
    }
}
