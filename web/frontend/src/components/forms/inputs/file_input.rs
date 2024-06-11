//! Submodule providing the file input component for the frontend.

use super::file_like::*;
use super::InputErrors;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use web_common::file_formats::GenericFileFormat;
use yew::prelude::*;
use yew_hooks::use_throttle;

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
    #[prop_or(1_048_576)] // 1 MB
    pub maximal_size: u64,
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

#[function_component(MultiFileInput)]
pub fn multi_file_input<Data: FileLike>(props: &MultiFileInputProp<Data>) -> Html {
    let container_node = use_node_ref();
    let input_node = use_node_ref();
    let inner_errors = use_state(|| Vec::new());
    let is_dragging = use_state(|| false);

    // First, we handle that on click on the container node, it should trigger a click on the input node.
    let container_on_click = {
        let input_node = input_node.clone();
        Callback::from(move |_| {
            input_node
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .click();
        })
    };

    // We define the callback that handles the file input messages.
    let on_files = {
        let props = props.clone();
        let inner_errors = inner_errors.clone();
        let container_node = container_node.clone();
        let input_node = input_node.clone();
        Callback::from(move |_| {
            // TODO! Handle here the file being provided.
            let file_list: web_sys::FileList = input_node
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .files()
                .unwrap();

            
        })
    };

    // Then, we handle the input event, which is triggered when the user selects files.
    let on_input = {
        let on_files = on_files.clone();
        Callback::from(move |input_event: InputEvent| {
            input_event.prevent_default();
            on_files.emit(());
        })
    };

    let on_drop = {
        let on_files = on_files.clone();
        Callback::from(move |drop_event: DragEvent| {
            drop_event.prevent_default();
            on_files.emit(());
        })
    };

    let throttle_drag_start = {
        let is_dragging = is_dragging.clone();
        use_throttle(
            move || {
                is_dragging.set(true);
            },
            300,
        )
    };
    let throttle_drag_end = {
        let is_dragging = is_dragging.clone();
        use_throttle(
            move || {
                is_dragging.set(false);
            },
            300,
        )
    };

    // We set a callback event listening for when any file is dragged over the document.
    // We need to throttle how often this event is called to avoid performance issues.
    // When the dragging is ongoing, we set the currently_dragging state to true, and
    // when the dragging ends, we set it to false.
    use_effect({
        move || {
            let document = web_sys::window().unwrap().document().unwrap();

            let drag_over_closure = Closure::wrap(Box::new(move |e: Event| {
                e.prevent_default();
            }) as Box<dyn FnMut(_)>);

            let drag_enter_closure = Closure::wrap(Box::new(move |e: Event| {
                e.prevent_default();
                throttle_drag_start.run();
            }) as Box<dyn FnMut(_)>);

            let drag_leave_closure = Closure::wrap(Box::new(move |_: Event| {
                throttle_drag_end.run();
            }) as Box<dyn FnMut(_)>);

            document
                .add_event_listener_with_callback(
                    "dragover",
                    drag_over_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            document
                .add_event_listener_with_callback(
                    "dragenter",
                    drag_enter_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            document
                .add_event_listener_with_callback(
                    "dragleave",
                    drag_leave_closure.as_ref().unchecked_ref(),
                )
                .unwrap();

            drag_over_closure.forget();
            drag_enter_closure.forget();
            drag_leave_closure.forget();

            move || {
                // Clean up code here if necessary
            }
        }
    });

    let droparea_classes = format!("droparea{}", if *is_dragging { " dragging" } else { "" },);

    let label_classes = format!(
        "input-label{}",
        if props.optional {
            ""
        } else {
            " input-label-mandatory"
        }
    );

    let all_errors = props
        .errors
        .iter()
        .chain(inner_errors.iter())
        .cloned()
        .collect::<Vec<ApiError>>();

    let classes = if !all_errors.is_empty() {
        "input-group file input-group-valid"
    } else {
        "input-group file input-group-invalid"
    };

    let object_unique_identifier = uuid::Uuid::new_v4();

    html! {
        <div ref={container_node} class={classes} onclick={container_on_click} ondrop={on_drop}>
            <label class={label_classes} for={object_unique_identifier.to_string()}>{format!("{}:", props.label)}</label>
            <input
                type="file"
                ref={input_node}
                class="input-control"
                name={object_unique_identifier.to_string()}
                multiple={props.maximum_number_of_expected_files > 1}
                oninput={on_input}
            />
            if *is_dragging || props.files.is_empty() {
                <div class={droparea_classes}>
                    <div class="droparea-icon"><i class="fas fa-file-upload"></i></div>
                    <p>{"Drag & Drop files here or click to select"}</p>
                </div>
            } else {
                {props.files.iter().cloned().enumerate().map(|(index, file)| {
                    let delete = {
                        let remove_file = props.remove_file.clone();
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
    #[prop_or(1_048_576)] // 1 MB
    pub maximal_size: u64,
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
            maximal_size={props.maximal_size}
        />
    }
}
