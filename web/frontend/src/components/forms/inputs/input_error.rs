//! Submodule defining the `InputError` component, which is used to define the error message for an input or form.

use web_common::api::ApiError;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputErrorsProp {
    pub errors: Vec<ApiError>,
}

#[function_component(InputErrors)]
pub fn input_errors(props: &InputErrorsProp) -> Html {
    if props.errors.is_empty() {
        return html! { <></> };
    }

    html! {
        <ul class="input-errors">
            { for props.errors.iter().map(|error| html! {
                <li class="input-error">
                    <p>{format!("{:?}", error)}</p>
                </li>
            })}
        </ul>
    }
}
