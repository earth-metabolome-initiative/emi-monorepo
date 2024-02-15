//! A Yew-based search bar component to be placed in the navigator component.
//! 
//! The search bar is used to search for items in the application.
//! 
//! The styles of the search bar are defined using the Tailwind CSS framework.
//! 

use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let (state, _) = use_store::<Store>();

    let search_bar_classes = if state.sidebar_open {
        "hidden md:block bg-gray-100 text-gray-700 rounded-lg p-2 pl-4"
    } else {
        "hidden md:block bg-gray-100 text-gray-700 rounded-lg p-2 pl-4"
    };

    html! {
        <input class={search_bar_classes} type="text" placeholder="Search..." />
    }
}