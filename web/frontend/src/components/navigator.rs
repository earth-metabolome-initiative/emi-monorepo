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

use crate::store::Store;

#[function_component(Navigator)]
pub fn navigator() -> Html {
    let (state, _) = use_store::<Store>();

    let navigator_classes = if state.sidebar_open {
        "bg-white shadow"
    } else {
        "bg-white shadow md:hidden"
    };

    html! {
        <nav class={navigator_classes}>
            <div class="flex items-center justify-between p-4">
                <div class="flex items center">
                    <button class="text-gray-500 focus:outline-none md:hidden">
                        <i class="fas fa-bars"></i>
                    </button>
                    <img class="h-8" src="/logo.svg" alt="logo" />
                </div>
                <div class="flex items
                center">
                    <input class="hidden md:block bg-gray-100 text-gray-700 rounded-lg p-2 pl-4" type="text" placeholder="Search..." />
                    <div class="flex items
                    center ml-4">
                        <div class="hidden md:block">
                            <span class="text-gray-700 font-bold">{"John Doe"}</span>
                            <img class="h-8 w-8 rounded-full ml-2" src="/avatar.jpg" alt="avatar" />
                            <i class="fas fa-caret-down ml-2"></i>
                        </div>
                        <div class="md:hidden">
                            <img class="h-8 w-8 rounded-full" src="/avatar.jpg" alt="avatar" />
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
