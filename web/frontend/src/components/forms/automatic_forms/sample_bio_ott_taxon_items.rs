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
#[derive(Store, PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SampleBioOttTaxonItemBuilder {
    pub sample: Option<Rc<NestedSample>>,
    pub taxon: Option<Rc<NestedBioOttTaxonItem>>,
    pub errors_sample: Vec<ApiError>,
    pub errors_taxon: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for SampleBioOttTaxonItemBuilder {}
unsafe impl Sync for SampleBioOttTaxonItemBuilder {}
impl Default for SampleBioOttTaxonItemBuilder {
    fn default() -> Self {
        Self {
            sample: Default::default(),
            taxon: Default::default(),
            errors_sample: Default::default(),
            errors_taxon: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum SampleBioOttTaxonItemActions {
    SetSample(Option<Rc<NestedSample>>),
    SetTaxon(Option<Rc<NestedBioOttTaxonItem>>),
}

impl FromOperation for SampleBioOttTaxonItemActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "sample" => {
                SampleBioOttTaxonItemActions::SetSample(Some(bincode::deserialize(&row).unwrap()))
            }
            "taxon" => {
                SampleBioOttTaxonItemActions::SetTaxon(Some(bincode::deserialize(&row).unwrap()))
            }
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<SampleBioOttTaxonItemBuilder> for SampleBioOttTaxonItemActions {
    fn apply(
        self,
        mut state: std::rc::Rc<SampleBioOttTaxonItemBuilder>,
    ) -> std::rc::Rc<SampleBioOttTaxonItemBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampleBioOttTaxonItemActions::SetSample(sample) => 'sample: {
                state_mut.errors_sample.clear();
                if sample.is_none() {
                    state_mut.errors_sample.push(ApiError::BadRequest(vec![
                        "The Sample field is required.".to_string(),
                    ]));
                    state_mut.sample = None;
                    break 'sample;
                }
                state_mut.sample = sample.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sample;
            }
            SampleBioOttTaxonItemActions::SetTaxon(taxon) => 'taxon: {
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
impl FormBuilder for SampleBioOttTaxonItemBuilder {
    type Actions = SampleBioOttTaxonItemActions;

    type RichVariant = NestedSampleBioOttTaxonItem;

    fn has_errors(&self) -> bool {
        !self.errors_sample.is_empty() || !self.errors_taxon.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.apply(SampleBioOttTaxonItemActions::SetSample(
            Some(richest_variant.sample).map(Rc::from),
        ));
        dispatcher.apply(SampleBioOttTaxonItemActions::SetTaxon(
            Some(richest_variant.taxon).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors() && self.sample.is_some() && self.taxon.is_some()
    }
}

impl From<SampleBioOttTaxonItemBuilder> for NewSampleBioOttTaxonItem {
    fn from(builder: SampleBioOttTaxonItemBuilder) -> Self {
        Self {
            sample_id: builder
                .sample
                .as_ref()
                .map(|sample| sample.inner.id)
                .unwrap(),
            taxon_id: builder.taxon.as_ref().map(|taxon| taxon.inner.id).unwrap(),
        }
    }
}
impl FormBuildable for NewSampleBioOttTaxonItem {
    type Builder = SampleBioOttTaxonItemBuilder;
    fn title() -> &'static str {
        "Sample bio ott taxon item"
    }
    fn task_target() -> &'static str {
        "Sample bio ott taxon item"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampleBioOttTaxonItemFormProp {
    #[prop_or_default]
    pub sample_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub taxon_id: Option<i32>,
}

#[function_component(CreateSampleBioOttTaxonItemForm)]
pub fn create_sample_bio_ott_taxon_item_form(props: &CreateSampleBioOttTaxonItemFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<SampleBioOttTaxonItemBuilder>();
    if let Some(sample_id) = props.sample_id {
        named_requests.push(ComponentMessage::get_named::<&str, Sample>(
            "sample",
            sample_id.into(),
        ));
    }
    if let Some(taxon_id) = props.taxon_id {
        named_requests.push(ComponentMessage::get_named::<&str, BioOttTaxonItem>(
            "taxon",
            taxon_id.into(),
        ));
    }
    let set_sample = builder_dispatch.apply_callback(|sample: Option<Rc<NestedSample>>| {
        SampleBioOttTaxonItemActions::SetSample(sample)
    });
    let set_taxon = builder_dispatch.apply_callback(|taxon: Option<Rc<NestedBioOttTaxonItem>>| {
        SampleBioOttTaxonItemActions::SetTaxon(taxon)
    });
    html! {
        <BasicForm<NewSampleBioOttTaxonItem>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedSample, false> builder={set_sample} optional={false} errors={builder_store.errors_sample.clone()} value={builder_store.sample.clone()} label="Sample" scanner={false} />
            <Datalist<NestedBioOttTaxonItem, false> builder={set_taxon} optional={false} errors={builder_store.errors_taxon.clone()} value={builder_store.taxon.clone()} label="Taxon" scanner={false} />
        </BasicForm<NewSampleBioOttTaxonItem>>
    }
}
