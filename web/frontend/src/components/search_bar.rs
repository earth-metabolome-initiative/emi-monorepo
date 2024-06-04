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

use crate::components::forms::Datalist;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_common::database::SearchableStruct;
use yew::prelude::*;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let focus_state = use_state(|| false);
    // When the user clicks anywhere outside of the search bar, we want to remove the focus from it.
    // This is done by adding an event listener to the document.
    // The event listener is removed when the component is destroyed.
    let document = web_sys::window().unwrap().document().unwrap();

    // When the document is clicked, we want to remove the focus from the search bar.
    // Since we also have an onclick listener on the searchbar that stops the propagation of the event,
    // we need to check if the target of the event is not the search bar.
    let on_document_click = {
        let focus_state = focus_state.clone();
        Closure::wrap(Box::new(move |_: web_sys::Event| {
            focus_state.set(false);
        }) as Box<dyn FnMut(_)>)
    };

    // Add the event listener to the document.
    document
        .add_event_listener_with_callback("click", on_document_click.as_ref().unchecked_ref())
        .unwrap();

    on_document_click.forget();

    let onclick = {
        let focus_state = focus_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
            focus_state.set(true)
        })
    };

    let classes = format!("search{}", if *focus_state { " focus" } else { "" });

    html! {
        <div id="search-bar" class={classes} onclick={onclick}>
            <Datalist<SearchableStruct, false> label="Search" show_label={false} show_load_more={false} />
        </div>
    }
}
