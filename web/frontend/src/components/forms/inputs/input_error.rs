//! Submodule defining the `InputError` component, which is used to define the
//! error message for an input or form.

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
            { for props.errors.iter().map(|error| {
                let font_awesome_icon = error.font_awesome_icon();

                if error == &ApiError::NoResults {
                    html! {
                        <li class="input-warning">
                            <p>
                                <i class={format!("fas fa-{}", font_awesome_icon)}></i>
                                {'\u{00a0}'}
                                <span>{"No Results"}</span>
                            </p>
                        </li>
                    }
                } else {
                    let errors: Vec<String> = error.clone().into();
                    html! {
                        { for errors.iter().map(|error| {
                            html! {
                                <li class="input-error">
                                    <p>
                                        <i class={format!("fas fa-{}", font_awesome_icon)}></i>
                                        {'\u{00a0}'}
                                        <span>{error}</span>
                                    </p>
                                </li>
                            }
                        })}
                    }
                }
            })}
        </ul>
    }
}
