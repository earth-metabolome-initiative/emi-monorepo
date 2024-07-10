//! This module contains the forms for the frontend.
//!
//! This module is automatically generated. Do not write anything here.

use crate::components::forms::*;
use crate::workers::ws_worker::ComponentMessage;
use std::ops::Deref;
use std::rc::Rc;
use web_common::api::form_traits::FormMethod;
use web_common::api::ApiError;
use web_common::database::*;
use yew::prelude::*;
use yewdux::Dispatch;
use yewdux::{Reducer, Store};
#[derive(Store, PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct OrganismTaxonBuilder {
    pub organism: Option<Rc<web_common::database::nested_variants::NestedOrganism>>,
    pub taxon: Option<Rc<web_common::database::nested_variants::NestedTaxon>>,
    pub errors_organism: Vec<ApiError>,
    pub errors_taxon: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for OrganismTaxonBuilder {}
unsafe impl Sync for OrganismTaxonBuilder {}
impl Default for OrganismTaxonBuilder {
    fn default() -> Self {
        Self {
            organism: Default::default(),
            taxon: Default::default(),
            errors_organism: Default::default(),
            errors_taxon: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum OrganismTaxonActions {
    SetOrganism(Option<Rc<web_common::database::nested_variants::NestedOrganism>>),
    SetTaxon(Option<Rc<web_common::database::nested_variants::NestedTaxon>>),
}

impl FromOperation for OrganismTaxonActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "organism" => {
                OrganismTaxonActions::SetOrganism(Some(bincode::deserialize(&row).unwrap()))
            }
            "taxon" => OrganismTaxonActions::SetTaxon(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<OrganismTaxonBuilder> for OrganismTaxonActions {
    fn apply(
        self,
        mut state: std::rc::Rc<OrganismTaxonBuilder>,
    ) -> std::rc::Rc<OrganismTaxonBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            OrganismTaxonActions::SetOrganism(organism) => 'organism: {
                state_mut.errors_organism.clear();
                if organism.is_none() {
                    state_mut.errors_organism.push(ApiError::BadRequest(vec![
                        "The Organism field is required.".to_string(),
                    ]));
                    state_mut.organism = None;
                    break 'organism;
                }
                state_mut.organism = organism.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'organism;
            }
            OrganismTaxonActions::SetTaxon(taxon) => 'taxon: {
                state_mut.errors_taxon.clear();
                if taxon.is_none() {
                    state_mut.errors_taxon.push(ApiError::BadRequest(vec![
                        "The Taxon field is required.".to_string(),
                    ]));
                    state_mut.taxon = None;
                    break 'taxon;
                }
                state_mut.taxon = taxon.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'taxon;
            }
        }
        state
    }
}
impl FormBuilder for OrganismTaxonBuilder {
    type Actions = OrganismTaxonActions;

    type RichVariant = NestedOrganismTaxon;

    fn has_errors(&self) -> bool {
        !self.errors_organism.is_empty() || !self.errors_taxon.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.apply(OrganismTaxonActions::SetOrganism(
            Some(richest_variant.organism).map(Rc::from),
        ));
        dispatcher.apply(OrganismTaxonActions::SetTaxon(
            Some(richest_variant.taxon).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors() && self.organism.is_some() && self.taxon.is_some()
    }
}

impl From<OrganismTaxonBuilder> for NewOrganismTaxon {
    fn from(builder: OrganismTaxonBuilder) -> Self {
        Self {
            organism_id: builder
                .organism
                .as_ref()
                .map(|organism| organism.inner.id)
                .unwrap(),
            taxon_id: builder.taxon.as_ref().map(|taxon| taxon.inner.id).unwrap(),
        }
    }
}
impl FormBuildable for NewOrganismTaxon {
    type Builder = OrganismTaxonBuilder;
    fn title() -> &'static str {
        "Organism taxon"
    }
    fn task_target() -> &'static str {
        "Organism taxon"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateOrganismTaxonFormProp {
    #[prop_or_default]
    pub organism_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub taxon_id: Option<i32>,
}

#[function_component(CreateOrganismTaxonForm)]
pub fn create_organism_taxon_form(props: &CreateOrganismTaxonFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<OrganismTaxonBuilder>();
    if let Some(organism_id) = props.organism_id {
        named_requests.push(ComponentMessage::get_named::<&str, Organism>(
            "organism",
            organism_id.into(),
        ));
    }
    if let Some(taxon_id) = props.taxon_id {
        named_requests.push(ComponentMessage::get_named::<&str, Taxon>(
            "taxon",
            taxon_id.into(),
        ));
    }
    let set_organism = builder_dispatch.apply_callback(
        |organism: Option<Rc<web_common::database::nested_variants::NestedOrganism>>| {
            OrganismTaxonActions::SetOrganism(organism)
        },
    );
    let set_taxon = builder_dispatch.apply_callback(
        |taxon: Option<Rc<web_common::database::nested_variants::NestedTaxon>>| {
            OrganismTaxonActions::SetTaxon(taxon)
        },
    );
    html! {
        <BasicForm<NewOrganismTaxon>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<web_common::database::nested_variants::NestedOrganism, false> builder={set_organism} optional={false} errors={builder_store.errors_organism.clone()} value={builder_store.organism.clone()} label="Organism" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedTaxon, false> builder={set_taxon} optional={false} errors={builder_store.errors_taxon.clone()} value={builder_store.taxon.clone()} label="Taxon" scanner={false} />
        </BasicForm<NewOrganismTaxon>>
    }
}
