//! The navigator Yew 0.21 component is responsible for rendering the navigation bar at the top of the page.
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
//! The navigator component handles where and whether to show the search bar, the hamburger icon, the user name and avatar,
//! the offline badge, the dropdown menu, and the side bar. When the screen is small, such as in a mobile device, the navigator
//! will also handle the toggling of the side bar when the hamburger icon is clicked.
//! 

use yew::prelude::*;
use yewdux::prelude::*;


use crate::components::search_bar::SearchBar;
use crate::store::Store;


#[function_component(Navigator)]
pub fn navigator() -> Html {
    let (store, _) = use_store::<Store>();

    let is_offline = store.is_offline;

    html! {
        <nav>
            <h1>{"Earth Metabolome Initiative"}</h1>
        </nav>
    }
}