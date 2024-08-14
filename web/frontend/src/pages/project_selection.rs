//! Page where a logged user can select a project context to work on.
use crate::components::forms::Datalist;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use std::rc::Rc;
use web_common::database::NestedProject;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;
use yewdux::prelude::*;

pub struct ProjectSelection {
    user_state: Rc<UserState>,
    user_dispatch: Dispatch<UserState>,
}

pub enum ProjectSelectionMessage {
    SelectedProject(Option<Rc<NestedProject>>),
    UserState(Rc<UserState>),
}

#[derive(Clone, Properties, Eq, PartialEq)]
pub struct ProjectSelectionProperties {
    #[prop_or(None)]
    /// The page from which the user was redirected to this page,
    /// to which they can will be redirected back once they select a project.
    pub source_page: Option<AppRoute>,
}

impl Component for ProjectSelection {
    type Message = ProjectSelectionMessage;
    type Properties = ProjectSelectionProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let user_dispatch = Dispatch::<UserState>::global()
            .subscribe(ctx.link().callback(ProjectSelectionMessage::UserState));
        let user_state = user_dispatch.get();

        Self {
            user_state,
            user_dispatch,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectSelectionMessage::UserState(user_state) => {
                self.user_state = user_state;
                true
            }
            ProjectSelectionMessage::SelectedProject(project) => {
                if let Some(project) = project {
                    self.user_dispatch.reduce_mut(move |state| {
                        state.set_project(project.clone());
                    });
                    if let Some(source_page) = ctx.props().source_page.as_ref() {
                        ctx.link().navigator().unwrap().push(source_page);
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_project: Option<Rc<NestedProject>> = self.user_state.project();
        let link = ctx.link();

        // We prepare a callback for when the user selects a project.
        let selected_project = link.callback(move |project: Option<Rc<NestedProject>>| {
            ProjectSelectionMessage::SelectedProject(project)
        });

        html! {
            <div class="page">
                <h2>{"Select a project"}</h2>
                <p>{"Select a project to work on."}</p>
                <Datalist<web_common::database::nested_variants::NestedProject, true> builder={selected_project} optional={false} value={current_project} label="Select project" scanner={false} />
            </div>
        }
    }
}
