//! The footer component.
use yew::prelude::*;

#[function_component(Footer)]
/// The footer component.
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="container">
                <div class="footer-links">
                <ul>
                    <li><a href="#">{"Home"}</a></li>
                    <li><a href="#">{"About"}</a></li>
                    <li><a href="#">{"Services"}</a></li>
                    <li><a href="#">{"Contact"}</a></li>
                </ul>
                </div>
                <div class="footer-info">
                <p>{"2024 Earth Metabolome Initiative. All rights reserved."}</p>
                </div>
            </div>
        </footer>
    }
}
