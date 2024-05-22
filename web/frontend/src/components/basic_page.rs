//! Module providing a yew component for a basic page with a websocket connection.
use std::rc::Rc;

use crate::router::AppRoute;
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

pub trait PageLike: DeserializeOwned + Filtrable + PartialEq + Clone + Tabular + 'static {
    fn title(&self) -> String;

    fn section() -> String {
        let mut section = Self::TABLE.to_string().replace("_", " ");
        if let Some(r) = section.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
        section
    }

    fn description(&self) -> Option<&str>;

    fn id(&self) -> PrimaryKey;

    fn update_path(&self) -> Option<AppRoute>;

    /// Create a path to create a new item.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the path.
    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute>;

    /// Returns font-awesome icon associated with the page type.
    fn icon() -> &'static str;
}

impl PageLike for NestedProject {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn description(&self) -> Option<&str> {
        Some(&self.inner.description)
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::ProjectsUpdate { id: self.inner.id })
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        filter
            .and_then(|f| {
                f.parent_project_id.map(|parent_project_id| {
                    AppRoute::ProjectsNewWithParentProject { parent_project_id }
                })
            })
            .or(Some(AppRoute::ProjectsNew))
    }

    fn icon() -> &'static str {
        "diagram-project"
    }
}

impl PageLike for NestedObservation {
    fn title(&self) -> String {
        format!("Observation {}", self.inner.id)
    }

    fn description(&self) -> Option<&str> {
        self.inner.notes.as_deref()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        None
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        filter
            .and_then(|f| {
                f.project_id
                    .map(|project_id| AppRoute::ObservationsNewWithProject { project_id })
            })
            .or(Some(AppRoute::ObservationsNew))
    }

    fn icon() -> &'static str {
        "tower-observation"
    }
}

impl PageLike for NestedSpectraCollection {
    fn title(&self) -> String {
        format!("Spectra collection {}", self.inner.id)
    }

    fn description(&self) -> Option<&str> {
        self.inner.notes.as_deref()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::SpectraCollectionsUpdate { id: self.inner.id })
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        filter
            .and_then(|f| {
                f.sample_id
                    .map(|sample_id| AppRoute::SpectraCollectionsNewWithSample { sample_id })
            })
            .or(Some(AppRoute::SpectraCollectionsNew))
    }

    fn icon() -> &'static str {
        "flask-vial"
    }
}

impl PageLike for NestedSpectra {
    fn title(&self) -> String {
        format!("Spectra {}", self.inner.id)
    }

    fn description(&self) -> Option<&str> {
        self.inner.notes.as_deref()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        None
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        // filter
        //     .and_then(|f| {
        //         f.spectra_collection_id.map(|spectra_collection_id| {
        //             AppRoute::SpectraNewWithSpectraCollection { spectra_collection_id }
        //         })
        //     })
        //     .or(Some(AppRoute::SpectraNew))
        None
    }

    fn icon() -> &'static str {
        "vials"
    }
}

impl PageLike for NestedSampledIndividual {
    fn title(&self) -> String {
        format!("Sampled individual {}", self.inner.id)
    }

    fn description(&self) -> Option<&str> {
        self.inner.notes.as_deref()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::SampledIndividualsUpdate { id: self.inner.id })
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        filter
            .and_then(|f| {
                f.project_id
                    .map(|project_id| AppRoute::SampledIndividualsNewWithProject { project_id })
            })
            .or(Some(AppRoute::SampledIndividualsNew))
    }

    fn icon() -> &'static str {
        "dna"
    }
}

impl PageLike for NestedSample {
    fn title(&self) -> String {
        format!("{}", self.inner.id)
    }

    fn description(&self) -> Option<&str> {
        self.inner.notes.as_deref()
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::SamplesUpdate { id: self.inner.id })
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        filter
            .and_then(|f| {
                f.sampled_by
                    .map(|sampled_by| AppRoute::SamplesNewWithSampledBy { sampled_by })
            })
            .or(Some(AppRoute::SamplesNew))
    }

    fn icon() -> &'static str {
        "vial"
    }
}

impl PageLike for User {
    fn title(&self) -> String {
        self.full_name()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn id(&self) -> PrimaryKey {
        self.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::UsersUpdate { id: self.id })
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        None
    }

    fn icon() -> &'static str {
        "users"
    }
}

impl PageLike for NestedTeam {
    fn title(&self) -> String {
        self.inner.name.clone()
    }

    fn description(&self) -> Option<&str> {
        Some(self.inner.description.as_str())
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        Some(AppRoute::TeamsUpdate { id: self.inner.id })
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        filter
            .and_then(|f| {
                f.parent_team_id
                    .map(|parent_team_id| AppRoute::TeamsNewWithParentTeam { parent_team_id })
            })
            .or(Some(AppRoute::TeamsNew))
    }

    fn icon() -> &'static str {
        "people-group"
    }
}

impl PageLike for NestedSampleContainer {
    fn title(&self) -> String {
        self.inner.barcode.clone()
    }

    fn description(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> PrimaryKey {
        self.inner.id.into()
    }

    fn update_path(&self) -> Option<AppRoute> {
        None
    }

    fn create_path(filter: Option<&Self::Filter>) -> Option<AppRoute> {
        Some(AppRoute::SampleContainersNew)
    }

    fn icon() -> &'static str {
        "box"
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct PageProps<Page>
where
    Page: PageLike,
{
    pub id: PrimaryKey,
    pub children: Html,
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
    pub children: Html,
    #[prop_or_default]
    _phantom: std::marker::PhantomData<Page>,
}

pub struct InnerBasicPage<Page> {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    page: Option<Page>,
    user_state: Rc<UserState>,
    _dispatcher: Dispatch<UserState>,
    can_update: bool,
    can_delete: bool,
}

pub enum PageMessage {
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
            can_delete: false,
            user_state,
            _dispatcher: user_dispatch,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.websocket
                .send(ComponentMessage::can_view::<Page>(ctx.props().id));
            if self.user_state.has_user() {
                self.websocket
                    .send(ComponentMessage::can_delete::<Page>(ctx.props().id));
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
                        .send(ComponentMessage::can_delete::<Page>(ctx.props().id));
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
                    if self.can_delete {
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
pub fn basic_page<Page: PageLike>(props: &PageProps<Page>) -> Html {
    let navigator = use_navigator().unwrap();
    html! {
        <InnerBasicPage<Page> id={props.id} navigator={navigator} >
            { props.children.clone() }
        </InnerBasicPage<Page>>
    }
}
