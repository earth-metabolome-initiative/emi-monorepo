use crate::components::*;
use crate::router::{switch, AppRoute};
use crate::workers::WebsocketWorker;
use web_common::api::ws::messages::*;
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <WorkerProvider<WebsocketWorker<FrontendMessage, BackendMessage>> path="web_socket_worker.js">
            <BrowserRouter>
                <crate::components::NavigatorWrapper />
                <div class="app">
                    <Switch<AppRoute> render={switch} />
                    <Footer />
                </div>
            </BrowserRouter>
        </WorkerProvider<WebsocketWorker<FrontendMessage, BackendMessage>>>
    }
}
