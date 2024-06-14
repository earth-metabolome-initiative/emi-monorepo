use crate::components::*;
use crate::router::{switch, AppRoute};
use crate::workers::*;
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <WorkerProvider<WebsocketWorker> path="/web_socket_worker.js">
            <BrowserRouter>
                <crate::components::NavigatorWrapper />
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
