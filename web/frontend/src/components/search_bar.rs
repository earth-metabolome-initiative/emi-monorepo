//! A Yew-based search bar component to be placed in the navigator component.
//!
//! The search bar is used to search for items in the application.
//! When the screen is either large or medium, the search bar is shown in the center of the navigation bar.
//! When the screen is small, such as in a mobile device, the search bar is shown on top of the side bar when the hamburger icon is clicked.
//!
//! The search bar is a simple input field with smoothed corners, a search icon on the right (inside the input field),
//! and a placeholder text "Search...". Furthermore, a search button is shown when the input field is focused or has text inside it.
//! The search bar is also responsive, as it will shrink and grow depending on the size of the screen.
//! Upon clicking the search button, the search bar will be cleared and the focus will be removed from it. The url is updated by
//! using the Yew router, as we are working in a single page application.

use crate::components::InputWithIcon;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    // We prepare a boolean stored variable for the button_disabled property of the InputWithIcon component.
    // The variable is updated by the oninput event of the input field.
    let button_disabled = use_state(|| true);

    // For the time being, the search bar does not do anything.
    let onsubmit = { Callback::from(move |_| {}) };

    // For the oninput event, when the user types in the search bar,
    // for the time being we solely enable the search button.
    let oninput = {
        // We can clone the button_disabled state variable, as it is a reference counted pointer.
        let button_disabled = button_disabled.clone();
        Callback::from(move |input_event: InputEvent| {
            // If the input field is empty, we disable the search button.
            let input = input_event
                .target()
                .unwrap_throw()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap_throw();
            button_disabled.set(input.value().is_empty());
        })
    };

    html! {
        <div class="search">
            <InputWithIcon icon="search" placeholder="Search..." button_disabled={*button_disabled} onsubmit={onsubmit} oninput={oninput}/>
        </div>
    }
}
