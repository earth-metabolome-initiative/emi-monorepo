//! Module providing a yew component for a basic page with a websocket connection.
use crate::workers::ws_worker::{ComponentMessage, Tabular, WebsocketMessage};
use crate::workers::WebsocketWorker;
use serde::de::DeserializeOwned;
use web_common::database::PrimaryKey;
use web_common::database::*;
use yew::prelude::*;
use crate::router::AppRoute;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use web_common::api::form_traits::FormMethod;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;

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

    fn update_path(&self) -> Option<AppRoute>;
}

impl PageLike for NestedProject {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::ProjectsUpdate { id: self.inner.id })
    }
}

impl PageLike for NestedSampledIndividual {
    fn title(&self) -> String {
        format!("Sampled individual {}", self.inner.id)
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::SampledIndividualsUpdate { id: self.inner.id })
    }
}

impl PageLike for NestedSample {
    fn title(&self) -> String {
        format!("{}", self.inner.id)
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::SamplesUpdate { id: self.inner.id })
    }
}

impl PageLike for User {
    fn title(&self) -> String {
        self.full_name()
    }

    fn id(&self) -> PrimaryKey {
        self.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::UsersUpdate { id: self.id })
    }
}

impl PageLike for NestedTeam {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::TeamsUpdate { id: self.inner.id })
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct PageProps<Page>
where
    Page: PageLike,
{
    pub id: PrimaryKey,
    #[prop_or_default]
    _phantom: std::marker::PhantomData<Page>,
}


#[derive(Properties, Clone, PartialEq)]
pub struct InnerBasicPageProps<Page>
where
    Page: PageLike,
{
    pub id: PrimaryKey,
    pub navigator: yew_router::navigator::Navigator,
    #[prop_or_default]
    _phantom: std::marker::PhantomData<Page>,
}

pub struct InnerBasicPage<Page> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    page: Option<Page>,
    can_update: bool,
    can_delete: bool,
}

pub enum PageMessage {
    Backend(WebsocketMessage),
}

impl<Page: PageLike> Component for InnerBasicPage<Page> {
    type Message = PageMessage;
    type Properties = InnerBasicPageProps<Page>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: WebsocketMessage| {
                    link.send_message(PageMessage::Backend(message));
                }
            })),
            page: None,
            can_update: false,
            can_delete: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.websocket
                .send(ComponentMessage::can_update::<Page>(ctx.props().id));
            self.websocket
                .send(ComponentMessage::can_delete::<Page>(ctx.props().id));
            self.websocket
                .send(ComponentMessage::can_view::<Page>(ctx.props().id));
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PageMessage::Backend(message) => match message {
                WebsocketMessage::GetTable(_, row) => {
                    self.page = Some(bincode::deserialize(&row).unwrap());
                    true
                }
                WebsocketMessage::CanView(can_view) => {
                    if can_view {
                        self.websocket
                            .send(ComponentMessage::get::<Page>(ctx.props().id));
                    } else {
                        ctx.props().navigator.push(&AppRoute::Home);
                    }
                    true
                }
                WebsocketMessage::CanDelete(can_delete) => {
                    self.can_delete = can_delete;
                    true
                }
                WebsocketMessage::CanUpdate(can_update) => {
                    self.can_update = can_update;
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
                <div class="page">
                    <h2>{ page.title() }</h2>
                    if self.can_update {
                        <Link<AppRoute> classes={"button-like"} to={page.update_path().unwrap()}>
                            <i class={FormMethod::PUT.font_awesome_icon()}></i>
                            <span>{"Update"}</span>
                        </Link<AppRoute>>
                    } else {
                        <></>
                    }
                    <div class="clear"></div>
                </div>
            }
        } else {
            html! {
                <div>{"Loading..."}</div>
            }
        }
    }
}


#[function_component(BasicPage)]
pub fn basic_page<Page: PageLike>(props: &PageProps<Page>) -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <InnerBasicPage<Page> id={props.id} navigator={navigator} />
    }
}