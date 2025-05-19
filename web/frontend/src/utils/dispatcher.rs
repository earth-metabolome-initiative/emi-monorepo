//! Submodule providing the `Dispatcher` component, which takes care of adding
//! the `dispatch` property to an underlying component.

use yew::{Component, Html, Properties, function_component, html};
use yewdux::{Dispatch, use_dispatch};

use crate::stores::app_state::AppState;

pub trait DispatchableComponent:
    Component<Properties = <Self as DispatchableComponent>::DispatchableProperties>
{
    type PartialProperties: Properties;
    type DispatchableProperties: DispatchableProperties<PartialProperties = Self::PartialProperties>;
}

impl<C> DispatchableComponent for C
where
    C: Component,
    C::Properties: DispatchableProperties,
{
    type PartialProperties =
        <<C as Component>::Properties as DispatchableProperties>::PartialProperties;
    type DispatchableProperties = <C as Component>::Properties;
}

pub trait DispatchableProperties: Properties {
    type PartialProperties: Properties;

    fn dispatchable(
        dispatch: Dispatch<AppState>,
        partial_properties: &Self::PartialProperties,
    ) -> Self;
}

impl DispatchableProperties for () {
    type PartialProperties = ();

    fn dispatchable(
        _dispatch: Dispatch<AppState>,
        _partial_properties: &Self::PartialProperties,
    ) -> Self {
    }
}

#[function_component]
pub(crate) fn Dispatcher<C>(partial_properties: &C::PartialProperties) -> Html
where
    C: DispatchableComponent,
{
    let dispatch = use_dispatch::<AppState>();
    let properties = C::DispatchableProperties::dispatchable(dispatch, partial_properties);
    html! {
        <C ..properties />
    }
}
