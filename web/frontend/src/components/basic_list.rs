//! Module providing a yew component for a basic page with a websocket connection.
use crate::components::PageLike;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::{ComponentMessage, WebsocketMessage};
use crate::workers::WebsocketWorker;
use std::rc::Rc;
use web_common::api::form_traits::FormMethod;
use web_common::database::*;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::Link;
use yewdux::prelude::*;

use super::database::row_to_badge::RowToBadge;

#[derive(Clone, Debug, PartialEq, Properties)]
/// Properties for a BasicList component.
pub struct BasicListProps<Page: Filtrable> {
    /// The filters to apply to the list.
    #[prop_or_default]
    pub filters: Option<Page::Filter>,
}

pub struct BasicList<Page> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    pages: Vec<Page>,
    no_more_pages: bool,
    request_is_ongoing: bool,
    user_state: Rc<UserState>,
    _dispatcher: Dispatch<UserState>,
}

pub enum PagesMessage {
    Backend(WebsocketMessage),
    LoadMore,
    UserState(Rc<UserState>),
}

impl<Page: Filtrable + PageLike + RowToBadge> Component for BasicList<Page> {
    type Message = PagesMessage;
    type Properties = BasicListProps<Page>;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch =
            Dispatch::<UserState>::global().subscribe(ctx.link().callback(PagesMessage::UserState));
        let user_state = user_dispatch.get();

        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: WebsocketMessage| {
                    link.send_message(PagesMessage::Backend(message));
                }
            })),
            user_state,
            _dispatcher: user_dispatch,
            no_more_pages: false,
            request_is_ongoing: false,
            pages: Vec::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(PagesMessage::LoadMore);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PagesMessage::UserState(user_state) => {
                self.user_state = user_state;
                true
            }
            PagesMessage::Backend(message) => match message {
                WebsocketMessage::AllTable(rows) => {
                    let mut new_pages: Vec<Page> = bincode::deserialize(&rows).unwrap();
                    new_pages.retain(|page| {
                        self.pages.iter().all(|old_page| old_page.id() != page.id())
                    });

                    self.no_more_pages = new_pages.is_empty();
                    self.request_is_ongoing = false;

                    self.pages.extend(new_pages);
                    true
                }
                _ => {
                    log::info!("Received message: {:?}", message);
                    false
                }
            },
            PagesMessage::LoadMore => {
                self.request_is_ongoing = true;
                self.websocket
                    .send(ComponentMessage::all_by_updated_at::<Page>(
                        ctx.props().filters.clone(),
                        24,
                        self.pages.len() as i64,
                    ));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="page">
                if ctx.props().filters.is_some() {
                    <h3>
                        <i class={format!("fas fa-{}", Page::icon())}></i>
                        {'\u{00a0}'}
                        <span>{Page::section()}</span>
                    </h3>
                } else {
                    <h2>
                        <i class={format!("fas fa-{}", Page::icon())}></i>
                        {'\u{00a0}'}
                        <span>{Page::section()}</span>
                    </h2>
                }
                <ul class="badges-container">
                { for self.pages.iter().map(|page| html!{<li>{page.to_badge()}</li>}) }
                if self.no_more_pages {
                    <li>{"There are no more entries to load"}</li>
                }
                </ul>
                if self.user_state.has_user() {
                    if let Some(create_path) = Page::create_path(ctx.props().filters.as_ref()) {
                        <Link<AppRoute> classes={"button-like create"} to={create_path}>
                            <i class={FormMethod::POST.font_awesome_icon()}></i>
                            {'\u{00a0}'}
                            <span>{"New"}</span>
                        </Link<AppRoute>>
                    }
                }
                <button class="retrieve" onclick={ctx.link().callback(|_| PagesMessage::LoadMore)} disabled={self.request_is_ongoing}>
                    if self.request_is_ongoing {
                        <i class="fas fa-arrows-rotate fa-spin"></i>
                    } else {
                        <i class="fas fa-arrows-rotate"></i>
                    }
                    {'\u{00a0}'}
                    <span>{"Load more entries"}</span>
                </button>
                <div class="clear"></div>
            </div>
        }
    }
}
