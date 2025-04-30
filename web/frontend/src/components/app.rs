//! Submodule providing the primary App component for the Yew application.
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yew_router::prelude::*;
use yewdux::{YewduxRoot, use_dispatch};

use crate::{
    components::Footer,
    pages::{Home, Login, NotFound},
    router::AppRoute,
    stores::app_state::AppState,
    utils::Dispatcher,
    workers::DBWSWorker,
};

#[function_component]
fn Root() -> Html {
    let dispatch = use_dispatch::<AppState>();
    html! {
        <BrowserRouter>
            <crate::components::Navigator {dispatch} />
            <div class="app">
                <div class="page-container">
                    <Switch<AppRoute> render={Callback::from({move |switch: AppRoute| {
                        match switch {
                            AppRoute::Home => {
                                html! { <Dispatcher<Home>/> }
                            }
                            AppRoute::Login => {
                                html! { <Dispatcher<Login>/> }
                            }
                            AppRoute::NotFound => {
                                html! { <Dispatcher<NotFound>/> }
                            }
                        }
                    }})}
                    />
                </div>
                <Footer />
            </div>
        </BrowserRouter>
    }
}

#[function_component]
/// Main application component.
pub fn App() -> Html {
    html! {
        <YewduxRoot>
            <WorkerProvider<DBWSWorker> path="/dbws_worker.js">
                <Root />
            </WorkerProvider<DBWSWorker>>
        </YewduxRoot>
    }
}
