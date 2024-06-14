//! Submodule providing the file input component for the frontend.

use super::file_like::*;
use super::InputErrors;
use crate::workers::FileProcessor;
use gloo::timers::callback::Timeout;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use web_common::file_formats::GenericFileFormat;
use yew::prelude::*;
use yew_agent::scope_ext::AgentScopeExt;

#[derive(Clone, PartialEq, Properties)]
pub struct MultiFileInputProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub label: String,
    pub append_file: Callback<Rc<Data>>,
    pub remove_file: Callback<usize>,
    pub errors: Vec<ApiError>,
    pub files: Rc<Vec<Rc<Data>>>,
    #[prop_or(false)]
    pub optional: bool,
    #[prop_or(1)]
    pub maximum_number_of_expected_files: usize,
    #[prop_or(1)]
    pub minimum_number_of_expected_files: usize,
    #[prop_or(16_777_216)] // 16 MB
    pub maximal_raw_size: u64,
    #[prop_or(1_048_576)] // 1 MB
    pub maximal_processed_size: u64,
}

impl<Data> MultiFileInputProp<Data>
where
    Data: FileLike,
{
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn normalized_label(&self) -> String {
        self.label.to_lowercase().replace(" ", "_")
    }

    pub fn human_readable_allowed_formats(&self) -> String {
        let mut formats = Data::FORMATS
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

pub struct MultiFileInput<Data> {
    input_ref: NodeRef,
    errors: HashSet<ApiError>,
    hide_box_timeout: Option<Timeout>,
    box_visible: bool,
    number_of_files_currently_processing: usize,
    _phantom: std::marker::PhantomData<Data>,
}

pub enum MultiFileInputMessage<Data> {
    DragStart,
    HideDropBox,
    NoFiles,
    FileProcessed(Data),
    FileProcessingFailed(ApiError),
    LoadedFile(Vec<u8>),
    Files(web_sys::FileList),
    AddError(ApiError),
}

impl<Data> Component for MultiFileInput<Data>
where
    Data: FileLike,
{
    type Message = MultiFileInputMessage<Data>;
    type Properties = MultiFileInputProp<Data>;

    fn create(ctx: &Context<Self>) -> Self {
        // We setup an event listener for the dragstart event, which is triggered when the user starts dragging a file
        // on the document. When this event is triggered, we send the DragStart message to the component.
        let document = web_sys::window().unwrap().document().unwrap();
        let link = ctx.link().clone();
        let drag_over_closure = Closure::wrap(Box::new(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            link.send_message(MultiFileInputMessage::DragStart);
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback(
                "dragover",
                drag_over_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        drag_over_closure.forget();

        let link = ctx.link().clone();
        let drop_closure = Closure::wrap(Box::new(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
            link.send_message(MultiFileInputMessage::HideDropBox);
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("drop", drop_closure.as_ref().unchecked_ref())
            .unwrap();
        drop_closure.forget();

        Self {
            input_ref: NodeRef::default(),
            errors: HashSet::new(),
            hide_box_timeout: None,
            box_visible: ctx.props().files.is_empty(),
            number_of_files_currently_processing: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MultiFileInputMessage::DragStart => {
                log::info!("Performing drag start");
                let box_was_invisible = !self.box_visible;
                self.box_visible = true;
                if let Some(timeout) = self.hide_box_timeout.take() {
                    timeout.cancel();
                }
                let link = ctx.link().clone();
                self.hide_box_timeout = Some(Timeout::new(1000, move || {
                    link.send_message(MultiFileInputMessage::HideDropBox)
                }));
                box_was_invisible
            }
            MultiFileInputMessage::HideDropBox => {
                let box_was_visible = self.box_visible;
                self.box_visible = false;
                box_was_visible
            }
            MultiFileInputMessage::NoFiles => {
                log::info!("No files");
                self.errors.clear();
                true
            }
            MultiFileInputMessage::FileProcessingFailed(error) => {
                self.number_of_files_currently_processing -= 1;
                ctx.link().send_message(MultiFileInputMessage::AddError(error));
                false
            }
            MultiFileInputMessage::AddError(error) => self.errors.insert(error),
            MultiFileInputMessage::LoadedFile(data) => {
                let link = ctx.link().clone();
                ctx.link().run_oneshot::<FileProcessor<Data>>(
                    data,
                    Callback::from(move |data: Result<Data, ApiError>| match data {
                        Ok(data) => {
                            link.send_message(MultiFileInputMessage::FileProcessed(data))
                        }
                        Err(error) => {
                            link.send_message(MultiFileInputMessage::FileProcessingFailed(error));
                        }
                    }),
                );
                false
            }
            MultiFileInputMessage::FileProcessed(data) => {
                self.number_of_files_currently_processing -= 1;
                ctx.props().append_file.emit(Rc::new(data));
                false
            }
            MultiFileInputMessage::Files(files) => {
                log::info!("Performing file upload");
                if files.length() == 0 {
                    ctx.link().send_message(MultiFileInputMessage::NoFiles);
                    return false;
                }

                self.errors.clear();

                // Before anything else, we check the file sizes.
                for i in 0..files.length() {
                    let file = files.get(i).unwrap();
                    if file.size() as u64 > ctx.props().maximal_raw_size {
                        ctx.link().send_message(MultiFileInputMessage::AddError(
                            ApiError::BadRequest(vec![format!(
                                "File {} is too large ({}). Maximum size is {}.",
                                file.name(),
                                human_readable_file_size(file.size() as usize),
                                human_readable_file_size(ctx.props().maximal_raw_size as usize)
                            )]),
                        ));
                        break;
                    }
                    // We check that the file mime type is allowed.
                    let mime_type = file.type_();

                    if !Data::FORMATS
                        .iter()
                        .any(|format| format.mime_types().contains(&mime_type.as_str()))
                    {
                        ctx.link().send_message(MultiFileInputMessage::AddError(
                            ApiError::InvalidFileFormat(format!(
                                "File {} has an invalid format. Only {} files are allowed.",
                                file.name(),
                                ctx.props().human_readable_allowed_formats()
                            )),
                        ));
                        break;
                    }

                    // We read the file and convert it to the appropriate type.
                    let file_name = file.name();

                    self.number_of_files_currently_processing += 1;

                    // First, we read the file as an array buffer using a send_future call.
                    ctx.link().send_future(async move {
                        match wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await {
                            Ok(buffer) => {
                                log::info!("File loaded");
                                let data = js_sys::Uint8Array::new(&buffer).to_vec();
                                MultiFileInputMessage::LoadedFile(data)
                            }
                            Err(_) => MultiFileInputMessage::AddError(ApiError::BadRequest(vec![
                                format!("Unable to read file {}", file_name),
                            ])),
                        }
                    });
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let container_on_click = {
            let input_node = self.input_ref.clone();
            Callback::from(move |_| {
                input_node
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .click();
            })
        };

        let on_input = {
            let link = ctx.link().clone();
            let input_node = self.input_ref.clone();
            Callback::from(move |input_event: InputEvent| {
                input_event.prevent_default();
                input_event.stop_propagation();
                match input_node.cast::<web_sys::HtmlInputElement>() {
                    Some(input) => match input.files() {
                        Some(files) => {
                            link.send_message(MultiFileInputMessage::Files(files));
                        }
                        None => {
                            link.send_message(MultiFileInputMessage::NoFiles);
                        }
                    },
                    None => {
                        link.send_message(MultiFileInputMessage::AddError(ApiError::BadRequest(
                            vec!["Unable to get files from input".to_string()],
                        )));
                    }
                };
            })
        };

        let on_drop = {
            let link = ctx.link().clone();
            Callback::from(move |drop_event: DragEvent| {
                drop_event.stop_propagation();
                drop_event.prevent_default();
                let files = drop_event.data_transfer().unwrap().files().unwrap();
                link.send_message(MultiFileInputMessage::Files(files));
            })
        };

        let droparea_classes = format!(
            "droparea{}",
            if self.box_visible { " dragging" } else { "" },
        );

        let label_classes = format!(
            "input-label{}",
            if ctx.props().optional {
                ""
            } else {
                " input-label-mandatory"
            }
        );

        let all_errors = ctx
            .props()
            .errors
            .iter()
            .chain(self.errors.iter())
            .cloned()
            .collect::<Vec<ApiError>>();

        let classes = format!(
            "input-group file {}{}",
            if !all_errors.is_empty() {
                "input-group-valid"
            } else {
                "input-group-invalid"
            },
            if self.number_of_files_currently_processing > 0 {
                " processing"
            } else {
                ""
            }
        );

        let object_unique_identifier = uuid::Uuid::new_v4();

        html! {
                <div class={classes} onclick={container_on_click} ondrop={on_drop}>
                    <label class={label_classes} for={object_unique_identifier.to_string()}>{format!("{}:", ctx.props().label)}</label>
                    <input
                        type="file"
                        ref={&self.input_ref}
                        class="input-control"
                        name={object_unique_identifier.to_string()}
                        multiple={ctx.props().maximum_number_of_expected_files > 1}
                        oninput={on_input}
                    />
                    if self.box_visible || ctx.props().files.is_empty() {
                        <div class={droparea_classes}>
                            <div class="droparea-icon"><i class="fas fa-file-upload"></i></div>
                            <p>{"Drag & Drop files here or click to select"}</p>
                        </div>
                    } else {
                        {ctx.props().files.iter().cloned().enumerate().map(|(index, file)| {
                            let delete = {
                                let remove_file = ctx.props().remove_file.clone();
                                Callback::from(move |_| {
                                    remove_file.emit(index);
                                })
                            };

                            html!{
                                <FilePreview<Data>
                                    file={file}
                                    delete={delete}
                                />
                            }
                        }).collect::<Html>()}
                    }
                    <InputErrors errors={all_errors} />
                </div>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FileInputProp<Data>
where
    Data: FileLike,
{
    pub label: String,
    pub builder: Callback<Option<Rc<Data>>>,
    pub errors: Vec<ApiError>,
    pub file: Option<Rc<Data>>,
    #[prop_or(false)]
    pub optional: bool,
    #[prop_or_default]
    pub allowed_formats: Vec<GenericFileFormat>,
    #[prop_or(16_777_216)] // 16 MB
    pub maximal_raw_size: u64,
    #[prop_or(1_048_576)] // 1 MB
    pub maximal_processed_size: u64,
}

#[function_component(FileInput)]
pub fn file_input<Data>(props: &FileInputProp<Data>) -> Html
where
    Data: FileLike,
{
    let append_file = {
        let old_builder = props.builder.clone();
        Callback::from(move |data: Rc<Data>| {
            old_builder.emit(Some(data));
        })
    };
    let remove_file = {
        let old_builder = props.builder.clone();
        Callback::from(move |index: usize| {
            assert_eq!(index, 0);
            old_builder.emit(None);
        })
    };

    html! {
        <MultiFileInput<Data>
            label={props.label.clone()}
            append_file={append_file}
            remove_file={remove_file}
            errors={props.errors.clone()}
            optional={props.optional}
            minimum_number_of_expected_files={1}
            maximum_number_of_expected_files={1}
            files={props.file.clone().map_or_else(|| Rc::from(Vec::new()), |file| Rc::from(vec![file]))}
            maximal_raw_size={props.maximal_raw_size}
            maximal_processed_size={props.maximal_processed_size}
        />
    }
}
