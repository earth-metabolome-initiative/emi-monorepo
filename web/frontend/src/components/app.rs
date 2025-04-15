use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yew_router::prelude::*;

use crate::{
    components::*,
    router::{AppRoute, switch},
    workers::*,
};

#[function_component]
pub fn App() -> Html {
    html! {
        <WorkerProvider<WebsocketWorker> path="/web_socket_worker.js">
            <BrowserRouter>
                <crate::components::Navigator />
                <div class="app">
                    <div class="page-container">
                        <Switch<AppRoute> render={switch} />
                    </div>
                    <Footer />
                </div>
            </BrowserRouter>
        </WorkerProvider<WebsocketWorker>>
    }
}
