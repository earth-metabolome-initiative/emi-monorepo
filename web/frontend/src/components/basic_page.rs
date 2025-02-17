//! Module providing a yew component for a basic page with a websocket
//! connection.
use std::rc::Rc;

use serde::de::DeserializeOwned;
use web_common::api::form_traits::FormMethod;
use yew::prelude::*;
use yew_agent::{prelude::WorkerBridgeHandle, scope_ext::AgentScopeExt};
use yew_router::{prelude::Link, scope_ext::RouterScopeExt};
use yewdux::Dispatch;

use super::RowToBadge;
use crate::{
    router::*,
    stores::user_state::UserState,
    workers::{
        ws_worker::{ComponentMessage, WebsocketMessage},
        WebsocketWorker,
    },
};

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct PageProps<Page>
where
    Page: PageLike,
{
    pub id: PrimaryKey,
    pub children: Html,
    #[prop_or_default]
    _phantom: std::marker::PhantomData<Page>,
}

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct BasicPageProps<Page>
where
    Page: PageLike,
{
    pub id: PrimaryKey,
    pub children: Html,
    #[prop_or_default]
    pub can_update: Callback<bool>,
    #[prop_or_default]
    pub can_admin: Callback<bool>,
    #[prop_or_default]
    _phantom: std::marker::PhantomData<Page>,
}

pub(crate) struct BasicPage<Page> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    page: Option<Page>,
    user_state: Rc<UserState>,
    _dispatcher: Dispatch<UserState>,
    can_update: bool,
    can_admin: bool,
}

pub(crate) enum PageMessage {
    Backend(WebsocketMessage),
    UserState(Rc<UserState>),
}

impl<Page: PageLike> Component for BasicPage<Page> {
    type Message = PageMessage;
    type Properties = BasicPageProps<Page>;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch =
            Dispatch::<UserState>::global().subscribe(ctx.link().callback(PageMessage::UserState));
        let user_state = user_dispatch.get();

        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: WebsocketMessage| {
                    link.send_message(PageMessage::Backend(message));
                }
            })),
            page: None,
            can_update: false,
            can_admin: false,
            user_state,
            _dispatcher: user_dispatch,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.page.as_ref().map_or(true, |p| p.id() != ctx.props().id) {
            self.websocket.send(ComponentMessage::can_view::<Page>(ctx.props().id));
            if self.user_state.has_user() {
                if Page::create_path(None).is_some() {
                    self.websocket.send(ComponentMessage::can_admin::<Page>(ctx.props().id));
                }
                self.websocket.send(ComponentMessage::can_update::<Page>(ctx.props().id));
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PageMessage::UserState(user_state) => {
                if user_state.has_user() {
                    self.websocket.send(ComponentMessage::can_admin::<Page>(ctx.props().id));
                    self.websocket.send(ComponentMessage::can_update::<Page>(ctx.props().id));
                }
                self.websocket.send(ComponentMessage::can_view::<Page>(ctx.props().id));

                self.user_state = user_state;
                true
            }
            PageMessage::Backend(message) => {
                match message {
                    WebsocketMessage::GetTable(_, row) => {
                        self.page = Some(bincode::deserialize(&row).unwrap());
                        true
                    }
                    WebsocketMessage::CanView(can_view) => {
                        if can_view {
                            self.websocket.send(ComponentMessage::get::<Page>(ctx.props().id));
                        } else {
                            ctx.link().navigator().unwrap().push(&AppRoute::Home);
                        }
                        true
                    }
                    WebsocketMessage::CanDelete(can_admin) => {
                        self.can_admin = can_admin;
                        ctx.props().can_admin.emit(can_admin);
                        true
                    }
                    WebsocketMessage::CanUpdate(can_update) => {
                        self.can_update = can_update;
                        ctx.props().can_update.emit(can_update);
                        true
                    }
                    _ => {
                        log::info!("Received message: {:?}", message);
                        false
                    }
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(page) = &self.page {
            html! {
                <div class="page">
                    <h2>
                        if let Some(icon) = page.font_awesome_icon() {
                            <i class={format!("fas fa-{}", icon)}></i>
                            {'\u{00a0}'}
                        }
                        <span>{ page.badge_title() }</span>
                    </h2>
                    if let Some(description) = page.description() {
                        <p>{ description }</p>
                    }
                    if self.can_update {
                        <Link<AppRoute> classes={"button-like update"} to={page.update_path().unwrap()}>
                            <i class={FormMethod::PUT.font_awesome_icon()}></i>
                            {'\u{00a0}'}
                            <span>{"Update"}</span>
                        </Link<AppRoute>>
                    }
                    if self.can_admin {
                        <Link<AppRoute> classes={"button-like delete"} to={page.update_path().unwrap()}>
                            <i class={FormMethod::DELETE.font_awesome_icon()}></i>
                            {'\u{00a0}'}
                            <span>{"Delete"}</span>
                        </Link<AppRoute>>
                    }
                    <div class="clear"></div>
                    { ctx.props().children.clone() }
                </div>
            }
        } else {
            html! {
                <div>{"Loading..."}</div>
            }
        }
    }
}
