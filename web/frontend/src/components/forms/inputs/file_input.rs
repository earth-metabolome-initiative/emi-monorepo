//! Submodule providing the file input component for the frontend.

use std::collections::HashSet;

use super::InputErrors;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use validator::Validate;
use wasm_bindgen::JsCast;
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

                let data = data.unwrap();

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
                true
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

        html! {
            <div class={classes} onclick={on_click} ondrop={on_drop} ondragover={|event: DragEvent| event.prevent_default()}>
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
                <p>{"Drag & Drop files here or click to select"}</p>
                <FilePreview files={self.files.clone()} on_delete={on_file_delete} />
                <InputErrors errors={self.errors.clone()} on_delete={on_delete} />
            </div>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FilePreviewProp {
    pub files: Vec<web_sys::File>,
    pub on_delete: Callback<usize>,
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
        let props = props.clone();
        let on_delete = Callback::from(move |_| {
            props.on_delete.emit(0);
        });

        return html! {
            <LargeFilePreviewItem file={props.files[0].clone()} on_delete={on_delete} />
        };
    }

    html! {
        <ul class="file-preview">
            { for props.files.iter().enumerate().map(|(i, file)|{
                let props = props.clone();
                let on_delete = Callback::from(move |_| {
                    props.on_delete.emit(i);
                });

                html! {
                <FilePreviewItem file={file.clone()} on_delete={on_delete} />
            }})}
        </ul>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FilePreviewItemProp {
    pub file: web_sys::File,
    pub on_delete: Callback<()>,
}

const DEFAULT_PREVIEW_LINES: usize = 5;

pub fn read_text_preview(file: &web_sys::File) -> Option<String> {
    let reader = web_sys::FileReader::new().unwrap();
    reader.read_as_text(&file).ok()?;
    let result = reader.result().ok()?;
    let text = result.as_string().unwrap();

    Some(get_first_lines(&text))
}

fn get_first_lines(text: &str) -> String {
    let mut preview = String::new();
    let lines = text.lines().take(DEFAULT_PREVIEW_LINES);
    for line in lines {
        preview.push_str(line);
        preview.push('\n');
    }
    preview.trim().to_string()
}

#[function_component(LargeFilePreviewItem)]
/// A component to display a preview of a single file.
pub fn large_file_preview_item(props: &FilePreviewItemProp) -> Html {
    let file = props.file.clone();
    let name = file.name();
    let size = file.size();
    let last_modified = file.last_modified();
    let on_delete = props.on_delete.clone();

    let on_click = {
        Callback::from(move |click: MouseEvent| {
            click.stop_immediate_propagation();
            click.prevent_default();
            on_delete.emit(());
        })
    };

    // Depending on whether the file is an image or a text file, we
    // wither display the image or the first few lines of the file as
    // a background.

    let background: Html = {
        let file_type = file.type_();
        if file_type.starts_with("image") {
            let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
            let image_url = format!("url({})", url);
            html! {
                <div class="file-preview-item-background" style={format!("background-image: {}", image_url)}></div>
            }
        } else if file_type.starts_with("text") {
            let text = match read_text_preview(&file) {
                Some(preview) => preview,
                None => "none".to_string(),
            };
            html! {
                <div class="file-preview-item-background">
                    <p>{text}</p>
                </div>
            }
        } else {
            html! { <div class="file-preview-item-background"></div> }
        }
    };

    html! {
        <div class="file-preview-item large">
            <div class="file-preview-item-header">
                {background}
                <p>{name}</p>
                <button onclick={on_click}><i class="fas fa-times"></i></button>
            </div>
            <div class="file-preview-item-body">
                <p>{format!("Size: {}", size)}</p>
                <p>{format!("Last modified: {}", last_modified)}</p>
            </div>
        </div>
    }
}

#[function_component(FilePreviewItem)]
/// A component to display a preview of a single file of a list of files.
pub fn file_preview_item(props: &FilePreviewItemProp) -> Html {
    let file = props.file.clone();
    let name = file.name();
    let size = file.size();
    let last_modified = file.last_modified();
    let on_delete = props.on_delete.clone();

    let on_click = {
        Callback::from(move |click: MouseEvent| {
            click.stop_immediate_propagation();
            click.prevent_default();
            on_delete.emit(());
        })
    };

    let thumbnail: Html = {
        let file_type = file.type_();
        if file_type.starts_with("image") {
            let url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
            let image_url = format!("url({})", url);
            html! {
                <div class="file-preview-item-thumbnail" style={format!("background-image: {}", image_url)}></div>
            }
        } else {
            html! { <div class="file-preview-item-thumbnail"></div> }
        }
    };

    html! {
        <li class="file-preview-item">
            <div class="file-preview-item-header">
                {thumbnail}
                <p>{name}</p>
                <button onclick={on_click}><i class="fas fa-times"></i></button>
            </div>
            <div class="file-preview-item-body">
                <p>{format!("Size: {}", size)}</p>
                <p>{format!("Last modified: {}", last_modified)}</p>
            </div>
        </li>
    }
}
