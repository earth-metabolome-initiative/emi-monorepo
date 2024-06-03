//! Module providing a yew component for a basic page with a websocket connection.
use super::RowToBadge;
use crate::components::forms::Datalist;
use crate::components::PageLike;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use std::rc::Rc;
use web_common::api::form_traits::FormMethod;
use web_common::database::*;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
/// Properties for a BasicList component.
pub struct BasicListProps<Page: Filtrable> {
    #[prop_or_default]
    pub column_name: Option<String>,
    /// The filters to apply to the list.
    #[prop_or_default]
    pub filters: Option<Page::Filter>,
}

pub struct BasicList<Page> {
    // websocket: WorkerBridgeHandle<WebsocketWorker>,
    user_state: Rc<UserState>,
    _dispatcher: Dispatch<UserState>,
    _phantom: std::marker::PhantomData<Page>,
}

pub enum PagesMessage {
    UserState(Rc<UserState>),
}

impl<Page: Filtrable + Searchable<false> + PageLike + RowToBadge> Component for BasicList<Page> {
    type Message = PagesMessage;
    type Properties = BasicListProps<Page>;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch =
            Dispatch::<UserState>::global().subscribe(ctx.link().callback(PagesMessage::UserState));
        let user_state = user_dispatch.get();

        Self {
            // websocket: ctx.link().bridge_worker(Callback::from({
            //     let link = ctx.link().clone();
            //     move |message: WebsocketMessage| {
            //         link.send_message(PagesMessage::Backend(message));
            //     }
            // })),
            user_state,
            _dispatcher: user_dispatch,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PagesMessage::UserState(user_state) => {
                self.user_state = user_state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let header_type = if ctx.props().filters.is_some() {
            "h3"
        } else {
            "h2"
        };

        html! {
            <div class="page">
                <@{header_type}>
                    if ctx.props().filters.is_some() {
                        // If we are currently in a subsection, we add the paragraph icon to allow
                        // the user to refer directly to this subsection.
                        <a href={format!("#{}", Page::section())}>
                            <i class="fas fa-paragraph"></i>
                        </a>
                        {'\u{00a0}'}
                    }
                    <Link<AppRoute> to={Page::list_route()}>
                        <i class={format!("fas fa-{}", Page::icon())}></i>
                        {'\u{00a0}'}
                        <span>{Page::section()}</span>
                        if let Some(column) = ctx.props().column_name.as_ref() {
                            {'\u{00a0}'}
                            <span>{column.clone()}</span>
                        }
                    </Link<AppRoute>>
                </@>
                <Datalist<Page, false> label={format!("Search {}", Page::section())} filters={ctx.props().filters.clone()} always_shows_candidates={true} />
                if self.user_state.has_user() {
                    if let Some(create_path) = Page::create_path(ctx.props().filters.as_ref()) {
                        <Link<AppRoute> classes={"button-like create"} to={create_path}>
                            <i class={FormMethod::POST.font_awesome_icon()}></i>
                            {'\u{00a0}'}
                            <span>{"New"}</span>
                        </Link<AppRoute>>
                    }
                }
                <div class="clear"></div>
            </div>
        }
    }
}
