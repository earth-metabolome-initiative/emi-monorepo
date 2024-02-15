//! A sidebar Yew component is responsible for rendering the sidebar on the left side of the page.
//! 
//! The sidebar contains a list of links to different pages of the application.
//! Some links may only be visible to the user if they have the appropriate permissions.
//! 
//! The sidebar is hidden by default on medium and small pages, and can be toggled by clicking the hamburger icon,
//! which is located on the left side of the navigation bar.
//! 
//! The sidebar is always visible on large pages.
//! 
//! The styles of the sidebar are defined using the Tailwind CSS framework.
//! 
//! The sidebar component is used in the navigator component.
//! 

use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let (state, _) = use_store::<Store>();

    let sidebar_classes = if state.sidebar_open {
        "w-64 bg-gray-800 text-white"
    } else {
        "w-64 bg-gray-800 text-white hidden"
    };

    html! {
        <div class={sidebar_classes}>
            <div class="flex items center justify-center p-4">
                <img class="h-8" src="/logo.svg" alt="logo" />
            </div>
            <ul class="mt-5">
                <li class="mb-2">
                    <a class="flex items
                    center text-white hover:bg-gray-700 p-2 pl-4" href="/dashboard">
                        <i class="fas fa-tachometer-alt"></i>
                        <span class="ml-2">{"Dashboard"}</span>
                    </a>
                </li>
                <li class="mb-2">
                    <a class="flex items
                    center text-white hover:bg-gray-700 p-2 pl-4" href="/feedback">
                        <i class="fas fa-comment-alt"></i>
                        <span class="ml-2">{"Feedback"}</span>
                    </a>
                </li>
                <li class="mb-2">
                    <a class="flex items
                    center text-white hover:bg-gray-700 p-2 pl-4" href="/users">
                        <i class="fas fa-users"></i>
                        <span class="ml-2">{"Users"}</span>
                    </a>
                </li>
                <li class="mb-2">
                    <a class="flex items
                    center text-white hover:bg-gray-700 p-2 pl-4" href="/settings">
                        <i class="fas fa-cog
                        "></i>
                        <span class="ml-2">{"Settings"}</span>
                    </a>
                </li>
            </ul>
        </div>
    }
}