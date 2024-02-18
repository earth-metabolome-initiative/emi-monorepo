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
//!
//! The styles of the search bar are defined using the Tailwind CSS framework.
//!

use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;


#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    html! {
        <div class="relative">
            <input
                type="text"
                class="w-64 h-10 px-3 pr-10 rounded-full border-2 border-gray-300 focus:outline-none focus:border-blue-500"
                placeholder="Search..."
            />
            <button
                class="absolute right-0 top-0 h-10 w-10 rounded-full bg-blue-500 text-white"
            >
                <i class="fas fa-search"></i>
            </button>
        </div>
    }
}
