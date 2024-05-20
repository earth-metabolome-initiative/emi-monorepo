//! Module providing a yew component for a basic page with a websocket connection.
use crate::components::PageLike;
use crate::router::AppRoute;
use crate::workers::ws_worker::{ComponentMessage, WebsocketMessage};
use crate::workers::WebsocketWorker;
use web_common::api::form_traits::FormMethod;
use web_common::database::*;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::prelude::Link;

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
}

pub enum PagesMessage {
    Backend(WebsocketMessage),
    LoadMore,
}

impl<Page: Filtrable + PageLike + RowToBadge> Component for BasicList<Page> {
    type Message = PagesMessage;
    type Properties = BasicListProps<Page>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: WebsocketMessage| {
                    link.send_message(PagesMessage::Backend(message));
                }
            })),
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
            PagesMessage::Backend(message) => match message {
                WebsocketMessage::AllTable(rows) => {
                    log::info!("Received {} rows", rows.len());
                    let new_pages: Vec<Page> = rows
                        .into_iter()
                        .map(|row| bincode::deserialize(&row).unwrap())
                        .filter(|page: &Page| {
                            self.pages.iter().all(|old_page| old_page.id() != page.id())
                        })
                        .collect();

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
                        10,
                        self.pages.len() as i64,
                    ));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="section_explorer">
                <h2>{Page::section()}</h2>
                <ul>
                { for self.pages.iter().map(|page| html!{<li>{page.to_badge()}</li>}) }
                if self.no_more_pages {
                    <li>{"There are no more entries to load"}</li>
                } else {
                    <></>
                }
                </ul>
                if let Some(create_path) = Page::create_path() {
                    <Link<AppRoute> classes={"button-like create"} to={create_path}>
                        <i class={FormMethod::POST.font_awesome_icon()}></i>
                        <span>{"New"}</span>
                    </Link<AppRoute>>
                } else {
                    <></>
                }
                <button class="retrieve" onclick={ctx.link().callback(|_| PagesMessage::LoadMore)} disabled={self.request_is_ongoing}>
                    if self.request_is_ongoing {
                        <i class="fas fa-arrows-rotate fa-spin"></i>
                    } else {
                        <i class="fas fa-arrows-rotate"></i>
                    }
                    <span>{"Load more entries"}</span>
                </button>
                <div class="clear"></div>
            </div>
        }
    }
}
