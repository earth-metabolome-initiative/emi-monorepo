mod api;
mod components;
mod cookies;
mod pages;
mod router;
mod stores;

#[cfg(target_arch = "wasm32")]
/// While we are always compiling for WASM32, I have yet to figure out how to
/// let the RustAnalyzer know that. So, I have to use cfg to make it happy.
/// Fortunately, instead, cargo check is aware of the target architecture.
mod database;

#[cfg(target_arch = "wasm32")]
mod wasm {

    use crate::components::*;
    use crate::router::{switch, AppRoute};
    use crate::stores::{UserState, refresh_access_token};
    use web_common::user::User;
    use yew::prelude::*;
    use yew_router::prelude::*;
    use yewdux::prelude::*;

    #[function_component]
    pub fn App() -> Html {
        use crate::stores::user_state::logout;

        let (user_state, dispatch) = use_store::<UserState>();
        refresh_access_token(dispatch.clone());

        // In order to continuously check whether we are online, we need to create
        // a timed callback that is called multiple times every few seconds, say 5.
        // {
        //     let dispatch = dispatch.clone();
        //     use_effect_with((), move |_| {
        //         let callback = Closure::wrap(Box::new(move || {
        //             let is_online = web_sys::window().map(|w| w.navigator().on_line()).unwrap_or(false);
        //             dispatch.reduce_mut(move |store| {
        //                 store.set_offline(!is_online);
        //             });
        //         }) as Box<dyn FnMut()>);

        //         let window = web_sys::window().unwrap();
        //         let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
        //             callback.as_ref().unchecked_ref(),
        //             5000,
        //         );

        //         // We need to keep the callback alive, otherwise it will be deallocated.
        //         callback.forget();
        //     });
        // }

        html! {
            <BrowserRouter>
                <crate::components::Navigator />
                <div class="app">
                    <Switch<AppRoute> render={switch} />
                    <Footer />
                </div>
            </BrowserRouter>
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<wasm::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This application is only compiled to WebAssembly.")
}
