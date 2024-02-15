//! The navigator Yew component is responsible for rendering the navigation bar at the top of the page.
//! 
//! Depending on whether we are on a large page, a middle page (such as table)
//! or a small page (such as a mobile device), the navigation bar will render differently.
//! 
//! In the case of a large page, a side bar component is always shown on the left side of the page.
//! In the case of a middle page, the side bar is hidden by default and can be toggled by clicking the hamburger icon,
//! which is located on the left side of the navigation bar.
//! In the case of a small page, the side bar is hidden by default and can be toggled by clicking the hamburger icon,
//! which is located on the left side of the navigation bar.
//! 
//! The navigator component, besides the occasional hamburger icon, also contains the logo of the application,
//! and in both the large and medium page cases, in the center shows a search bar. On the right side of the navigation bar,
//! it is shown in the large and medium page cases, the user name and the user avatar, and in the small page case, solely the user avatar.
//! On the right of the user name, in the large and medium page cases, there is a dropdown menu (symbolized by a down triangle),
//! which contains the user's settings and the logout button.
//! 
//! The overall web application needs to function also offline, as the user may want to use it while
//! they do not have an internet connection. Therefore, the navigator will also display a message to the
//! user if they are offline by putting a badge with the text "Offline" on their avatar. Upon returning online,
//! the badge will disappear.
//! 
//! The styles of the navigator are defined using the Tailwind CSS framework.

use yew::prelude::*;
use yewdux::prelude::*;


use crate::components::search_bar::SearchBar;
use crate::store::Store;


#[function_component(Navigator)]
pub fn navigator() -> Html {
    html! {
        <nav class="bg-white shadow">
            <div class="flex items center justify-between p-4">
                <div class="flex items center">
                    <button class="md:hidden" >
                        <i class="fas fa-bars"></i>
                    </button>
                    <img class="h-8" src="/logo.svg" alt="logo" />
                </div>
                <SearchBar />
                <div class="flex items center">
                    <div class="flex items center">
                        <img class="h-8 w-8 rounded-full" src="/avatar.jpg" alt="avatar" />
                        <span class="ml-2 hidden md:block">{"John Doe"}</span>
                        <div class="ml-2 hidden md:block">
                            <i class="fas fa-caret-down"></i>
                        </div>
                    </div>
                    <div class="ml-2 hidden md:block">
                        <span class="bg-red-500 text-white rounded-full p-1">{"Offline"}</span>
                    </div>
                </div>
            </div>
        </nav>
    }
}
