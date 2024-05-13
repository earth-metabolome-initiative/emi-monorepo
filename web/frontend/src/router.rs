//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::*;
use uuid::Uuid;
use crate::components::forms::automatic_forms::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/container_horizontal_rules")]
    ContainerHorizontalRules,
    #[at("/container_horizontal_rules/:id")]
    ContainerHorizontalRulesView{id: i32},
    #[at("/container_horizontal_rules/new")]
    ContainerHorizontalRulesNew,
    #[at("/container_horizontal_rules/:id/update")]
    ContainerHorizontalRulesUpdate{id: i32},
    #[at("/container_vertical_rules")]
    ContainerVerticalRules,
    #[at("/container_vertical_rules/:id")]
    ContainerVerticalRulesView{id: i32},
    #[at("/container_vertical_rules/new")]
    ContainerVerticalRulesNew,
    #[at("/container_vertical_rules/:id/update")]
    ContainerVerticalRulesUpdate{id: i32},
    #[at("/item_categories")]
    ItemCategories,
    #[at("/item_categories/:id")]
    ItemCategoriesView{id: i32},
    #[at("/item_categories/new")]
    ItemCategoriesNew,
    #[at("/item_categories/:id/update")]
    ItemCategoriesUpdate{id: i32},
    #[at("/procedures")]
    Procedures,
    #[at("/procedures/:id")]
    ProceduresView{id: i32},
    #[at("/procedures/new")]
    ProceduresNew,
    #[at("/procedures/:id/update")]
    ProceduresUpdate{id: i32},
    #[at("/project_requirements")]
    ProjectRequirements,
    #[at("/project_requirements/:id")]
    ProjectRequirementsView{id: i32},
    #[at("/project_requirements/new")]
    ProjectRequirementsNew,
    #[at("/project_requirements/:id/update")]
    ProjectRequirementsUpdate{id: i32},
    #[at("/projects")]
    Projects,
    #[at("/projects/:id")]
    ProjectsView{id: i32},
    #[at("/projects/new")]
    ProjectsNew,
    #[at("/projects/:id/update")]
    ProjectsUpdate{id: i32},
    #[at("/sampled_individuals")]
    SampledIndividuals,
    #[at("/sampled_individuals/:id")]
    SampledIndividualsView{id: Uuid},
    #[at("/sampled_individuals/new")]
    SampledIndividualsNew,
    #[at("/sampled_individuals/:id/update")]
    SampledIndividualsUpdate{id: Uuid},
    #[at("/samples")]
    Samples,
    #[at("/samples/:id")]
    SamplesView{id: Uuid},
    #[at("/samples/new")]
    SamplesNew,
    #[at("/samples/:id/update")]
    SamplesUpdate{id: Uuid},
    #[at("/sampling_procedures")]
    SamplingProcedures,
    #[at("/sampling_procedures/:id")]
    SamplingProceduresView{id: Uuid},
    #[at("/sampling_procedures/new")]
    SamplingProceduresNew,
    #[at("/sampling_procedures/:id/update")]
    SamplingProceduresUpdate{id: Uuid},
    #[at("/teams")]
    Teams,
    #[at("/teams/:id")]
    TeamsView{id: i32},
    #[at("/teams/new")]
    TeamsNew,
    #[at("/teams/:id/update")]
    TeamsUpdate{id: i32},
    #[at("/users")]
    Users,
    #[at("/users/:id")]
    UsersView{id: i32},
    #[at("/users/:id/update")]
    UsersUpdate{id: i32},
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The switch to map each instance of the AppRoute to the corresponding page.
///
/// # Arguments
/// * `route` - The route to map.
///
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::ContainerHorizontalRules => {
             html! { <span>{"ContainerHorizontalRules page"}</span> }
        }
        AppRoute::ContainerHorizontalRulesView{id} => {
             html! { <span>{"Specific Container horizontal rule page"}</span> }
        }
        AppRoute::ContainerHorizontalRulesNew => {
            html! { <CreateContainerHorizontalRuleForm /> }
        }
        AppRoute::ContainerHorizontalRulesUpdate{id} => {
            html! { <UpdateContainerHorizontalRuleForm id={id} /> }
        }
        AppRoute::ContainerVerticalRules => {
             html! { <span>{"ContainerVerticalRules page"}</span> }
        }
        AppRoute::ContainerVerticalRulesView{id} => {
             html! { <span>{"Specific Container vertical rule page"}</span> }
        }
        AppRoute::ContainerVerticalRulesNew => {
            html! { <CreateContainerVerticalRuleForm /> }
        }
        AppRoute::ContainerVerticalRulesUpdate{id} => {
            html! { <UpdateContainerVerticalRuleForm id={id} /> }
        }
        AppRoute::ItemCategories => {
             html! { <span>{"ItemCategories page"}</span> }
        }
        AppRoute::ItemCategoriesView{id} => {
             html! { <span>{"Specific Item category page"}</span> }
        }
        AppRoute::ItemCategoriesNew => {
            html! { <CreateItemCategoryForm /> }
        }
        AppRoute::ItemCategoriesUpdate{id} => {
            html! { <UpdateItemCategoryForm id={id} /> }
        }
        AppRoute::Procedures => {
             html! { <span>{"Procedures page"}</span> }
        }
        AppRoute::ProceduresView{id} => {
             html! { <span>{"Specific Procedure page"}</span> }
        }
        AppRoute::ProceduresNew => {
            html! { <CreateProcedureForm /> }
        }
        AppRoute::ProceduresUpdate{id} => {
            html! { <UpdateProcedureForm id={id} /> }
        }
        AppRoute::ProjectRequirements => {
             html! { <span>{"ProjectRequirements page"}</span> }
        }
        AppRoute::ProjectRequirementsView{id} => {
             html! { <span>{"Specific Project requirement page"}</span> }
        }
        AppRoute::ProjectRequirementsNew => {
            html! { <CreateProjectRequirementForm /> }
        }
        AppRoute::ProjectRequirementsUpdate{id} => {
            html! { <UpdateProjectRequirementForm id={id} /> }
        }
        AppRoute::Projects => {
             html! { <span>{"Projects page"}</span> }
        }
        AppRoute::ProjectsView{id} => {
             html! { <span>{"Specific Project page"}</span> }
        }
        AppRoute::ProjectsNew => {
            html! { <CreateProjectForm /> }
        }
        AppRoute::ProjectsUpdate{id} => {
            html! { <UpdateProjectForm id={id} /> }
        }
        AppRoute::SampledIndividuals => {
             html! { <span>{"SampledIndividuals page"}</span> }
        }
        AppRoute::SampledIndividualsView{id} => {
             html! { <span>{"Specific Sampled individual page"}</span> }
        }
        AppRoute::SampledIndividualsNew => {
            html! { <CreateSampledIndividualForm /> }
        }
        AppRoute::SampledIndividualsUpdate{id} => {
            html! { <UpdateSampledIndividualForm id={id} /> }
        }
        AppRoute::Samples => {
             html! { <span>{"Samples page"}</span> }
        }
        AppRoute::SamplesView{id} => {
             html! { <span>{"Specific Sample page"}</span> }
        }
        AppRoute::SamplesNew => {
            html! { <CreateSampleForm /> }
        }
        AppRoute::SamplesUpdate{id} => {
            html! { <UpdateSampleForm id={id} /> }
        }
        AppRoute::SamplingProcedures => {
             html! { <span>{"SamplingProcedures page"}</span> }
        }
        AppRoute::SamplingProceduresView{id} => {
             html! { <span>{"Specific Sampling procedure page"}</span> }
        }
        AppRoute::SamplingProceduresNew => {
            html! { <CreateSamplingProcedureForm /> }
        }
        AppRoute::SamplingProceduresUpdate{id} => {
            html! { <UpdateSamplingProcedureForm id={id} /> }
        }
        AppRoute::Teams => {
             html! { <span>{"Teams page"}</span> }
        }
        AppRoute::TeamsView{id} => {
             html! { <span>{"Specific Team page"}</span> }
        }
        AppRoute::TeamsNew => {
            html! { <CreateTeamForm /> }
        }
        AppRoute::TeamsUpdate{id} => {
            html! { <UpdateTeamForm id={id} /> }
        }
        AppRoute::Users => {
             html! { <span>{"Users page"}</span> }
        }
        AppRoute::UsersView{id} => {
             html! { <span>{"Specific User page"}</span> }
        }
        AppRoute::UsersUpdate{id} => {
            html! { <UpdateUserForm id={id} /> }
        }
        AppRoute::Home => {
            html! { <Home /> }
        }
        AppRoute::Login => {
            html! { <Login /> }
        }
        AppRoute::NotFound => {
            html! { <NotFound /> }
        }
    }
}
