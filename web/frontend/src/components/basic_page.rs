//! Module providing a yew component for a basic page with a websocket connection.
use std::rc::Rc;

use crate::router::*;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::{ComponentMessage, WebsocketMessage};
use crate::workers::WebsocketWorker;
use serde::de::DeserializeOwned;
use web_common::api::form_traits::FormMethod;
use web_common::database::PrimaryKey;
use web_common::database::*;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;
use yewdux::Dispatch;

use super::RowToBadge;

pub(crate) trait PageLike:
    RowToBadge + DeserializeOwned + Filtrable + Viewable + PartialEq + Clone + Tabular + 'static
{
    fn section() -> String {
        let mut section = Self::TABLE.to_string().replace("_", " ");
        if let Some(r) = section.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
        section
    }

    fn id(&self) -> PrimaryKey;

    fn update_path(&self) -> Option<AppRoute> {
        None
    }

    /// Create a path to create a new item.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the path.
    fn create_path(_filter: Option<&Self::Filter>) -> Option<AppRoute> {
        None
    }

    /// Returns font-awesome icon associated with the page type.
    fn icon() -> &'static str;
}

impl PageLike for NestedBioOttRank {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn icon() -> &'static str {
        "dna"
    }
}

impl PageLike for NestedDerivedSample {
    fn id(&self) -> PrimaryKey {
        (self.inner.parent_sample_id, self.inner.child_sample_id).into()
    }

    fn icon() -> &'static str {
        "vial"
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }
}

impl PageLike for NestedBioOttTaxonItem {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn icon() -> &'static str {
        "dna"
    }
}

impl PageLike for NestedOrganismBioOttTaxonItem {
    fn id(&self) -> PrimaryKey {
        (self.inner.organism_id, self.inner.taxon_id).into()
    }

    fn icon() -> &'static str {
        "dna"
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedSampleBioOttTaxonItem {
    fn id(&self) -> PrimaryKey {
        (self.inner.sample_id, self.inner.taxon_id).into()
    }

    fn icon() -> &'static str {
        "dna"
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedTeamsUsersRoleRequest {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        NestedTeam::icon()
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for Country {
    fn id(&self) -> PrimaryKey {
        self.id.into()
    }

    fn icon() -> &'static str {
        "globe"
    }
}

impl PageLike for NestedSampleState {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn icon() -> &'static str {
        "vial"
    }
}

impl PageLike for NestedUsersUsersRole {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        "users"
    }
}

impl PageLike for NestedTeamsUsersRole {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        NestedTeam::icon()
    }
}

impl PageLike for NestedUsersUsersRoleRequest {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        "users"
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedUsersUsersRoleInvitation {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        "users"
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedTeamsUsersRoleInvitation {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        NestedTeam::icon()
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedTeamsTeamsRoleInvitation {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.team_id).into()
    }

    fn icon() -> &'static str {
        NestedTeam::icon()
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedProjectsUsersRoleRequest {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        NestedProject::icon()
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedProjectsUsersRoleInvitation {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        NestedProject::icon()
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedProjectsUsersRole {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.user_id).into()
    }

    fn icon() -> &'static str {
        NestedProject::icon()
    }
}

impl PageLike for NestedProjectsTeamsRoleRequest {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.team_id).into()
    }

    fn icon() -> &'static str {
        NestedProject::icon()
    }
    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedProjectsTeamsRole {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.team_id).into()
    }

    fn icon() -> &'static str {
        NestedProject::icon()
    }
}

impl PageLike for NestedProjectsTeamsRoleInvitation {
    fn id(&self) -> PrimaryKey {
        (self.inner.table_id, self.inner.team_id).into()
    }

    fn icon() -> &'static str {
        NestedProject::icon()
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }
}

impl PageLike for NestedProject {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "diagram-project"
    }
}

impl PageLike for NestedOrganization {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn icon() -> &'static str {
        "sitemap"
    }
}

impl PageLike for NestedObservationSubject {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn icon() -> &'static str {
        "dna"
    }
}

impl PageLike for NestedObservation {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "tower-observation"
    }
}

impl PageLike for NestedSpectraCollection {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }
    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "flask-vial"
    }
}

impl PageLike for NestedNameplate {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "tag"
    }
}

impl PageLike for NestedOrganism {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "dna"
    }
}

impl PageLike for NestedSample {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "vial"
    }
}

impl PageLike for NestedUser {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn icon() -> &'static str {
        "users"
    }
}

impl PageLike for NestedTeam {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "people-group"
    }
}

impl PageLike for NestedSampleContainer {
    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(self.update_route())
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(Self::new_route(filter))
    }

    fn icon() -> &'static str {
        "box"
    }
}

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
pub(crate) struct InnerBasicPageProps<Page>
where
    Page: PageLike,
{
    pub id: PrimaryKey,
    pub navigator: yew_router::navigator::Navigator,
    pub children: Html,
    #[prop_or_default]
    _phantom: std::marker::PhantomData<Page>,
}

pub(crate) struct InnerBasicPage<Page> {
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

impl<Page: PageLike> Component for InnerBasicPage<Page> {
    type Message = PageMessage;
    type Properties = InnerBasicPageProps<Page>;

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
        if self
            .page
            .as_ref()
            .map_or(true, |p| p.id() != ctx.props().id)
        {
            self.websocket
                .send(ComponentMessage::can_view::<Page>(ctx.props().id));
            if self.user_state.has_user() {
                if Page::create_path(None).is_some() {
                    self.websocket
                        .send(ComponentMessage::can_admin::<Page>(ctx.props().id));
                }
                self.websocket
                    .send(ComponentMessage::can_update::<Page>(ctx.props().id));
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PageMessage::UserState(user_state) => {
                if user_state.has_user() {
                    self.websocket
                        .send(ComponentMessage::can_admin::<Page>(ctx.props().id));
                    self.websocket
                        .send(ComponentMessage::can_update::<Page>(ctx.props().id));
                }
                self.websocket
                    .send(ComponentMessage::can_view::<Page>(ctx.props().id));

                self.user_state = user_state;
                true
            }
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
                WebsocketMessage::CanDelete(can_admin) => {
                    self.can_admin = can_admin;
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

#[function_component(BasicPage)]
pub(crate) fn basic_page<Page: PageLike>(props: &PageProps<Page>) -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <InnerBasicPage<Page> id={props.id} navigator={navigator} >
            { props.children.clone() }
        </InnerBasicPage<Page>>
    }
}
