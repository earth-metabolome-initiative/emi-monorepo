//! Module providing a yew component for a basic page with a websocket connection.
use crate::workers::ws_worker::{ComponentMessage, Tabular, WebsocketMessage};
use crate::workers::WebsocketWorker;
use web_common::database::*;
use serde::de::DeserializeOwned;
use web_common::database::PrimaryKey;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

pub trait PageLike: DeserializeOwned + PartialEq + Clone + Tabular + 'static {
    fn title(&self) -> String;

    fn section() -> String {
        let mut section = Self::TABLE.to_string().replace("_", " ");
        if let Some(r) = section.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
        section
    }

    fn id(&self) -> PrimaryKey;
}

impl PageLike for NestedContainerHorizontalRule {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedContainerVerticalRule {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedItemCategory {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedProcedure {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedProjectRequirement {
    fn title(&self) -> String {
        format!(
            "Requirement for project {}",
            self.project.inner.name
        )
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedProject {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedSamplingProcedure {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedSampledIndividual {
    fn title(&self) -> String {
        format!(
            "Sampled individual {}",
            self.inner.id
        )
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for NestedSample {
    fn title(&self) -> String {
        format!(
            "Sample {}",
            self.inner.id
        )
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
}

impl PageLike for User {
    fn title(&self) -> String {
        self.full_name()
    }

    fn id(&self) -> PrimaryKey {
        self.id.into()
    }
}

impl PageLike for NestedTeam {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }
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
