//! Submodule providing the primary App component for the Yew application.
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yew_router::prelude::*;

use crate::{
    components::Footer,
    router::{AppRoute, switch},
    workers::DBWSWorker,
};

#[function_component]
/// Main application component.
pub fn App() -> Html {
    html! {
        <WorkerProvider<DBWSWorker> path="/dbws_worker.js">
            <BrowserRouter>
                <crate::components::Navigator />
                <div class="app">
                    <div class="page-container">
                        <Switch<AppRoute> render={switch} />
                    </div>
                    <Footer />
                </div>
            </BrowserRouter>
        </WorkerProvider<DBWSWorker>>
    }
}
