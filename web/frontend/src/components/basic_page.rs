//! Module providing a yew component for a basic page with a websocket connection.
use crate::workers::ws_worker::{ComponentMessage, Tabular, WebsocketMessage};
use crate::workers::WebsocketWorker;
use serde::de::DeserializeOwned;
use web_common::database::PrimaryKey;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

pub trait PageLike: DeserializeOwned + PartialEq + Clone + Tabular + 'static {
    fn title(&self) -> String;
}

#[derive(Properties, Clone, PartialEq)]
pub struct BasicPageProps<Page>
where
    Page: PageLike,
{
    pub id: PrimaryKey,
    _phantom: std::marker::PhantomData<Page>,
}

pub struct BasicPage<Page> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    page: Option<Page>,
}

pub enum PageMessage {
    Backend(WebsocketMessage),
}

impl<Page: PageLike> Component for BasicPage<Page> {
    type Message = PageMessage;
    type Properties = BasicPageProps<Page>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: WebsocketMessage| {
                    link.send_message(PageMessage::Backend(message));
                }
            })),
            page: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // If it is the first time that we are rendering this component,
            // we check whether the provided builder has an ID. If it does,
            // we retrieve from the backend the most up to date version of the
            // data that we are editing.
            self.websocket
                .send(ComponentMessage::get::<Page>(ctx.props().id));
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PageMessage::Backend(message) => match message {
                WebsocketMessage::GetTable(_, row) => {
                    self.page = Some(bincode::deserialize(&row).unwrap());
                    true
                }
                _ => {
                    log::info!("Received message: {:?}", message);
                    false
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(page) = &self.page {

            // We set the title of the webpage to the title of the page.
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .set_title(&page.title());

            // We render the page.

            html! {
                <div>
                    <h1>{ page.title() }</h1>
                </div>
            }
        } else {
            html! {
                <div>{"Loading..."}</div>
            }
        }
    }
}
