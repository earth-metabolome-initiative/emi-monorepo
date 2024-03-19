//! Submodule providing the file input component for the frontend.

use std::collections::HashSet;
use std::hash::Hasher;
use std::hash::DefaultHasher;
use std::hash::Hash;

use super::InputErrors;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use validator::Validate;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_common::api::ws::messages::*;
use web_common::custom_validators::validation_errors::ValidationErrorToString;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

#[derive(Clone, PartialEq, Properties)]
pub struct FileInputProp {
    pub label: String,
    #[prop_or(false)]
    pub optional: bool,
    #[prop_or(false)]
    pub multiple: bool,
}

impl FileInputProp {
    pub fn label(&self) -> String {
        self.label.clone()
    }
}

pub struct FileInput {
    _websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    errors: HashSet<String>,
    is_valid: Option<bool>,
    validation_timeout: Option<Timeout>,
    files: Vec<web_sys::File>,
    show_drop_area: bool,
    show_drop_area_timeout: Option<Timeout>,
    hide_drop_area_timeout: Option<Timeout>,
}

pub enum InputMessage {
    Backend(BackendMessage),
    RemoveError(String),
    RemoveErrors,
    Validate(Result<(), Vec<String>>),
    StartValidationTimeout(Result<(), Vec<String>>),
    UpdateCurrentValue(String),
    // Loaded(String, String, Vec<u8>),
    Files(web_sys::FileList),
    FilesRemoved(usize),
    SetTimeoutDropAreaVisibility(bool),
    SetDropAreaVisibility(bool),
}

impl Component for FileInput {
    type Message = InputMessage;
    type Properties = FileInputProp;

    fn create(ctx: &Context<Self>) -> Self {
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
            show_drop_area: true,
            show_drop_area_timeout: None,
            hide_drop_area_timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMessage::Backend(_bm) => false,
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

                // If the properties require the file to be singular, we only keep the first file
                // and replace eventual previous files.
                if !ctx.props().multiple {
                    self.files.clear();
                }

                let number_of_files = if ctx.props().multiple {
                    files.length()
                } else {
                    files.length().min(1)
                };

                for i in 0..number_of_files {
                    let file = files.get(i).unwrap();
                    self.files.push(file);
                }

                ctx.link()
                    .send_message(InputMessage::SetTimeoutDropAreaVisibility(
                        self.files.is_empty(),
                    ));

                files.length() > 0
            }
            InputMessage::RemoveError(error) => {
                self.errors.remove(&error);
                true
            }
            InputMessage::Validate(data) => {
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }

                let mut change = false;

                if !self.errors.is_empty() {
                    self.errors.clear();
                    change = true;
                }

                if let Err(errors) = data {
                    for error in errors {
                        self.errors.insert(error);
                    }
                    self.is_valid = Some(false);
                    return true;
                }

                if self.is_valid != Some(true) {
                    self.is_valid = Some(true);
                    change = true;
                }

                change
            }
            InputMessage::StartValidationTimeout(data) => {
                let link = ctx.link().clone();
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }
                self.validation_timeout = Some(Timeout::new(300, move || {
                    link.send_message(InputMessage::Validate(data));
                }));
                false
            }
            InputMessage::UpdateCurrentValue(value) => {
                // self.current_value = Some(value);
                false
            }
            InputMessage::FilesRemoved(index) => {
                self.files.remove(index);
                ctx.link()
                    .send_message(InputMessage::SetTimeoutDropAreaVisibility(
                        self.files.is_empty(),
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

        let no_files = self.files.is_empty();

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

        let droparea_classes = if self.hide_drop_area_timeout.is_some() {
            "droparea hiding"
        } else {
            "droparea"
        };

        html! {
            <div class={classes} onclick={on_click} ondrop={on_drop} ondragover={ondragover} ondragleave={ondragleave}>
                <label for={props.label()}>{format!("{}:", props.label())}</label>
                <input
                    type="file"
                    id={uuid}
                    class="input-control"
                    name={props.label().to_lowercase().replace(" ", "_")}
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
    if props.files.is_empty() {
        return html! { <></> };
    }

    if props.files.len() == 1 {
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
                file={props.files[0].clone()}
                on_delete={on_delete}
            />
        };
    }

    let class = format!(
        "file-preview-list {}",
        if props.hiding { "hiding" } else { "" }
    );

    html! {
        <ul class={class}>
            { for props.files.iter().enumerate().map(|(i, file)|{
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
    pub file: web_sys::File,
    #[prop_or_default]
    pub on_delete: Callback<()>,
    pub large: bool,
    #[prop_or_default]
    pub hiding: bool,
}

impl FilePreviewItemProp {
    pub fn name(&self) -> String {
        self.file.name()
    }

    pub fn extension(&self) -> Option<String> {
        self.file.name().split('.').last().map(|s| s.to_string())
    }

    pub fn size(&self) -> u64 {
        self.file.size() as u64
    }

    pub fn is_image(&self) -> bool {
        self.file.type_().starts_with("image")
    }

    pub fn last_modified(&self) -> u64 {
        self.file.last_modified() as u64
    }

    pub fn metadata_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.file.name().hash(&mut hasher);
        self.size().hash(&mut hasher);
        self.last_modified().hash(&mut hasher);
        hasher.finish()
    }

    pub fn human_readable_size(&self) -> String {
        let size = self.size();
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.2} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    pub fn last_modified_date(&self) -> String {
        let date = self.file.last_modified();
        let date = js_sys::Date::new(&JsValue::from_f64(date));
        let date = date.to_locale_string("en-US", &JsValue::undefined());
        date.as_string().unwrap()
    }

    /// Returns a human-readable string representing the time elapsed since the file was last modified.
    pub fn human_readable_modified_delta(&self) -> String {
        let date = self.file.last_modified();
        let date = js_sys::Date::new(&JsValue::from_f64(date));
        let now = js_sys::Date::new_0();
        let delta = now.get_time() - date.get_time();
        let delta = (delta as f64 / 1000.0) as u64;
        if delta < 60 {
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
        let file = ctx.props().file.clone();
        let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();

        // We add an hash obtained from the file name and the associated informations, such
        // as the file size and the last modified date, to the URL to make sure that the URL
        // changes when the file changes, so that the image is reloaded when the file changes.

        let hash = ctx.props().metadata_hash();
        let url = format!("{}#{}", url, hash);

        html! {
            <div class="image-preview" style={format!("background-image: url({})", url)}></div>
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
        let file = ctx.props().file_props.file.clone();
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
            <button class="delete" onclick={on_click}><i class="fas fa-times"></i></button>
            <p class="metadata">{format!("{}, edited last {}", props.human_readable_size(), props.human_readable_modified_delta())}</p>
        </>
    };

    if props.large {
        html! {
            <div class={class}>
                {wrapped}
            </div>
        }
    } else {
        html! {
            <li class={class}>
                {wrapped}
            </li>
        }
    }
}
