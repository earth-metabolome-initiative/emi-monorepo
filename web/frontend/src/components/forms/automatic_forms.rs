//! This module contains the forms for the frontend.
//!
//! This module is automatically generated. Do not write anything here.

use crate::components::forms::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::rc::Rc;
use uuid::Uuid;
use web_common::api::form_traits::FormMethod;
use web_common::api::ApiError;
use web_common::database::*;
use yew::prelude::*;
use yewdux::{use_store, Reducer, Store};

#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewBioOttRankBuilder {
    pub font_awesome_icon: Option<FontAwesomeIcon>,
    pub name: Option<String>,
    pub errors_font_awesome_icon: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewBioOttRankBuilderActions {
    SetFontAwesomeIcon(Option<FontAwesomeIcon>),
    SetName(Option<String>),
}

impl FormBuilder for NestedNewBioOttRankBuilder {
    type Data = NewBioOttRank;
    type Actions = NestedNewBioOttRankBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_font_awesome_icon.is_empty() || !self.errors_name.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewBioOttRankBuilder> for NewBioOttRank {
    fn from(builder: NestedNewBioOttRankBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            font_awesome_icon_id: builder.font_awesome_icon.unwrap().id,
        }
    }
}
impl FormBuildable for NewBioOttRank {
    type Builder = NestedNewBioOttRankBuilder;
    const TABLE: Table = Table::BioOttRanks;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewBioOttRank"
    }
    fn task_target() -> &'static str {
        "NewBioOttRank"
    }
    fn description() -> &'static str {
        concat!("Create a new NewBioOttRank.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewBioOttRankBuilder> for NestedNewBioOttRankBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewBioOttRankBuilder>,
    ) -> std::rc::Rc<NestedNewBioOttRankBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewBioOttRankBuilderActions::SetFontAwesomeIcon(font_awesome_icon) => {
                if font_awesome_icon.is_none() {
                    state_mut
                        .errors_font_awesome_icon
                        .push(ApiError::BadRequest(vec![
                            "The FontAwesomeIcon field is required.".to_string(),
                        ]));
                }
                state_mut.font_awesome_icon = font_awesome_icon;
            }
            NestedNewBioOttRankBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
        }
        state
    }
}
#[function_component(NewBioOttRankForm)]
pub fn new_bio_ott_rank_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewBioOttRankBuilder>();
    let set_font_awesome_icon =
        builder_dispatch.apply_callback(|font_awesome_icon: Option<FontAwesomeIcon>| {
            NestedNewBioOttRankBuilderActions::SetFontAwesomeIcon(font_awesome_icon)
        });
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NestedNewBioOttRankBuilderActions::SetName(name));
    html! {
        <BasicForm<NewBioOttRank> builder={builder_store.deref().clone()}>
            <Datalist<FontAwesomeIcon> builder={set_font_awesome_icon} errors={builder_store.errors_font_awesome_icon.clone()} value={builder_store.font_awesome_icon.clone()} label="FontAwesomeIcon" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewBioOttRank>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewBioOttTaxonItemBuilder {
    pub ott_rank: Option<NestedBioOttRank>,
    pub domain: Option<NestedBioOttTaxonItem>,
    pub kingdom: Option<NestedBioOttTaxonItem>,
    pub phylum: Option<NestedBioOttTaxonItem>,
    pub class: Option<NestedBioOttTaxonItem>,
    pub order: Option<NestedBioOttTaxonItem>,
    pub family: Option<NestedBioOttTaxonItem>,
    pub genus: Option<NestedBioOttTaxonItem>,
    pub parent: Option<NestedBioOttTaxonItem>,
    pub font_awesome_icon: Option<FontAwesomeIcon>,
    pub color: Option<Color>,
    pub name: Option<String>,
    pub ott_id: Option<i32>,
    pub wikidata_id: Option<i32>,
    pub ncbi_id: Option<i32>,
    pub gbif_id: Option<i32>,
    pub irmng_id: Option<i32>,
    pub worms_id: Option<i32>,
    pub errors_ott_rank: Vec<ApiError>,
    pub errors_domain: Vec<ApiError>,
    pub errors_kingdom: Vec<ApiError>,
    pub errors_phylum: Vec<ApiError>,
    pub errors_class: Vec<ApiError>,
    pub errors_order: Vec<ApiError>,
    pub errors_family: Vec<ApiError>,
    pub errors_genus: Vec<ApiError>,
    pub errors_parent: Vec<ApiError>,
    pub errors_font_awesome_icon: Vec<ApiError>,
    pub errors_color: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_ott_id: Vec<ApiError>,
    pub errors_wikidata_id: Vec<ApiError>,
    pub errors_ncbi_id: Vec<ApiError>,
    pub errors_gbif_id: Vec<ApiError>,
    pub errors_irmng_id: Vec<ApiError>,
    pub errors_worms_id: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewBioOttTaxonItemBuilderActions {
    SetOttRank(Option<NestedBioOttRank>),
    SetDomain(Option<NestedBioOttTaxonItem>),
    SetKingdom(Option<NestedBioOttTaxonItem>),
    SetPhylum(Option<NestedBioOttTaxonItem>),
    SetClass(Option<NestedBioOttTaxonItem>),
    SetOrder(Option<NestedBioOttTaxonItem>),
    SetFamily(Option<NestedBioOttTaxonItem>),
    SetGenus(Option<NestedBioOttTaxonItem>),
    SetParent(Option<NestedBioOttTaxonItem>),
    SetFontAwesomeIcon(Option<FontAwesomeIcon>),
    SetColor(Option<Color>),
    SetName(Option<String>),
    SetOttId(Option<i32>),
    SetWikidataId(Option<i32>),
    SetNcbiId(Option<i32>),
    SetGbifId(Option<i32>),
    SetIrmngId(Option<i32>),
    SetWormsId(Option<i32>),
}

impl FormBuilder for NestedNewBioOttTaxonItemBuilder {
    type Data = NewBioOttTaxonItem;
    type Actions = NestedNewBioOttTaxonItemBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_ott_rank.is_empty()
            || !self.errors_domain.is_empty()
            || !self.errors_kingdom.is_empty()
            || !self.errors_phylum.is_empty()
            || !self.errors_class.is_empty()
            || !self.errors_order.is_empty()
            || !self.errors_family.is_empty()
            || !self.errors_genus.is_empty()
            || !self.errors_parent.is_empty()
            || !self.errors_font_awesome_icon.is_empty()
            || !self.errors_color.is_empty()
            || !self.errors_name.is_empty()
            || !self.errors_ott_id.is_empty()
            || !self.errors_wikidata_id.is_empty()
            || !self.errors_ncbi_id.is_empty()
            || !self.errors_gbif_id.is_empty()
            || !self.errors_irmng_id.is_empty()
            || !self.errors_worms_id.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewBioOttTaxonItemBuilder> for NewBioOttTaxonItem {
    fn from(builder: NestedNewBioOttTaxonItemBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            ott_id: builder.ott_id.unwrap(),
            ott_rank_id: builder.ott_rank.unwrap().inner.id,
            wikidata_id: builder.wikidata_id,
            ncbi_id: builder.ncbi_id,
            gbif_id: builder.gbif_id,
            irmng_id: builder.irmng_id,
            worms_id: builder.worms_id,
            domain_id: builder.domain.map(|domain| domain.inner.id),
            kingdom_id: builder.kingdom.map(|kingdom| kingdom.inner.id),
            phylum_id: builder.phylum.map(|phylum| phylum.inner.id),
            class_id: builder.class.map(|class| class.inner.id),
            order_id: builder.order.map(|order| order.inner.id),
            family_id: builder.family.map(|family| family.inner.id),
            genus_id: builder.genus.map(|genus| genus.inner.id),
            parent_id: builder.parent.unwrap().inner.id,
            font_awesome_icon_id: builder.font_awesome_icon.unwrap().id,
            color_id: builder.color.unwrap().id,
        }
    }
}
impl FormBuildable for NewBioOttTaxonItem {
    type Builder = NestedNewBioOttTaxonItemBuilder;
    const TABLE: Table = Table::BioOttTaxonItems;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewBioOttTaxonItem"
    }
    fn task_target() -> &'static str {
        "NewBioOttTaxonItem"
    }
    fn description() -> &'static str {
        concat!("Create a new NewBioOttTaxonItem.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewBioOttTaxonItemBuilder> for NestedNewBioOttTaxonItemBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewBioOttTaxonItemBuilder>,
    ) -> std::rc::Rc<NestedNewBioOttTaxonItemBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewBioOttTaxonItemBuilderActions::SetOttRank(ott_rank) => {
                if ott_rank.is_none() {
                    state_mut.errors_ott_rank.push(ApiError::BadRequest(vec![
                        "The OttRank field is required.".to_string(),
                    ]));
                }
                state_mut.ott_rank = ott_rank;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetDomain(domain) => {
                state_mut.domain = domain;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetKingdom(kingdom) => {
                state_mut.kingdom = kingdom;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetPhylum(phylum) => {
                state_mut.phylum = phylum;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetClass(class) => {
                state_mut.class = class;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetOrder(order) => {
                state_mut.order = order;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetFamily(family) => {
                state_mut.family = family;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetGenus(genus) => {
                state_mut.genus = genus;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetParent(parent) => {
                if parent.is_none() {
                    state_mut.errors_parent.push(ApiError::BadRequest(vec![
                        "The Parent field is required.".to_string(),
                    ]));
                }
                state_mut.parent = parent;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetFontAwesomeIcon(font_awesome_icon) => {
                if font_awesome_icon.is_none() {
                    state_mut
                        .errors_font_awesome_icon
                        .push(ApiError::BadRequest(vec![
                            "The FontAwesomeIcon field is required.".to_string(),
                        ]));
                }
                state_mut.font_awesome_icon = font_awesome_icon;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetColor(color) => {
                if color.is_none() {
                    state_mut.errors_color.push(ApiError::BadRequest(vec![
                        "The Color field is required.".to_string(),
                    ]));
                }
                state_mut.color = color;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetOttId(ott_id) => {
                if ott_id.is_none() {
                    state_mut.errors_ott_id.push(ApiError::BadRequest(vec![
                        "The OttId field is required.".to_string(),
                    ]));
                }
                state_mut.ott_id = ott_id;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetWikidataId(wikidata_id) => {
                state_mut.wikidata_id = wikidata_id;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetNcbiId(ncbi_id) => {
                state_mut.ncbi_id = ncbi_id;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetGbifId(gbif_id) => {
                state_mut.gbif_id = gbif_id;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetIrmngId(irmng_id) => {
                state_mut.irmng_id = irmng_id;
            }
            NestedNewBioOttTaxonItemBuilderActions::SetWormsId(worms_id) => {
                state_mut.worms_id = worms_id;
            }
        }
        state
    }
}
#[function_component(NewBioOttTaxonItemForm)]
pub fn new_bio_ott_taxon_item_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewBioOttTaxonItemBuilder>();
    let set_ott_rank = builder_dispatch.apply_callback(|ott_rank: Option<NestedBioOttRank>| {
        NestedNewBioOttTaxonItemBuilderActions::SetOttRank(ott_rank)
    });
    let set_domain = builder_dispatch.apply_callback(|domain: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetDomain(domain)
    });
    let set_kingdom = builder_dispatch.apply_callback(|kingdom: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetKingdom(kingdom)
    });
    let set_phylum = builder_dispatch.apply_callback(|phylum: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetPhylum(phylum)
    });
    let set_class = builder_dispatch.apply_callback(|class: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetClass(class)
    });
    let set_order = builder_dispatch.apply_callback(|order: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetOrder(order)
    });
    let set_family = builder_dispatch.apply_callback(|family: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetFamily(family)
    });
    let set_genus = builder_dispatch.apply_callback(|genus: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetGenus(genus)
    });
    let set_parent = builder_dispatch.apply_callback(|parent: Option<NestedBioOttTaxonItem>| {
        NestedNewBioOttTaxonItemBuilderActions::SetParent(parent)
    });
    let set_font_awesome_icon =
        builder_dispatch.apply_callback(|font_awesome_icon: Option<FontAwesomeIcon>| {
            NestedNewBioOttTaxonItemBuilderActions::SetFontAwesomeIcon(font_awesome_icon)
        });
    let set_color = builder_dispatch.apply_callback(|color: Option<Color>| {
        NestedNewBioOttTaxonItemBuilderActions::SetColor(color)
    });
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| {
        NestedNewBioOttTaxonItemBuilderActions::SetName(name)
    });
    let set_ott_id = builder_dispatch.apply_callback(|ott_id: Option<i32>| {
        NestedNewBioOttTaxonItemBuilderActions::SetOttId(ott_id)
    });
    let set_wikidata_id = builder_dispatch.apply_callback(|wikidata_id: Option<i32>| {
        NestedNewBioOttTaxonItemBuilderActions::SetWikidataId(wikidata_id)
    });
    let set_ncbi_id = builder_dispatch.apply_callback(|ncbi_id: Option<i32>| {
        NestedNewBioOttTaxonItemBuilderActions::SetNcbiId(ncbi_id)
    });
    let set_gbif_id = builder_dispatch.apply_callback(|gbif_id: Option<i32>| {
        NestedNewBioOttTaxonItemBuilderActions::SetGbifId(gbif_id)
    });
    let set_irmng_id = builder_dispatch.apply_callback(|irmng_id: Option<i32>| {
        NestedNewBioOttTaxonItemBuilderActions::SetIrmngId(irmng_id)
    });
    let set_worms_id = builder_dispatch.apply_callback(|worms_id: Option<i32>| {
        NestedNewBioOttTaxonItemBuilderActions::SetWormsId(worms_id)
    });
    html! {
        <BasicForm<NewBioOttTaxonItem> builder={builder_store.deref().clone()}>
            <Datalist<NestedBioOttRank> builder={set_ott_rank} errors={builder_store.errors_ott_rank.clone()} value={builder_store.ott_rank.clone()} label="OttRank" />
            <Datalist<NestedBioOttTaxonItem> builder={set_domain} errors={builder_store.errors_domain.clone()} value={builder_store.domain.clone()} label="Domain" />
            <Datalist<NestedBioOttTaxonItem> builder={set_kingdom} errors={builder_store.errors_kingdom.clone()} value={builder_store.kingdom.clone()} label="Kingdom" />
            <Datalist<NestedBioOttTaxonItem> builder={set_phylum} errors={builder_store.errors_phylum.clone()} value={builder_store.phylum.clone()} label="Phylum" />
            <Datalist<NestedBioOttTaxonItem> builder={set_class} errors={builder_store.errors_class.clone()} value={builder_store.class.clone()} label="Class" />
            <Datalist<NestedBioOttTaxonItem> builder={set_order} errors={builder_store.errors_order.clone()} value={builder_store.order.clone()} label="Order" />
            <Datalist<NestedBioOttTaxonItem> builder={set_family} errors={builder_store.errors_family.clone()} value={builder_store.family.clone()} label="Family" />
            <Datalist<NestedBioOttTaxonItem> builder={set_genus} errors={builder_store.errors_genus.clone()} value={builder_store.genus.clone()} label="Genus" />
            <Datalist<NestedBioOttTaxonItem> builder={set_parent} errors={builder_store.errors_parent.clone()} value={builder_store.parent.clone()} label="Parent" />
            <Datalist<FontAwesomeIcon> builder={set_font_awesome_icon} errors={builder_store.errors_font_awesome_icon.clone()} value={builder_store.font_awesome_icon.clone()} label="FontAwesomeIcon" />
            <Datalist<Color> builder={set_color} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewBioOttTaxonItem>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewColorBuilder {
    pub name: Option<String>,
    pub hexadecimal_value: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_hexadecimal_value: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewColorBuilderActions {
    SetName(Option<String>),
    SetHexadecimalValue(Option<String>),
}

impl FormBuilder for NewColorBuilder {
    type Data = NewColor;
    type Actions = NewColorBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty() || !self.errors_hexadecimal_value.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewColorBuilder> for NewColor {
    fn from(builder: NewColorBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            hexadecimal_value: builder.hexadecimal_value.unwrap(),
        }
    }
}
impl FormBuildable for NewColor {
    type Builder = NewColorBuilder;
    const TABLE: Table = Table::Colors;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewColor"
    }
    fn task_target() -> &'static str {
        "NewColor"
    }
    fn description() -> &'static str {
        concat!("Create a new NewColor.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewColorBuilder> for NewColorBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NewColorBuilder>) -> std::rc::Rc<NewColorBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewColorBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NewColorBuilderActions::SetHexadecimalValue(hexadecimal_value) => {
                if hexadecimal_value.is_none() {
                    state_mut
                        .errors_hexadecimal_value
                        .push(ApiError::BadRequest(vec![
                            "The HexadecimalValue field is required.".to_string(),
                        ]));
                }
                state_mut.hexadecimal_value = hexadecimal_value;
            }
        }
        state
    }
}
#[function_component(NewColorForm)]
pub fn new_color_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewColorBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewColorBuilderActions::SetName(name));
    let set_hexadecimal_value =
        builder_dispatch.apply_callback(|hexadecimal_value: Option<String>| {
            NewColorBuilderActions::SetHexadecimalValue(hexadecimal_value)
        });
    html! {
        <BasicForm<NewColor> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="HexadecimalValue" errors={builder_store.errors_hexadecimal_value.clone()} builder={set_hexadecimal_value} value={builder_store.hexadecimal_value.clone()} input_type={InputType::Text} />
        </BasicForm<NewColor>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewContainerHorizontalRuleBuilder {
    pub item_type: Option<NestedItemCategory>,
    pub other_item_type: Option<NestedItemCategory>,
    pub name: Option<String>,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
    pub errors_item_type: Vec<ApiError>,
    pub errors_other_item_type: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_minimum_temperature: Vec<ApiError>,
    pub errors_maximum_temperature: Vec<ApiError>,
    pub errors_minimum_humidity: Vec<ApiError>,
    pub errors_maximum_humidity: Vec<ApiError>,
    pub errors_minimum_pressure: Vec<ApiError>,
    pub errors_maximum_pressure: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewContainerHorizontalRuleBuilderActions {
    SetItemType(Option<NestedItemCategory>),
    SetOtherItemType(Option<NestedItemCategory>),
    SetName(Option<String>),
    SetMinimumTemperature(Option<i32>),
    SetMaximumTemperature(Option<i32>),
    SetMinimumHumidity(Option<i32>),
    SetMaximumHumidity(Option<i32>),
    SetMinimumPressure(Option<i32>),
    SetMaximumPressure(Option<i32>),
}

impl FormBuilder for NestedNewContainerHorizontalRuleBuilder {
    type Data = NewContainerHorizontalRule;
    type Actions = NestedNewContainerHorizontalRuleBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_item_type.is_empty()
            || !self.errors_other_item_type.is_empty()
            || !self.errors_name.is_empty()
            || !self.errors_minimum_temperature.is_empty()
            || !self.errors_maximum_temperature.is_empty()
            || !self.errors_minimum_humidity.is_empty()
            || !self.errors_maximum_humidity.is_empty()
            || !self.errors_minimum_pressure.is_empty()
            || !self.errors_maximum_pressure.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewContainerHorizontalRuleBuilder> for NewContainerHorizontalRule {
    fn from(builder: NestedNewContainerHorizontalRuleBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            item_type_id: builder.item_type.unwrap().inner.id,
            other_item_type_id: builder.other_item_type.unwrap().inner.id,
            minimum_temperature: builder.minimum_temperature,
            maximum_temperature: builder.maximum_temperature,
            minimum_humidity: builder.minimum_humidity,
            maximum_humidity: builder.maximum_humidity,
            minimum_pressure: builder.minimum_pressure,
            maximum_pressure: builder.maximum_pressure,
        }
    }
}
impl FormBuildable for NewContainerHorizontalRule {
    type Builder = NestedNewContainerHorizontalRuleBuilder;
    const TABLE: Table = Table::ContainerHorizontalRules;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewContainerHorizontalRule"
    }
    fn task_target() -> &'static str {
        "NewContainerHorizontalRule"
    }
    fn description() -> &'static str {
        concat!("Create a new NewContainerHorizontalRule.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewContainerHorizontalRuleBuilder>
    for NestedNewContainerHorizontalRuleBuilderActions
{
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewContainerHorizontalRuleBuilder>,
    ) -> std::rc::Rc<NestedNewContainerHorizontalRuleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewContainerHorizontalRuleBuilderActions::SetItemType(item_type) => {
                if item_type.is_none() {
                    state_mut.errors_item_type.push(ApiError::BadRequest(vec![
                        "The ItemType field is required.".to_string(),
                    ]));
                }
                state_mut.item_type = item_type;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetOtherItemType(other_item_type) => {
                if other_item_type.is_none() {
                    state_mut
                        .errors_other_item_type
                        .push(ApiError::BadRequest(vec![
                            "The OtherItemType field is required.".to_string(),
                        ]));
                }
                state_mut.other_item_type = other_item_type;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMinimumTemperature(
                minimum_temperature,
            ) => {
                state_mut.minimum_temperature = minimum_temperature;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMaximumTemperature(
                maximum_temperature,
            ) => {
                state_mut.maximum_temperature = maximum_temperature;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMinimumHumidity(
                minimum_humidity,
            ) => {
                state_mut.minimum_humidity = minimum_humidity;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMaximumHumidity(
                maximum_humidity,
            ) => {
                state_mut.maximum_humidity = maximum_humidity;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMinimumPressure(
                minimum_pressure,
            ) => {
                state_mut.minimum_pressure = minimum_pressure;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMaximumPressure(
                maximum_pressure,
            ) => {
                state_mut.maximum_pressure = maximum_pressure;
            }
        }
        state
    }
}
#[function_component(NewContainerHorizontalRuleForm)]
pub fn new_container_horizontal_rule_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewContainerHorizontalRuleBuilder>();
    let set_item_type = builder_dispatch.apply_callback(|item_type: Option<NestedItemCategory>| {
        NestedNewContainerHorizontalRuleBuilderActions::SetItemType(item_type)
    });
    let set_other_item_type =
        builder_dispatch.apply_callback(|other_item_type: Option<NestedItemCategory>| {
            NestedNewContainerHorizontalRuleBuilderActions::SetOtherItemType(other_item_type)
        });
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| {
        NestedNewContainerHorizontalRuleBuilderActions::SetName(name)
    });
    let set_minimum_temperature =
        builder_dispatch.apply_callback(|minimum_temperature: Option<i32>| {
            NestedNewContainerHorizontalRuleBuilderActions::SetMinimumTemperature(
                minimum_temperature,
            )
        });
    let set_maximum_temperature =
        builder_dispatch.apply_callback(|maximum_temperature: Option<i32>| {
            NestedNewContainerHorizontalRuleBuilderActions::SetMaximumTemperature(
                maximum_temperature,
            )
        });
    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<i32>| {
        NestedNewContainerHorizontalRuleBuilderActions::SetMinimumHumidity(minimum_humidity)
    });
    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<i32>| {
        NestedNewContainerHorizontalRuleBuilderActions::SetMaximumHumidity(maximum_humidity)
    });
    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<i32>| {
        NestedNewContainerHorizontalRuleBuilderActions::SetMinimumPressure(minimum_pressure)
    });
    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<i32>| {
        NestedNewContainerHorizontalRuleBuilderActions::SetMaximumPressure(maximum_pressure)
    });
    html! {
        <BasicForm<NewContainerHorizontalRule> builder={builder_store.deref().clone()}>
            <Datalist<NestedItemCategory> builder={set_item_type} errors={builder_store.errors_item_type.clone()} value={builder_store.item_type.clone()} label="ItemType" />
            <Datalist<NestedItemCategory> builder={set_other_item_type} errors={builder_store.errors_other_item_type.clone()} value={builder_store.other_item_type.clone()} label="OtherItemType" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewContainerHorizontalRule>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewContainerVerticalRuleBuilder {
    pub container_item_type: Option<NestedItemCategory>,
    pub contained_item_type: Option<NestedItemCategory>,
    pub name: Option<String>,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
    pub errors_container_item_type: Vec<ApiError>,
    pub errors_contained_item_type: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_minimum_temperature: Vec<ApiError>,
    pub errors_maximum_temperature: Vec<ApiError>,
    pub errors_minimum_humidity: Vec<ApiError>,
    pub errors_maximum_humidity: Vec<ApiError>,
    pub errors_minimum_pressure: Vec<ApiError>,
    pub errors_maximum_pressure: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewContainerVerticalRuleBuilderActions {
    SetContainerItemType(Option<NestedItemCategory>),
    SetContainedItemType(Option<NestedItemCategory>),
    SetName(Option<String>),
    SetMinimumTemperature(Option<i32>),
    SetMaximumTemperature(Option<i32>),
    SetMinimumHumidity(Option<i32>),
    SetMaximumHumidity(Option<i32>),
    SetMinimumPressure(Option<i32>),
    SetMaximumPressure(Option<i32>),
}

impl FormBuilder for NestedNewContainerVerticalRuleBuilder {
    type Data = NewContainerVerticalRule;
    type Actions = NestedNewContainerVerticalRuleBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_container_item_type.is_empty()
            || !self.errors_contained_item_type.is_empty()
            || !self.errors_name.is_empty()
            || !self.errors_minimum_temperature.is_empty()
            || !self.errors_maximum_temperature.is_empty()
            || !self.errors_minimum_humidity.is_empty()
            || !self.errors_maximum_humidity.is_empty()
            || !self.errors_minimum_pressure.is_empty()
            || !self.errors_maximum_pressure.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewContainerVerticalRuleBuilder> for NewContainerVerticalRule {
    fn from(builder: NestedNewContainerVerticalRuleBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            container_item_type_id: builder.container_item_type.unwrap().inner.id,
            contained_item_type_id: builder.contained_item_type.unwrap().inner.id,
            minimum_temperature: builder.minimum_temperature,
            maximum_temperature: builder.maximum_temperature,
            minimum_humidity: builder.minimum_humidity,
            maximum_humidity: builder.maximum_humidity,
            minimum_pressure: builder.minimum_pressure,
            maximum_pressure: builder.maximum_pressure,
        }
    }
}
impl FormBuildable for NewContainerVerticalRule {
    type Builder = NestedNewContainerVerticalRuleBuilder;
    const TABLE: Table = Table::ContainerVerticalRules;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewContainerVerticalRule"
    }
    fn task_target() -> &'static str {
        "NewContainerVerticalRule"
    }
    fn description() -> &'static str {
        concat!("Create a new NewContainerVerticalRule.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewContainerVerticalRuleBuilder>
    for NestedNewContainerVerticalRuleBuilderActions
{
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewContainerVerticalRuleBuilder>,
    ) -> std::rc::Rc<NestedNewContainerVerticalRuleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewContainerVerticalRuleBuilderActions::SetContainerItemType(
                container_item_type,
            ) => {
                if container_item_type.is_none() {
                    state_mut
                        .errors_container_item_type
                        .push(ApiError::BadRequest(vec![
                            "The ContainerItemType field is required.".to_string(),
                        ]));
                }
                state_mut.container_item_type = container_item_type;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetContainedItemType(
                contained_item_type,
            ) => {
                if contained_item_type.is_none() {
                    state_mut
                        .errors_contained_item_type
                        .push(ApiError::BadRequest(vec![
                            "The ContainedItemType field is required.".to_string(),
                        ]));
                }
                state_mut.contained_item_type = contained_item_type;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMinimumTemperature(
                minimum_temperature,
            ) => {
                state_mut.minimum_temperature = minimum_temperature;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMaximumTemperature(
                maximum_temperature,
            ) => {
                state_mut.maximum_temperature = maximum_temperature;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMinimumHumidity(minimum_humidity) => {
                state_mut.minimum_humidity = minimum_humidity;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMaximumHumidity(maximum_humidity) => {
                state_mut.maximum_humidity = maximum_humidity;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMinimumPressure(minimum_pressure) => {
                state_mut.minimum_pressure = minimum_pressure;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMaximumPressure(maximum_pressure) => {
                state_mut.maximum_pressure = maximum_pressure;
            }
        }
        state
    }
}
#[function_component(NewContainerVerticalRuleForm)]
pub fn new_container_vertical_rule_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewContainerVerticalRuleBuilder>();
    let set_container_item_type =
        builder_dispatch.apply_callback(|container_item_type: Option<NestedItemCategory>| {
            NestedNewContainerVerticalRuleBuilderActions::SetContainerItemType(container_item_type)
        });
    let set_contained_item_type =
        builder_dispatch.apply_callback(|contained_item_type: Option<NestedItemCategory>| {
            NestedNewContainerVerticalRuleBuilderActions::SetContainedItemType(contained_item_type)
        });
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| {
        NestedNewContainerVerticalRuleBuilderActions::SetName(name)
    });
    let set_minimum_temperature =
        builder_dispatch.apply_callback(|minimum_temperature: Option<i32>| {
            NestedNewContainerVerticalRuleBuilderActions::SetMinimumTemperature(minimum_temperature)
        });
    let set_maximum_temperature =
        builder_dispatch.apply_callback(|maximum_temperature: Option<i32>| {
            NestedNewContainerVerticalRuleBuilderActions::SetMaximumTemperature(maximum_temperature)
        });
    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<i32>| {
        NestedNewContainerVerticalRuleBuilderActions::SetMinimumHumidity(minimum_humidity)
    });
    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<i32>| {
        NestedNewContainerVerticalRuleBuilderActions::SetMaximumHumidity(maximum_humidity)
    });
    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<i32>| {
        NestedNewContainerVerticalRuleBuilderActions::SetMinimumPressure(minimum_pressure)
    });
    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<i32>| {
        NestedNewContainerVerticalRuleBuilderActions::SetMaximumPressure(maximum_pressure)
    });
    html! {
        <BasicForm<NewContainerVerticalRule> builder={builder_store.deref().clone()}>
            <Datalist<NestedItemCategory> builder={set_container_item_type} errors={builder_store.errors_container_item_type.clone()} value={builder_store.container_item_type.clone()} label="ContainerItemType" />
            <Datalist<NestedItemCategory> builder={set_contained_item_type} errors={builder_store.errors_contained_item_type.clone()} value={builder_store.contained_item_type.clone()} label="ContainedItemType" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewContainerVerticalRule>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewDocumentFormatBuilder {
    pub extension: Option<String>,
    pub mime_type: Option<String>,
    pub errors_extension: Vec<ApiError>,
    pub errors_mime_type: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewDocumentFormatBuilderActions {
    SetExtension(Option<String>),
    SetMimeType(Option<String>),
}

impl FormBuilder for NewDocumentFormatBuilder {
    type Data = NewDocumentFormat;
    type Actions = NewDocumentFormatBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_extension.is_empty() || !self.errors_mime_type.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewDocumentFormatBuilder> for NewDocumentFormat {
    fn from(builder: NewDocumentFormatBuilder) -> Self {
        Self {
            extension: builder.extension.unwrap(),
            mime_type: builder.mime_type.unwrap(),
        }
    }
}
impl FormBuildable for NewDocumentFormat {
    type Builder = NewDocumentFormatBuilder;
    const TABLE: Table = Table::DocumentFormats;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewDocumentFormat"
    }
    fn task_target() -> &'static str {
        "NewDocumentFormat"
    }
    fn description() -> &'static str {
        concat!("Create a new NewDocumentFormat.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewDocumentFormatBuilder> for NewDocumentFormatBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NewDocumentFormatBuilder>,
    ) -> std::rc::Rc<NewDocumentFormatBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewDocumentFormatBuilderActions::SetExtension(extension) => {
                if extension.is_none() {
                    state_mut.errors_extension.push(ApiError::BadRequest(vec![
                        "The Extension field is required.".to_string(),
                    ]));
                }
                state_mut.extension = extension;
            }
            NewDocumentFormatBuilderActions::SetMimeType(mime_type) => {
                if mime_type.is_none() {
                    state_mut.errors_mime_type.push(ApiError::BadRequest(vec![
                        "The MimeType field is required.".to_string(),
                    ]));
                }
                state_mut.mime_type = mime_type;
            }
        }
        state
    }
}
#[function_component(NewDocumentFormatForm)]
pub fn new_document_format_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewDocumentFormatBuilder>();
    let set_extension = builder_dispatch.apply_callback(|extension: Option<String>| {
        NewDocumentFormatBuilderActions::SetExtension(extension)
    });
    let set_mime_type = builder_dispatch.apply_callback(|mime_type: Option<String>| {
        NewDocumentFormatBuilderActions::SetMimeType(mime_type)
    });
    html! {
        <BasicForm<NewDocumentFormat> builder={builder_store.deref().clone()}>
            <BasicInput label="Extension" errors={builder_store.errors_extension.clone()} builder={set_extension} value={builder_store.extension.clone()} input_type={InputType::Text} />
            <BasicInput label="MimeType" errors={builder_store.errors_mime_type.clone()} builder={set_mime_type} value={builder_store.mime_type.clone()} input_type={InputType::Text} />
        </BasicForm<NewDocumentFormat>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedDocumentBuilder {
    pub author: Option<User>,
    pub format: Option<DocumentFormat>,
    pub path: Option<String>,
    pub bytes: Option<i32>,
    pub errors_author: Vec<ApiError>,
    pub errors_format: Vec<ApiError>,
    pub errors_path: Vec<ApiError>,
    pub errors_bytes: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedDocumentBuilderActions {
    SetAuthor(Option<User>),
    SetFormat(Option<DocumentFormat>),
    SetPath(Option<String>),
    SetBytes(Option<i32>),
}

impl FormBuilder for NestedDocumentBuilder {
    type Data = Document;
    type Actions = NestedDocumentBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_author.is_empty()
            || !self.errors_format.is_empty()
            || !self.errors_path.is_empty()
            || !self.errors_bytes.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedDocumentBuilder> for Document {
    fn from(builder: NestedDocumentBuilder) -> Self {
        Self {
            id: Uuid::new_v4(),
            author_id: builder.author.unwrap().id,
            path: builder.path.unwrap(),
            format_id: builder.format.unwrap().id,
            bytes: builder.bytes.unwrap(),
        }
    }
}
impl FormBuildable for Document {
    type Builder = NestedDocumentBuilder;
    const TABLE: Table = Table::Documents;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "Document"
    }
    fn task_target() -> &'static str {
        "Document"
    }
    fn description() -> &'static str {
        concat!("Create a new Document.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedDocumentBuilder> for NestedDocumentBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedDocumentBuilder>,
    ) -> std::rc::Rc<NestedDocumentBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedDocumentBuilderActions::SetAuthor(author) => {
                if author.is_none() {
                    state_mut.errors_author.push(ApiError::BadRequest(vec![
                        "The Author field is required.".to_string(),
                    ]));
                }
                state_mut.author = author;
            }
            NestedDocumentBuilderActions::SetFormat(format) => {
                if format.is_none() {
                    state_mut.errors_format.push(ApiError::BadRequest(vec![
                        "The Format field is required.".to_string(),
                    ]));
                }
                state_mut.format = format;
            }
            NestedDocumentBuilderActions::SetPath(path) => {
                if path.is_none() {
                    state_mut.errors_path.push(ApiError::BadRequest(vec![
                        "The Path field is required.".to_string(),
                    ]));
                }
                state_mut.path = path;
            }
            NestedDocumentBuilderActions::SetBytes(bytes) => {
                if bytes.is_none() {
                    state_mut.errors_bytes.push(ApiError::BadRequest(vec![
                        "The Bytes field is required.".to_string(),
                    ]));
                }
                state_mut.bytes = bytes;
            }
        }
        state
    }
}
#[function_component(DocumentForm)]
pub fn document_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedDocumentBuilder>();
    let set_author = builder_dispatch
        .apply_callback(|author: Option<User>| NestedDocumentBuilderActions::SetAuthor(author));
    let set_format = builder_dispatch.apply_callback(|format: Option<DocumentFormat>| {
        NestedDocumentBuilderActions::SetFormat(format)
    });
    let set_path = builder_dispatch
        .apply_callback(|path: Option<String>| NestedDocumentBuilderActions::SetPath(path));
    let set_bytes = builder_dispatch
        .apply_callback(|bytes: Option<i32>| NestedDocumentBuilderActions::SetBytes(bytes));
    html! {
        <BasicForm<Document> builder={builder_store.deref().clone()}>
            <Datalist<User> builder={set_author} errors={builder_store.errors_author.clone()} value={builder_store.author.clone()} label="Author" />
            <Datalist<DocumentFormat> builder={set_format} errors={builder_store.errors_format.clone()} value={builder_store.format.clone()} label="Format" />
            <BasicInput label="Path" errors={builder_store.errors_path.clone()} builder={set_path} value={builder_store.path.clone()} input_type={InputType::Text} />
        </BasicForm<Document>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewFontAwesomeIconBuilder {
    pub name: Option<String>,
    pub errors_name: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewFontAwesomeIconBuilderActions {
    SetName(Option<String>),
}

impl FormBuilder for NewFontAwesomeIconBuilder {
    type Data = NewFontAwesomeIcon;
    type Actions = NewFontAwesomeIconBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewFontAwesomeIconBuilder> for NewFontAwesomeIcon {
    fn from(builder: NewFontAwesomeIconBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
        }
    }
}
impl FormBuildable for NewFontAwesomeIcon {
    type Builder = NewFontAwesomeIconBuilder;
    const TABLE: Table = Table::FontAwesomeIcons;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewFontAwesomeIcon"
    }
    fn task_target() -> &'static str {
        "NewFontAwesomeIcon"
    }
    fn description() -> &'static str {
        concat!("Create a new NewFontAwesomeIcon.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewFontAwesomeIconBuilder> for NewFontAwesomeIconBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NewFontAwesomeIconBuilder>,
    ) -> std::rc::Rc<NewFontAwesomeIconBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewFontAwesomeIconBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
        }
        state
    }
}
#[function_component(NewFontAwesomeIconForm)]
pub fn new_font_awesome_icon_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewFontAwesomeIconBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewFontAwesomeIconBuilderActions::SetName(name));
    html! {
        <BasicForm<NewFontAwesomeIcon> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewFontAwesomeIcon>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewItemCategoryBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewItemCategoryBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewItemCategoryBuilder {
    type Data = NewItemCategory;
    type Actions = NestedNewItemCategoryBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewItemCategoryBuilder> for NewItemCategory {
    fn from(builder: NestedNewItemCategoryBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
        }
    }
}
impl FormBuildable for NewItemCategory {
    type Builder = NestedNewItemCategoryBuilder;
    const TABLE: Table = Table::ItemCategories;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewItemCategory"
    }
    fn task_target() -> &'static str {
        "NewItemCategory"
    }
    fn description() -> &'static str {
        concat!("Create a new NewItemCategory.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewItemCategoryBuilder> for NestedNewItemCategoryBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewItemCategoryBuilder>,
    ) -> std::rc::Rc<NestedNewItemCategoryBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewItemCategoryBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewItemCategoryBuilderActions::SetDescription(description) => {
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                }
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewItemCategoryForm)]
pub fn new_item_category_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewItemCategoryBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NestedNewItemCategoryBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NestedNewItemCategoryBuilderActions::SetDescription(description)
    });
    html! {
        <BasicForm<NewItemCategory> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewItemCategory>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewItemCategoryRelationshipBuilder {
    pub parent: Option<NestedItemCategory>,
    pub child: Option<NestedItemCategory>,
    pub added_by: Option<User>,
    pub errors_parent: Vec<ApiError>,
    pub errors_child: Vec<ApiError>,
    pub errors_added_by: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewItemCategoryRelationshipBuilderActions {
    SetParent(Option<NestedItemCategory>),
    SetChild(Option<NestedItemCategory>),
    SetAddedBy(Option<User>),
}

impl FormBuilder for NestedNewItemCategoryRelationshipBuilder {
    type Data = NewItemCategoryRelationship;
    type Actions = NestedNewItemCategoryRelationshipBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_parent.is_empty()
            || !self.errors_child.is_empty()
            || !self.errors_added_by.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewItemCategoryRelationshipBuilder> for NewItemCategoryRelationship {
    fn from(builder: NestedNewItemCategoryRelationshipBuilder) -> Self {
        Self {
            parent_id: builder.parent.unwrap().inner.id,
            child_id: builder.child.unwrap().inner.id,
            added_by: builder.added_by.unwrap().id,
        }
    }
}
impl FormBuildable for NewItemCategoryRelationship {
    type Builder = NestedNewItemCategoryRelationshipBuilder;
    const TABLE: Table = Table::ItemCategoryRelationships;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewItemCategoryRelationship"
    }
    fn task_target() -> &'static str {
        "NewItemCategoryRelationship"
    }
    fn description() -> &'static str {
        concat!("Create a new NewItemCategoryRelationship.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewItemCategoryRelationshipBuilder>
    for NestedNewItemCategoryRelationshipBuilderActions
{
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewItemCategoryRelationshipBuilder>,
    ) -> std::rc::Rc<NestedNewItemCategoryRelationshipBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewItemCategoryRelationshipBuilderActions::SetParent(parent) => {
                if parent.is_none() {
                    state_mut.errors_parent.push(ApiError::BadRequest(vec![
                        "The Parent field is required.".to_string(),
                    ]));
                }
                state_mut.parent = parent;
            }
            NestedNewItemCategoryRelationshipBuilderActions::SetChild(child) => {
                if child.is_none() {
                    state_mut.errors_child.push(ApiError::BadRequest(vec![
                        "The Child field is required.".to_string(),
                    ]));
                }
                state_mut.child = child;
            }
            NestedNewItemCategoryRelationshipBuilderActions::SetAddedBy(added_by) => {
                if added_by.is_none() {
                    state_mut.errors_added_by.push(ApiError::BadRequest(vec![
                        "The AddedBy field is required.".to_string(),
                    ]));
                }
                state_mut.added_by = added_by;
            }
        }
        state
    }
}
#[function_component(NewItemCategoryRelationshipForm)]
pub fn new_item_category_relationship_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewItemCategoryRelationshipBuilder>();
    let set_parent = builder_dispatch.apply_callback(|parent: Option<NestedItemCategory>| {
        NestedNewItemCategoryRelationshipBuilderActions::SetParent(parent)
    });
    let set_child = builder_dispatch.apply_callback(|child: Option<NestedItemCategory>| {
        NestedNewItemCategoryRelationshipBuilderActions::SetChild(child)
    });
    let set_added_by = builder_dispatch.apply_callback(|added_by: Option<User>| {
        NestedNewItemCategoryRelationshipBuilderActions::SetAddedBy(added_by)
    });
    html! {
        <BasicForm<NewItemCategoryRelationship> builder={builder_store.deref().clone()}>
            <Datalist<NestedItemCategory> builder={set_parent} errors={builder_store.errors_parent.clone()} value={builder_store.parent.clone()} label="Parent" />
            <Datalist<NestedItemCategory> builder={set_child} errors={builder_store.errors_child.clone()} value={builder_store.child.clone()} label="Child" />
            <Datalist<User> builder={set_added_by} errors={builder_store.errors_added_by.clone()} value={builder_store.added_by.clone()} label="AddedBy" />
        </BasicForm<NewItemCategoryRelationship>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewItemCategoryUnitBuilder {
    pub item_category: Option<NestedItemCategory>,
    pub unit: Option<Unit>,
    pub errors_item_category: Vec<ApiError>,
    pub errors_unit: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewItemCategoryUnitBuilderActions {
    SetItemCategory(Option<NestedItemCategory>),
    SetUnit(Option<Unit>),
}

impl FormBuilder for NestedNewItemCategoryUnitBuilder {
    type Data = NewItemCategoryUnit;
    type Actions = NestedNewItemCategoryUnitBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_item_category.is_empty() || !self.errors_unit.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewItemCategoryUnitBuilder> for NewItemCategoryUnit {
    fn from(builder: NestedNewItemCategoryUnitBuilder) -> Self {
        Self {
            item_category_id: builder.item_category.unwrap().inner.id,
            unit_id: builder.unit.unwrap().id,
        }
    }
}
impl FormBuildable for NewItemCategoryUnit {
    type Builder = NestedNewItemCategoryUnitBuilder;
    const TABLE: Table = Table::ItemCategoryUnits;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewItemCategoryUnit"
    }
    fn task_target() -> &'static str {
        "NewItemCategoryUnit"
    }
    fn description() -> &'static str {
        concat!("Create a new NewItemCategoryUnit.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewItemCategoryUnitBuilder> for NestedNewItemCategoryUnitBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewItemCategoryUnitBuilder>,
    ) -> std::rc::Rc<NestedNewItemCategoryUnitBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewItemCategoryUnitBuilderActions::SetItemCategory(item_category) => {
                if item_category.is_none() {
                    state_mut
                        .errors_item_category
                        .push(ApiError::BadRequest(vec![
                            "The ItemCategory field is required.".to_string(),
                        ]));
                }
                state_mut.item_category = item_category;
            }
            NestedNewItemCategoryUnitBuilderActions::SetUnit(unit) => {
                if unit.is_none() {
                    state_mut.errors_unit.push(ApiError::BadRequest(vec![
                        "The Unit field is required.".to_string(),
                    ]));
                }
                state_mut.unit = unit;
            }
        }
        state
    }
}
#[function_component(NewItemCategoryUnitForm)]
pub fn new_item_category_unit_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewItemCategoryUnitBuilder>();
    let set_item_category =
        builder_dispatch.apply_callback(|item_category: Option<NestedItemCategory>| {
            NestedNewItemCategoryUnitBuilderActions::SetItemCategory(item_category)
        });
    let set_unit = builder_dispatch.apply_callback(|unit: Option<Unit>| {
        NestedNewItemCategoryUnitBuilderActions::SetUnit(unit)
    });
    html! {
        <BasicForm<NewItemCategoryUnit> builder={builder_store.deref().clone()}>
            <Datalist<NestedItemCategory> builder={set_item_category} errors={builder_store.errors_item_category.clone()} value={builder_store.item_category.clone()} label="ItemCategory" />
            <Datalist<Unit> builder={set_unit} errors={builder_store.errors_unit.clone()} value={builder_store.unit.clone()} label="Unit" />
        </BasicForm<NewItemCategoryUnit>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewLoginProviderBuilder {
    pub name: Option<String>,
    pub font_awesome_icon: Option<String>,
    pub client_id_var_name: Option<String>,
    pub redirect_uri_var_name: Option<String>,
    pub oauth_url: Option<String>,
    pub scope: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_font_awesome_icon: Vec<ApiError>,
    pub errors_client_id_var_name: Vec<ApiError>,
    pub errors_redirect_uri_var_name: Vec<ApiError>,
    pub errors_oauth_url: Vec<ApiError>,
    pub errors_scope: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewLoginProviderBuilderActions {
    SetName(Option<String>),
    SetFontAwesomeIcon(Option<String>),
    SetClientIdVarName(Option<String>),
    SetRedirectUriVarName(Option<String>),
    SetOauthUrl(Option<String>),
    SetScope(Option<String>),
}

impl FormBuilder for NewLoginProviderBuilder {
    type Data = NewLoginProvider;
    type Actions = NewLoginProviderBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
            || !self.errors_font_awesome_icon.is_empty()
            || !self.errors_client_id_var_name.is_empty()
            || !self.errors_redirect_uri_var_name.is_empty()
            || !self.errors_oauth_url.is_empty()
            || !self.errors_scope.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewLoginProviderBuilder> for NewLoginProvider {
    fn from(builder: NewLoginProviderBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            font_awesome_icon: builder.font_awesome_icon.unwrap(),
            client_id_var_name: builder.client_id_var_name.unwrap(),
            redirect_uri_var_name: builder.redirect_uri_var_name.unwrap(),
            oauth_url: builder.oauth_url.unwrap(),
            scope: builder.scope.unwrap(),
        }
    }
}
impl FormBuildable for NewLoginProvider {
    type Builder = NewLoginProviderBuilder;
    const TABLE: Table = Table::LoginProviders;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewLoginProvider"
    }
    fn task_target() -> &'static str {
        "NewLoginProvider"
    }
    fn description() -> &'static str {
        concat!("Create a new NewLoginProvider.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewLoginProviderBuilder> for NewLoginProviderBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NewLoginProviderBuilder>,
    ) -> std::rc::Rc<NewLoginProviderBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewLoginProviderBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NewLoginProviderBuilderActions::SetFontAwesomeIcon(font_awesome_icon) => {
                if font_awesome_icon.is_none() {
                    state_mut
                        .errors_font_awesome_icon
                        .push(ApiError::BadRequest(vec![
                            "The FontAwesomeIcon field is required.".to_string(),
                        ]));
                }
                state_mut.font_awesome_icon = font_awesome_icon;
            }
            NewLoginProviderBuilderActions::SetClientIdVarName(client_id_var_name) => {
                if client_id_var_name.is_none() {
                    state_mut
                        .errors_client_id_var_name
                        .push(ApiError::BadRequest(vec![
                            "The ClientIdVarName field is required.".to_string(),
                        ]));
                }
                state_mut.client_id_var_name = client_id_var_name;
            }
            NewLoginProviderBuilderActions::SetRedirectUriVarName(redirect_uri_var_name) => {
                if redirect_uri_var_name.is_none() {
                    state_mut
                        .errors_redirect_uri_var_name
                        .push(ApiError::BadRequest(vec![
                            "The RedirectUriVarName field is required.".to_string(),
                        ]));
                }
                state_mut.redirect_uri_var_name = redirect_uri_var_name;
            }
            NewLoginProviderBuilderActions::SetOauthUrl(oauth_url) => {
                if oauth_url.is_none() {
                    state_mut.errors_oauth_url.push(ApiError::BadRequest(vec![
                        "The OauthUrl field is required.".to_string(),
                    ]));
                }
                state_mut.oauth_url = oauth_url;
            }
            NewLoginProviderBuilderActions::SetScope(scope) => {
                if scope.is_none() {
                    state_mut.errors_scope.push(ApiError::BadRequest(vec![
                        "The Scope field is required.".to_string(),
                    ]));
                }
                state_mut.scope = scope;
            }
        }
        state
    }
}
#[function_component(NewLoginProviderForm)]
pub fn new_login_provider_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewLoginProviderBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewLoginProviderBuilderActions::SetName(name));
    let set_font_awesome_icon =
        builder_dispatch.apply_callback(|font_awesome_icon: Option<String>| {
            NewLoginProviderBuilderActions::SetFontAwesomeIcon(font_awesome_icon)
        });
    let set_client_id_var_name =
        builder_dispatch.apply_callback(|client_id_var_name: Option<String>| {
            NewLoginProviderBuilderActions::SetClientIdVarName(client_id_var_name)
        });
    let set_redirect_uri_var_name =
        builder_dispatch.apply_callback(|redirect_uri_var_name: Option<String>| {
            NewLoginProviderBuilderActions::SetRedirectUriVarName(redirect_uri_var_name)
        });
    let set_oauth_url = builder_dispatch.apply_callback(|oauth_url: Option<String>| {
        NewLoginProviderBuilderActions::SetOauthUrl(oauth_url)
    });
    let set_scope = builder_dispatch
        .apply_callback(|scope: Option<String>| NewLoginProviderBuilderActions::SetScope(scope));
    html! {
        <BasicForm<NewLoginProvider> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="FontAwesomeIcon" errors={builder_store.errors_font_awesome_icon.clone()} builder={set_font_awesome_icon} value={builder_store.font_awesome_icon.clone()} input_type={InputType::Text} />
            <BasicInput label="ClientIdVarName" errors={builder_store.errors_client_id_var_name.clone()} builder={set_client_id_var_name} value={builder_store.client_id_var_name.clone()} input_type={InputType::Text} />
            <BasicInput label="RedirectUriVarName" errors={builder_store.errors_redirect_uri_var_name.clone()} builder={set_redirect_uri_var_name} value={builder_store.redirect_uri_var_name.clone()} input_type={InputType::Text} />
            <BasicInput label="OauthUrl" errors={builder_store.errors_oauth_url.clone()} builder={set_oauth_url} value={builder_store.oauth_url.clone()} input_type={InputType::Text} />
            <BasicInput label="Scope" errors={builder_store.errors_scope.clone()} builder={set_scope} value={builder_store.scope.clone()} input_type={InputType::Text} />
        </BasicForm<NewLoginProvider>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewManufacturedItemCategoryBuilder {
    pub manifacturer: Option<NestedOrganization>,
    pub cost: Option<i32>,
    pub cost_per_day: Option<i32>,
    pub currency: Option<String>,
    pub errors_manifacturer: Vec<ApiError>,
    pub errors_cost: Vec<ApiError>,
    pub errors_cost_per_day: Vec<ApiError>,
    pub errors_currency: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewManufacturedItemCategoryBuilderActions {
    SetManifacturer(Option<NestedOrganization>),
    SetCost(Option<i32>),
    SetCostPerDay(Option<i32>),
    SetCurrency(Option<String>),
}

impl FormBuilder for NestedNewManufacturedItemCategoryBuilder {
    type Data = NewManufacturedItemCategory;
    type Actions = NestedNewManufacturedItemCategoryBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_manifacturer.is_empty()
            || !self.errors_cost.is_empty()
            || !self.errors_cost_per_day.is_empty()
            || !self.errors_currency.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewManufacturedItemCategoryBuilder> for NewManufacturedItemCategory {
    fn from(builder: NestedNewManufacturedItemCategoryBuilder) -> Self {
        Self {
            cost: builder.cost.unwrap(),
            cost_per_day: builder.cost_per_day.unwrap(),
            currency: builder.currency.unwrap(),
            manifacturer_id: builder.manifacturer.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewManufacturedItemCategory {
    type Builder = NestedNewManufacturedItemCategoryBuilder;
    const TABLE: Table = Table::ManufacturedItemCategories;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewManufacturedItemCategory"
    }
    fn task_target() -> &'static str {
        "NewManufacturedItemCategory"
    }
    fn description() -> &'static str {
        concat!("Create a new NewManufacturedItemCategory.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewManufacturedItemCategoryBuilder>
    for NestedNewManufacturedItemCategoryBuilderActions
{
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewManufacturedItemCategoryBuilder>,
    ) -> std::rc::Rc<NestedNewManufacturedItemCategoryBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewManufacturedItemCategoryBuilderActions::SetManifacturer(manifacturer) => {
                if manifacturer.is_none() {
                    state_mut
                        .errors_manifacturer
                        .push(ApiError::BadRequest(vec![
                            "The Manifacturer field is required.".to_string(),
                        ]));
                }
                state_mut.manifacturer = manifacturer;
            }
            NestedNewManufacturedItemCategoryBuilderActions::SetCost(cost) => {
                if cost.is_none() {
                    state_mut.errors_cost.push(ApiError::BadRequest(vec![
                        "The Cost field is required.".to_string(),
                    ]));
                }
                state_mut.cost = cost;
            }
            NestedNewManufacturedItemCategoryBuilderActions::SetCostPerDay(cost_per_day) => {
                if cost_per_day.is_none() {
                    state_mut
                        .errors_cost_per_day
                        .push(ApiError::BadRequest(vec![
                            "The CostPerDay field is required.".to_string(),
                        ]));
                }
                state_mut.cost_per_day = cost_per_day;
            }
            NestedNewManufacturedItemCategoryBuilderActions::SetCurrency(currency) => {
                if currency.is_none() {
                    state_mut.errors_currency.push(ApiError::BadRequest(vec![
                        "The Currency field is required.".to_string(),
                    ]));
                }
                state_mut.currency = currency;
            }
        }
        state
    }
}
#[function_component(NewManufacturedItemCategoryForm)]
pub fn new_manufactured_item_category_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewManufacturedItemCategoryBuilder>();
    let set_manifacturer =
        builder_dispatch.apply_callback(|manifacturer: Option<NestedOrganization>| {
            NestedNewManufacturedItemCategoryBuilderActions::SetManifacturer(manifacturer)
        });
    let set_cost = builder_dispatch.apply_callback(|cost: Option<i32>| {
        NestedNewManufacturedItemCategoryBuilderActions::SetCost(cost)
    });
    let set_cost_per_day = builder_dispatch.apply_callback(|cost_per_day: Option<i32>| {
        NestedNewManufacturedItemCategoryBuilderActions::SetCostPerDay(cost_per_day)
    });
    let set_currency = builder_dispatch.apply_callback(|currency: Option<String>| {
        NestedNewManufacturedItemCategoryBuilderActions::SetCurrency(currency)
    });
    html! {
        <BasicForm<NewManufacturedItemCategory> builder={builder_store.deref().clone()}>
            <Datalist<NestedOrganization> builder={set_manifacturer} errors={builder_store.errors_manifacturer.clone()} value={builder_store.manifacturer.clone()} label="Manifacturer" />
            <BasicInput label="Currency" errors={builder_store.errors_currency.clone()} builder={set_currency} value={builder_store.currency.clone()} input_type={InputType::Text} />
        </BasicForm<NewManufacturedItemCategory>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewNotificationBuilder {
    pub user: Option<User>,
    pub operation: Option<String>,
    pub table_name: Option<String>,
    pub read: Option<bool>,
    pub errors_user: Vec<ApiError>,
    pub errors_operation: Vec<ApiError>,
    pub errors_table_name: Vec<ApiError>,
    pub errors_read: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewNotificationBuilderActions {
    SetUser(Option<User>),
    SetOperation(Option<String>),
    SetTableName(Option<String>),
    SetRead(Option<bool>),
}

impl FormBuilder for NestedNewNotificationBuilder {
    type Data = NewNotification;
    type Actions = NestedNewNotificationBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_user.is_empty()
            || !self.errors_operation.is_empty()
            || !self.errors_table_name.is_empty()
            || !self.errors_read.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewNotificationBuilder> for NewNotification {
    fn from(builder: NestedNewNotificationBuilder) -> Self {
        Self {
            user_id: builder.user.unwrap().id,
            operation: builder.operation.unwrap(),
            table_name: builder.table_name.unwrap(),
            read: builder.read.unwrap(),
        }
    }
}
impl FormBuildable for NewNotification {
    type Builder = NestedNewNotificationBuilder;
    const TABLE: Table = Table::Notifications;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewNotification"
    }
    fn task_target() -> &'static str {
        "NewNotification"
    }
    fn description() -> &'static str {
        concat!("Create a new NewNotification.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewNotificationBuilder> for NestedNewNotificationBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewNotificationBuilder>,
    ) -> std::rc::Rc<NestedNewNotificationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewNotificationBuilderActions::SetUser(user) => {
                if user.is_none() {
                    state_mut.errors_user.push(ApiError::BadRequest(vec![
                        "The User field is required.".to_string(),
                    ]));
                }
                state_mut.user = user;
            }
            NestedNewNotificationBuilderActions::SetOperation(operation) => {
                if operation.is_none() {
                    state_mut.errors_operation.push(ApiError::BadRequest(vec![
                        "The Operation field is required.".to_string(),
                    ]));
                }
                state_mut.operation = operation;
            }
            NestedNewNotificationBuilderActions::SetTableName(table_name) => {
                if table_name.is_none() {
                    state_mut.errors_table_name.push(ApiError::BadRequest(vec![
                        "The TableName field is required.".to_string(),
                    ]));
                }
                state_mut.table_name = table_name;
            }
            NestedNewNotificationBuilderActions::SetRead(read) => {
                if read.is_none() {
                    state_mut.errors_read.push(ApiError::BadRequest(vec![
                        "The Read field is required.".to_string(),
                    ]));
                }
                state_mut.read = read;
            }
        }
        state
    }
}
#[function_component(NewNotificationForm)]
pub fn new_notification_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewNotificationBuilder>();
    let set_user = builder_dispatch
        .apply_callback(|user: Option<User>| NestedNewNotificationBuilderActions::SetUser(user));
    let set_operation = builder_dispatch.apply_callback(|operation: Option<String>| {
        NestedNewNotificationBuilderActions::SetOperation(operation)
    });
    let set_table_name = builder_dispatch.apply_callback(|table_name: Option<String>| {
        NestedNewNotificationBuilderActions::SetTableName(table_name)
    });
    let set_read = builder_dispatch
        .apply_callback(|read: bool| NestedNewNotificationBuilderActions::SetRead(Some(read)));
    html! {
        <BasicForm<NewNotification> builder={builder_store.deref().clone()}>
            <Datalist<User> builder={set_user} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" />
            <BasicInput label="Operation" errors={builder_store.errors_operation.clone()} builder={set_operation} value={builder_store.operation.clone()} input_type={InputType::Text} />
            <BasicInput label="TableName" errors={builder_store.errors_table_name.clone()} builder={set_table_name} value={builder_store.table_name.clone()} input_type={InputType::Text} />
            <Checkbox label="Read" errors={builder_store.errors_read.clone()} builder={set_read} value={builder_store.read.unwrap_or(false)} />
        </BasicForm<NewNotification>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewOrganizationBuilder {
    pub parent_organization: Option<NestedOrganization>,
    pub name: Option<String>,
    pub errors_parent_organization: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewOrganizationBuilderActions {
    SetParentOrganization(Option<NestedOrganization>),
    SetName(Option<String>),
}

impl FormBuilder for NestedNewOrganizationBuilder {
    type Data = NewOrganization;
    type Actions = NestedNewOrganizationBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_parent_organization.is_empty() || !self.errors_name.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewOrganizationBuilder> for NewOrganization {
    fn from(builder: NestedNewOrganizationBuilder) -> Self {
        Self {
            parent_organization_id: builder
                .parent_organization
                .map(|parent_organization| parent_organization.inner.id),
            name: builder.name.unwrap(),
        }
    }
}
impl FormBuildable for NewOrganization {
    type Builder = NestedNewOrganizationBuilder;
    const TABLE: Table = Table::Organizations;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewOrganization"
    }
    fn task_target() -> &'static str {
        "NewOrganization"
    }
    fn description() -> &'static str {
        concat!("Create a new NewOrganization.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewOrganizationBuilder> for NestedNewOrganizationBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewOrganizationBuilder>,
    ) -> std::rc::Rc<NestedNewOrganizationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewOrganizationBuilderActions::SetParentOrganization(parent_organization) => {
                state_mut.parent_organization = parent_organization;
            }
            NestedNewOrganizationBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
        }
        state
    }
}
#[function_component(NewOrganizationForm)]
pub fn new_organization_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewOrganizationBuilder>();
    let set_parent_organization =
        builder_dispatch.apply_callback(|parent_organization: Option<NestedOrganization>| {
            NestedNewOrganizationBuilderActions::SetParentOrganization(parent_organization)
        });
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NestedNewOrganizationBuilderActions::SetName(name));
    html! {
        <BasicForm<NewOrganization> builder={builder_store.deref().clone()}>
            <Datalist<NestedOrganization> builder={set_parent_organization} errors={builder_store.errors_parent_organization.clone()} value={builder_store.parent_organization.clone()} label="ParentOrganization" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewOrganization>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewProcedureBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewProcedureBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewProcedureBuilder {
    type Data = NewProcedure;
    type Actions = NestedNewProcedureBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewProcedureBuilder> for NewProcedure {
    fn from(builder: NestedNewProcedureBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description,
        }
    }
}
impl FormBuildable for NewProcedure {
    type Builder = NestedNewProcedureBuilder;
    const TABLE: Table = Table::Procedures;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewProcedure"
    }
    fn task_target() -> &'static str {
        "NewProcedure"
    }
    fn description() -> &'static str {
        concat!("Create a new NewProcedure.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewProcedureBuilder> for NestedNewProcedureBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewProcedureBuilder>,
    ) -> std::rc::Rc<NestedNewProcedureBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewProcedureBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewProcedureBuilderActions::SetDescription(description) => {
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewProcedureForm)]
pub fn new_procedure_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewProcedureBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NestedNewProcedureBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NestedNewProcedureBuilderActions::SetDescription(description)
    });
    html! {
        <BasicForm<NewProcedure> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewProcedure>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewProjectRequirementBuilder {
    pub project: Option<NestedProject>,
    pub item_category: Option<NestedItemCategory>,
    pub unit: Option<Unit>,
    pub quantity: Option<i32>,
    pub errors_project: Vec<ApiError>,
    pub errors_item_category: Vec<ApiError>,
    pub errors_unit: Vec<ApiError>,
    pub errors_quantity: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewProjectRequirementBuilderActions {
    SetProject(Option<NestedProject>),
    SetItemCategory(Option<NestedItemCategory>),
    SetUnit(Option<Unit>),
    SetQuantity(Option<i32>),
}

impl FormBuilder for NestedNewProjectRequirementBuilder {
    type Data = NewProjectRequirement;
    type Actions = NestedNewProjectRequirementBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_project.is_empty()
            || !self.errors_item_category.is_empty()
            || !self.errors_unit.is_empty()
            || !self.errors_quantity.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewProjectRequirementBuilder> for NewProjectRequirement {
    fn from(builder: NestedNewProjectRequirementBuilder) -> Self {
        Self {
            project_id: builder.project.unwrap().inner.id,
            item_category_id: builder.item_category.unwrap().inner.id,
            quantity: builder.quantity.unwrap(),
            unit_id: builder.unit.map(|unit| unit.id),
        }
    }
}
impl FormBuildable for NewProjectRequirement {
    type Builder = NestedNewProjectRequirementBuilder;
    const TABLE: Table = Table::ProjectRequirements;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewProjectRequirement"
    }
    fn task_target() -> &'static str {
        "NewProjectRequirement"
    }
    fn description() -> &'static str {
        concat!("Create a new NewProjectRequirement.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewProjectRequirementBuilder> for NestedNewProjectRequirementBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewProjectRequirementBuilder>,
    ) -> std::rc::Rc<NestedNewProjectRequirementBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewProjectRequirementBuilderActions::SetProject(project) => {
                if project.is_none() {
                    state_mut.errors_project.push(ApiError::BadRequest(vec![
                        "The Project field is required.".to_string(),
                    ]));
                }
                state_mut.project = project;
            }
            NestedNewProjectRequirementBuilderActions::SetItemCategory(item_category) => {
                if item_category.is_none() {
                    state_mut
                        .errors_item_category
                        .push(ApiError::BadRequest(vec![
                            "The ItemCategory field is required.".to_string(),
                        ]));
                }
                state_mut.item_category = item_category;
            }
            NestedNewProjectRequirementBuilderActions::SetUnit(unit) => {
                state_mut.unit = unit;
            }
            NestedNewProjectRequirementBuilderActions::SetQuantity(quantity) => {
                if quantity.is_none() {
                    state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                        "The Quantity field is required.".to_string(),
                    ]));
                }
                state_mut.quantity = quantity;
            }
        }
        state
    }
}
#[function_component(NewProjectRequirementForm)]
pub fn new_project_requirement_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewProjectRequirementBuilder>();
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| {
        NestedNewProjectRequirementBuilderActions::SetProject(project)
    });
    let set_item_category =
        builder_dispatch.apply_callback(|item_category: Option<NestedItemCategory>| {
            NestedNewProjectRequirementBuilderActions::SetItemCategory(item_category)
        });
    let set_unit = builder_dispatch.apply_callback(|unit: Option<Unit>| {
        NestedNewProjectRequirementBuilderActions::SetUnit(unit)
    });
    let set_quantity = builder_dispatch.apply_callback(|quantity: Option<i32>| {
        NestedNewProjectRequirementBuilderActions::SetQuantity(quantity)
    });
    html! {
        <BasicForm<NewProjectRequirement> builder={builder_store.deref().clone()}>
            <Datalist<NestedProject> builder={set_project} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
            <Datalist<NestedItemCategory> builder={set_item_category} errors={builder_store.errors_item_category.clone()} value={builder_store.item_category.clone()} label="ItemCategory" />
            <Datalist<Unit> builder={set_unit} errors={builder_store.errors_unit.clone()} value={builder_store.unit.clone()} label="Unit" />
        </BasicForm<NewProjectRequirement>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewProjectStateBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub font_awesome_icon: Option<String>,
    pub icon_color: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_font_awesome_icon: Vec<ApiError>,
    pub errors_icon_color: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewProjectStateBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetFontAwesomeIcon(Option<String>),
    SetIconColor(Option<String>),
}

impl FormBuilder for NewProjectStateBuilder {
    type Data = NewProjectState;
    type Actions = NewProjectStateBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_font_awesome_icon.is_empty()
            || !self.errors_icon_color.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewProjectStateBuilder> for NewProjectState {
    fn from(builder: NewProjectStateBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            font_awesome_icon: builder.font_awesome_icon.unwrap(),
            icon_color: builder.icon_color.unwrap(),
        }
    }
}
impl FormBuildable for NewProjectState {
    type Builder = NewProjectStateBuilder;
    const TABLE: Table = Table::ProjectStates;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewProjectState"
    }
    fn task_target() -> &'static str {
        "NewProjectState"
    }
    fn description() -> &'static str {
        concat!("Create a new NewProjectState.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewProjectStateBuilder> for NewProjectStateBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NewProjectStateBuilder>,
    ) -> std::rc::Rc<NewProjectStateBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewProjectStateBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NewProjectStateBuilderActions::SetDescription(description) => {
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                }
                state_mut.description = description;
            }
            NewProjectStateBuilderActions::SetFontAwesomeIcon(font_awesome_icon) => {
                if font_awesome_icon.is_none() {
                    state_mut
                        .errors_font_awesome_icon
                        .push(ApiError::BadRequest(vec![
                            "The FontAwesomeIcon field is required.".to_string(),
                        ]));
                }
                state_mut.font_awesome_icon = font_awesome_icon;
            }
            NewProjectStateBuilderActions::SetIconColor(icon_color) => {
                if icon_color.is_none() {
                    state_mut.errors_icon_color.push(ApiError::BadRequest(vec![
                        "The IconColor field is required.".to_string(),
                    ]));
                }
                state_mut.icon_color = icon_color;
            }
        }
        state
    }
}
#[function_component(NewProjectStateForm)]
pub fn new_project_state_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewProjectStateBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewProjectStateBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NewProjectStateBuilderActions::SetDescription(description)
    });
    let set_font_awesome_icon =
        builder_dispatch.apply_callback(|font_awesome_icon: Option<String>| {
            NewProjectStateBuilderActions::SetFontAwesomeIcon(font_awesome_icon)
        });
    let set_icon_color = builder_dispatch.apply_callback(|icon_color: Option<String>| {
        NewProjectStateBuilderActions::SetIconColor(icon_color)
    });
    html! {
        <BasicForm<NewProjectState> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
            <BasicInput label="FontAwesomeIcon" errors={builder_store.errors_font_awesome_icon.clone()} builder={set_font_awesome_icon} value={builder_store.font_awesome_icon.clone()} input_type={InputType::Text} />
            <BasicInput label="IconColor" errors={builder_store.errors_icon_color.clone()} builder={set_icon_color} value={builder_store.icon_color.clone()} input_type={InputType::Text} />
        </BasicForm<NewProjectState>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewProjectBuilder {
    pub state: Option<ProjectState>,
    pub parent_project: Option<NestedProject>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub public: Option<bool>,
    pub budget: Option<i64>,
    pub expenses: Option<i64>,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub errors_state: Vec<ApiError>,
    pub errors_parent_project: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_public: Vec<ApiError>,
    pub errors_budget: Vec<ApiError>,
    pub errors_expenses: Vec<ApiError>,
    pub errors_expected_end_date: Vec<ApiError>,
    pub errors_end_date: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewProjectBuilderActions {
    SetState(Option<ProjectState>),
    SetParentProject(Option<NestedProject>),
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetPublic(Option<bool>),
    SetBudget(Option<i64>),
    SetExpenses(Option<i64>),
    SetExpectedEndDate(Option<NaiveDateTime>),
    SetEndDate(Option<NaiveDateTime>),
}

impl FormBuilder for NestedNewProjectBuilder {
    type Data = NewProject;
    type Actions = NestedNewProjectBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_state.is_empty()
            || !self.errors_parent_project.is_empty()
            || !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_public.is_empty()
            || !self.errors_budget.is_empty()
            || !self.errors_expenses.is_empty()
            || !self.errors_expected_end_date.is_empty()
            || !self.errors_end_date.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewProjectBuilder> for NewProject {
    fn from(builder: NestedNewProjectBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            public: builder.public.unwrap(),
            state_id: builder.state.unwrap().id,
            parent_project_id: builder
                .parent_project
                .map(|parent_project| parent_project.inner.id),
            budget: builder.budget,
            expenses: builder.expenses,
            expected_end_date: builder.expected_end_date,
            end_date: builder.end_date,
        }
    }
}
impl FormBuildable for NewProject {
    type Builder = NestedNewProjectBuilder;
    const TABLE: Table = Table::Projects;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewProject"
    }
    fn task_target() -> &'static str {
        "NewProject"
    }
    fn description() -> &'static str {
        concat!("Create a new NewProject.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewProjectBuilder> for NestedNewProjectBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewProjectBuilder>,
    ) -> std::rc::Rc<NestedNewProjectBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewProjectBuilderActions::SetState(state) => {
                if state.is_none() {
                    state_mut.errors_state.push(ApiError::BadRequest(vec![
                        "The State field is required.".to_string(),
                    ]));
                }
                state_mut.state = state;
            }
            NestedNewProjectBuilderActions::SetParentProject(parent_project) => {
                state_mut.parent_project = parent_project;
            }
            NestedNewProjectBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewProjectBuilderActions::SetDescription(description) => {
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                }
                state_mut.description = description;
            }
            NestedNewProjectBuilderActions::SetPublic(public) => {
                if public.is_none() {
                    state_mut.errors_public.push(ApiError::BadRequest(vec![
                        "The Public field is required.".to_string(),
                    ]));
                }
                state_mut.public = public;
            }
            NestedNewProjectBuilderActions::SetBudget(budget) => {
                state_mut.budget = budget;
            }
            NestedNewProjectBuilderActions::SetExpenses(expenses) => {
                state_mut.expenses = expenses;
            }
            NestedNewProjectBuilderActions::SetExpectedEndDate(expected_end_date) => {
                state_mut.expected_end_date = expected_end_date;
            }
            NestedNewProjectBuilderActions::SetEndDate(end_date) => {
                state_mut.end_date = end_date;
            }
        }
        state
    }
}
#[function_component(NewProjectForm)]
pub fn new_project_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewProjectBuilder>();
    let set_state = builder_dispatch.apply_callback(|state: Option<ProjectState>| {
        NestedNewProjectBuilderActions::SetState(state)
    });
    let set_parent_project =
        builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| {
            NestedNewProjectBuilderActions::SetParentProject(parent_project)
        });
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NestedNewProjectBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NestedNewProjectBuilderActions::SetDescription(description)
    });
    let set_public = builder_dispatch
        .apply_callback(|public: bool| NestedNewProjectBuilderActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch
        .apply_callback(|budget: Option<i64>| NestedNewProjectBuilderActions::SetBudget(budget));
    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<i64>| {
        NestedNewProjectBuilderActions::SetExpenses(expenses)
    });
    let set_expected_end_date =
        builder_dispatch.apply_callback(|expected_end_date: Option<NaiveDateTime>| {
            NestedNewProjectBuilderActions::SetExpectedEndDate(expected_end_date)
        });
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<NaiveDateTime>| {
        NestedNewProjectBuilderActions::SetEndDate(end_date)
    });
    html! {
        <BasicForm<NewProject> builder={builder_store.deref().clone()}>
            <Datalist<ProjectState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<NestedProject> builder={set_parent_project} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="ParentProject" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
        </BasicForm<NewProject>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewRoleBuilder {
    pub name: Option<String>,
    pub errors_name: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewRoleBuilderActions {
    SetName(Option<String>),
}

impl FormBuilder for NewRoleBuilder {
    type Data = NewRole;
    type Actions = NewRoleBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewRoleBuilder> for NewRole {
    fn from(builder: NewRoleBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
        }
    }
}
impl FormBuildable for NewRole {
    type Builder = NewRoleBuilder;
    const TABLE: Table = Table::Roles;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewRole"
    }
    fn task_target() -> &'static str {
        "NewRole"
    }
    fn description() -> &'static str {
        concat!("Create a new NewRole.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewRoleBuilder> for NewRoleBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NewRoleBuilder>) -> std::rc::Rc<NewRoleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewRoleBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
        }
        state
    }
}
#[function_component(NewRoleForm)]
pub fn new_role_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewRoleBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewRoleBuilderActions::SetName(name));
    html! {
        <BasicForm<NewRole> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewRole>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewSampleStateBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub font_awesome_icon: Option<String>,
    pub icon_color: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_font_awesome_icon: Vec<ApiError>,
    pub errors_icon_color: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewSampleStateBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetFontAwesomeIcon(Option<String>),
    SetIconColor(Option<String>),
}

impl FormBuilder for NewSampleStateBuilder {
    type Data = NewSampleState;
    type Actions = NewSampleStateBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_font_awesome_icon.is_empty()
            || !self.errors_icon_color.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewSampleStateBuilder> for NewSampleState {
    fn from(builder: NewSampleStateBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            font_awesome_icon: builder.font_awesome_icon.unwrap(),
            icon_color: builder.icon_color.unwrap(),
        }
    }
}
impl FormBuildable for NewSampleState {
    type Builder = NewSampleStateBuilder;
    const TABLE: Table = Table::SampleStates;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewSampleState"
    }
    fn task_target() -> &'static str {
        "NewSampleState"
    }
    fn description() -> &'static str {
        concat!("Create a new NewSampleState.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewSampleStateBuilder> for NewSampleStateBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NewSampleStateBuilder>,
    ) -> std::rc::Rc<NewSampleStateBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewSampleStateBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NewSampleStateBuilderActions::SetDescription(description) => {
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                }
                state_mut.description = description;
            }
            NewSampleStateBuilderActions::SetFontAwesomeIcon(font_awesome_icon) => {
                if font_awesome_icon.is_none() {
                    state_mut
                        .errors_font_awesome_icon
                        .push(ApiError::BadRequest(vec![
                            "The FontAwesomeIcon field is required.".to_string(),
                        ]));
                }
                state_mut.font_awesome_icon = font_awesome_icon;
            }
            NewSampleStateBuilderActions::SetIconColor(icon_color) => {
                if icon_color.is_none() {
                    state_mut.errors_icon_color.push(ApiError::BadRequest(vec![
                        "The IconColor field is required.".to_string(),
                    ]));
                }
                state_mut.icon_color = icon_color;
            }
        }
        state
    }
}
#[function_component(NewSampleStateForm)]
pub fn new_sample_state_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewSampleStateBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewSampleStateBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NewSampleStateBuilderActions::SetDescription(description)
    });
    let set_font_awesome_icon =
        builder_dispatch.apply_callback(|font_awesome_icon: Option<String>| {
            NewSampleStateBuilderActions::SetFontAwesomeIcon(font_awesome_icon)
        });
    let set_icon_color = builder_dispatch.apply_callback(|icon_color: Option<String>| {
        NewSampleStateBuilderActions::SetIconColor(icon_color)
    });
    html! {
        <BasicForm<NewSampleState> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
            <BasicInput label="FontAwesomeIcon" errors={builder_store.errors_font_awesome_icon.clone()} builder={set_font_awesome_icon} value={builder_store.font_awesome_icon.clone()} input_type={InputType::Text} />
            <BasicInput label="IconColor" errors={builder_store.errors_icon_color.clone()} builder={set_icon_color} value={builder_store.icon_color.clone()} input_type={InputType::Text} />
        </BasicForm<NewSampleState>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewSampleBuilder {
    pub sampled_by: Option<User>,
    pub procedure: Option<NestedSamplingProcedure>,
    pub state: Option<SampleState>,
    pub errors_sampled_by: Vec<ApiError>,
    pub errors_procedure: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewSampleBuilderActions {
    SetSampledBy(Option<User>),
    SetProcedure(Option<NestedSamplingProcedure>),
    SetState(Option<SampleState>),
}

impl FormBuilder for NestedNewSampleBuilder {
    type Data = NewSample;
    type Actions = NestedNewSampleBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_sampled_by.is_empty()
            || !self.errors_procedure.is_empty()
            || !self.errors_state.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewSampleBuilder> for NewSample {
    fn from(builder: NestedNewSampleBuilder) -> Self {
        Self {
            sampled_by: builder.sampled_by.unwrap().id,
            procedure_id: builder.procedure.unwrap().inner.id,
            state: builder.state.unwrap().id,
        }
    }
}
impl FormBuildable for NewSample {
    type Builder = NestedNewSampleBuilder;
    const TABLE: Table = Table::Samples;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewSample"
    }
    fn task_target() -> &'static str {
        "NewSample"
    }
    fn description() -> &'static str {
        concat!("Create a new NewSample.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewSampleBuilder> for NestedNewSampleBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewSampleBuilder>,
    ) -> std::rc::Rc<NestedNewSampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewSampleBuilderActions::SetSampledBy(sampled_by) => {
                if sampled_by.is_none() {
                    state_mut.errors_sampled_by.push(ApiError::BadRequest(vec![
                        "The SampledBy field is required.".to_string(),
                    ]));
                }
                state_mut.sampled_by = sampled_by;
            }
            NestedNewSampleBuilderActions::SetProcedure(procedure) => {
                if procedure.is_none() {
                    state_mut.errors_procedure.push(ApiError::BadRequest(vec![
                        "The Procedure field is required.".to_string(),
                    ]));
                }
                state_mut.procedure = procedure;
            }
            NestedNewSampleBuilderActions::SetState(state) => {
                if state.is_none() {
                    state_mut.errors_state.push(ApiError::BadRequest(vec![
                        "The State field is required.".to_string(),
                    ]));
                }
                state_mut.state = state;
            }
        }
        state
    }
}
#[function_component(NewSampleForm)]
pub fn new_sample_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewSampleBuilder>();
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| {
        NestedNewSampleBuilderActions::SetSampledBy(sampled_by)
    });
    let set_procedure =
        builder_dispatch.apply_callback(|procedure: Option<NestedSamplingProcedure>| {
            NestedNewSampleBuilderActions::SetProcedure(procedure)
        });
    let set_state = builder_dispatch.apply_callback(|state: Option<SampleState>| {
        NestedNewSampleBuilderActions::SetState(state)
    });
    html! {
        <BasicForm<NewSample> builder={builder_store.deref().clone()}>
            <Datalist<User> builder={set_sampled_by} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="SampledBy" />
            <Datalist<NestedSamplingProcedure> builder={set_procedure} errors={builder_store.errors_procedure.clone()} value={builder_store.procedure.clone()} label="Procedure" />
            <Datalist<SampleState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
        </BasicForm<NewSample>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewSamplingProcedureBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewSamplingProcedureBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewSamplingProcedureBuilder {
    type Data = NewSamplingProcedure;
    type Actions = NestedNewSamplingProcedureBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewSamplingProcedureBuilder> for NewSamplingProcedure {
    fn from(builder: NestedNewSamplingProcedureBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description,
        }
    }
}
impl FormBuildable for NewSamplingProcedure {
    type Builder = NestedNewSamplingProcedureBuilder;
    const TABLE: Table = Table::SamplingProcedures;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewSamplingProcedure"
    }
    fn task_target() -> &'static str {
        "NewSamplingProcedure"
    }
    fn description() -> &'static str {
        concat!("Create a new NewSamplingProcedure.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewSamplingProcedureBuilder> for NestedNewSamplingProcedureBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewSamplingProcedureBuilder>,
    ) -> std::rc::Rc<NestedNewSamplingProcedureBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewSamplingProcedureBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewSamplingProcedureBuilderActions::SetDescription(description) => {
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewSamplingProcedureForm)]
pub fn new_sampling_procedure_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewSamplingProcedureBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| {
        NestedNewSamplingProcedureBuilderActions::SetName(name)
    });
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NestedNewSamplingProcedureBuilderActions::SetDescription(description)
    });
    html! {
        <BasicForm<NewSamplingProcedure> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewSamplingProcedure>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewTeamStateBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub font_awesome_icon: Option<String>,
    pub icon_color: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_font_awesome_icon: Vec<ApiError>,
    pub errors_icon_color: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewTeamStateBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetFontAwesomeIcon(Option<String>),
    SetIconColor(Option<String>),
}

impl FormBuilder for NewTeamStateBuilder {
    type Data = NewTeamState;
    type Actions = NewTeamStateBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_font_awesome_icon.is_empty()
            || !self.errors_icon_color.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewTeamStateBuilder> for NewTeamState {
    fn from(builder: NewTeamStateBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            font_awesome_icon: builder.font_awesome_icon.unwrap(),
            icon_color: builder.icon_color.unwrap(),
        }
    }
}
impl FormBuildable for NewTeamState {
    type Builder = NewTeamStateBuilder;
    const TABLE: Table = Table::TeamStates;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewTeamState"
    }
    fn task_target() -> &'static str {
        "NewTeamState"
    }
    fn description() -> &'static str {
        concat!("Create a new NewTeamState.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewTeamStateBuilder> for NewTeamStateBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NewTeamStateBuilder>,
    ) -> std::rc::Rc<NewTeamStateBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewTeamStateBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NewTeamStateBuilderActions::SetDescription(description) => {
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                }
                state_mut.description = description;
            }
            NewTeamStateBuilderActions::SetFontAwesomeIcon(font_awesome_icon) => {
                if font_awesome_icon.is_none() {
                    state_mut
                        .errors_font_awesome_icon
                        .push(ApiError::BadRequest(vec![
                            "The FontAwesomeIcon field is required.".to_string(),
                        ]));
                }
                state_mut.font_awesome_icon = font_awesome_icon;
            }
            NewTeamStateBuilderActions::SetIconColor(icon_color) => {
                if icon_color.is_none() {
                    state_mut.errors_icon_color.push(ApiError::BadRequest(vec![
                        "The IconColor field is required.".to_string(),
                    ]));
                }
                state_mut.icon_color = icon_color;
            }
        }
        state
    }
}
#[function_component(NewTeamStateForm)]
pub fn new_team_state_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewTeamStateBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewTeamStateBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NewTeamStateBuilderActions::SetDescription(description)
    });
    let set_font_awesome_icon =
        builder_dispatch.apply_callback(|font_awesome_icon: Option<String>| {
            NewTeamStateBuilderActions::SetFontAwesomeIcon(font_awesome_icon)
        });
    let set_icon_color = builder_dispatch.apply_callback(|icon_color: Option<String>| {
        NewTeamStateBuilderActions::SetIconColor(icon_color)
    });
    html! {
        <BasicForm<NewTeamState> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
            <BasicInput label="FontAwesomeIcon" errors={builder_store.errors_font_awesome_icon.clone()} builder={set_font_awesome_icon} value={builder_store.font_awesome_icon.clone()} input_type={InputType::Text} />
            <BasicInput label="IconColor" errors={builder_store.errors_icon_color.clone()} builder={set_icon_color} value={builder_store.icon_color.clone()} input_type={InputType::Text} />
        </BasicForm<NewTeamState>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewTeamBuilder {
    pub parent_team: Option<NestedTeam>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_parent_team: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewTeamBuilderActions {
    SetParentTeam(Option<NestedTeam>),
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewTeamBuilder {
    type Data = NewTeam;
    type Actions = NestedNewTeamBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_parent_team.is_empty()
            || !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NestedNewTeamBuilder> for NewTeam {
    fn from(builder: NestedNewTeamBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            parent_team_id: builder.parent_team.map(|parent_team| parent_team.inner.id),
        }
    }
}
impl FormBuildable for NewTeam {
    type Builder = NestedNewTeamBuilder;
    const TABLE: Table = Table::Teams;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewTeam"
    }
    fn task_target() -> &'static str {
        "NewTeam"
    }
    fn description() -> &'static str {
        concat!("Create a new NewTeam.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NestedNewTeamBuilder> for NestedNewTeamBuilderActions {
    fn apply(
        self,
        mut state: std::rc::Rc<NestedNewTeamBuilder>,
    ) -> std::rc::Rc<NestedNewTeamBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewTeamBuilderActions::SetParentTeam(parent_team) => {
                state_mut.parent_team = parent_team;
            }
            NestedNewTeamBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NestedNewTeamBuilderActions::SetDescription(description) => {
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                }
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewTeamForm)]
pub fn new_team_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewTeamBuilder>();
    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| {
        NestedNewTeamBuilderActions::SetParentTeam(parent_team)
    });
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NestedNewTeamBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NestedNewTeamBuilderActions::SetDescription(description)
    });
    html! {
        <BasicForm<NewTeam> builder={builder_store.deref().clone()}>
            <Datalist<NestedTeam> builder={set_parent_team} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="ParentTeam" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewTeam>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewUnitBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub symbol: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_symbol: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewUnitBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetSymbol(Option<String>),
}

impl FormBuilder for NewUnitBuilder {
    type Data = NewUnit;
    type Actions = NewUnitBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_symbol.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewUnitBuilder> for NewUnit {
    fn from(builder: NewUnitBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            symbol: builder.symbol.unwrap(),
        }
    }
}
impl FormBuildable for NewUnit {
    type Builder = NewUnitBuilder;
    const TABLE: Table = Table::Units;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewUnit"
    }
    fn task_target() -> &'static str {
        "NewUnit"
    }
    fn description() -> &'static str {
        concat!("Create a new NewUnit.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewUnitBuilder> for NewUnitBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NewUnitBuilder>) -> std::rc::Rc<NewUnitBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewUnitBuilderActions::SetName(name) => {
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                }
                state_mut.name = name;
            }
            NewUnitBuilderActions::SetDescription(description) => {
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                }
                state_mut.description = description;
            }
            NewUnitBuilderActions::SetSymbol(symbol) => {
                if symbol.is_none() {
                    state_mut.errors_symbol.push(ApiError::BadRequest(vec![
                        "The Symbol field is required.".to_string(),
                    ]));
                }
                state_mut.symbol = symbol;
            }
        }
        state
    }
}
#[function_component(NewUnitForm)]
pub fn new_unit_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewUnitBuilder>();
    let set_name = builder_dispatch
        .apply_callback(|name: Option<String>| NewUnitBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| {
        NewUnitBuilderActions::SetDescription(description)
    });
    let set_symbol = builder_dispatch
        .apply_callback(|symbol: Option<String>| NewUnitBuilderActions::SetSymbol(symbol));
    html! {
        <BasicForm<NewUnit> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
            <BasicInput label="Symbol" errors={builder_store.errors_symbol.clone()} builder={set_symbol} value={builder_store.symbol.clone()} input_type={InputType::Text} />
        </BasicForm<NewUnit>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default)]
#[store(storage = "session")]
pub struct NewUserBuilder {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub errors_first_name: Vec<ApiError>,
    pub errors_middle_name: Vec<ApiError>,
    pub errors_last_name: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NewUserBuilderActions {
    SetFirstName(Option<String>),
    SetMiddleName(Option<String>),
    SetLastName(Option<String>),
}

impl FormBuilder for NewUserBuilder {
    type Data = NewUser;
    type Actions = NewUserBuilderActions;

    fn has_errors(&self) -> bool {
        !self.errors_first_name.is_empty()
            || !self.errors_middle_name.is_empty()
            || !self.errors_last_name.is_empty()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }
}
impl From<NewUserBuilder> for NewUser {
    fn from(builder: NewUserBuilder) -> Self {
        Self {
            first_name: builder.first_name.unwrap(),
            middle_name: builder.middle_name,
            last_name: builder.last_name.unwrap(),
        }
    }
}
impl FormBuildable for NewUser {
    type Builder = NewUserBuilder;
    const TABLE: Table = Table::Users;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewUser"
    }
    fn task_target() -> &'static str {
        "NewUser"
    }
    fn description() -> &'static str {
        concat!("Create a new NewUser.",)
    }
    fn requires_authentication() -> bool {
        true
    }
}
impl Reducer<NewUserBuilder> for NewUserBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NewUserBuilder>) -> std::rc::Rc<NewUserBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewUserBuilderActions::SetFirstName(first_name) => {
                if first_name.is_none() {
                    state_mut.errors_first_name.push(ApiError::BadRequest(vec![
                        "The FirstName field is required.".to_string(),
                    ]));
                }
                state_mut.first_name = first_name;
            }
            NewUserBuilderActions::SetMiddleName(middle_name) => {
                state_mut.middle_name = middle_name;
            }
            NewUserBuilderActions::SetLastName(last_name) => {
                if last_name.is_none() {
                    state_mut.errors_last_name.push(ApiError::BadRequest(vec![
                        "The LastName field is required.".to_string(),
                    ]));
                }
                state_mut.last_name = last_name;
            }
        }
        state
    }
}
#[function_component(NewUserForm)]
pub fn new_user_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NewUserBuilder>();
    let set_first_name = builder_dispatch.apply_callback(|first_name: Option<String>| {
        NewUserBuilderActions::SetFirstName(first_name)
    });
    let set_middle_name = builder_dispatch.apply_callback(|middle_name: Option<String>| {
        NewUserBuilderActions::SetMiddleName(middle_name)
    });
    let set_last_name = builder_dispatch
        .apply_callback(|last_name: Option<String>| NewUserBuilderActions::SetLastName(last_name));
    html! {
        <BasicForm<NewUser> builder={builder_store.deref().clone()}>
            <BasicInput label="FirstName" errors={builder_store.errors_first_name.clone()} builder={set_first_name} value={builder_store.first_name.clone()} input_type={InputType::Text} />
            <BasicInput label="MiddleName" errors={builder_store.errors_middle_name.clone()} builder={set_middle_name} value={builder_store.middle_name.clone()} input_type={InputType::Text} />
            <BasicInput label="LastName" errors={builder_store.errors_last_name.clone()} builder={set_last_name} value={builder_store.last_name.clone()} input_type={InputType::Text} />
        </BasicForm<NewUser>>
    }
}
