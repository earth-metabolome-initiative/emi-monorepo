//! A submodule providing a list component which can be read from the database.

use std::rc::Rc;

use core_structures::tables::rows::Rows;
use web_common_traits::{
    crud::{CRUD, CrudTableOperation},
    database::{DeleteFromVec, UpsertVec},
    prelude::StaticTabular,
};
use yew::{Component, Context, Properties, html};
use yewdux::Dispatch;

use crate::{
    stores::app_state::AppState,
    traits::AssignedComponent,
    utils::{Connector, ConnectorMessage, DispatchableProperties},
    workers::dbws_worker::DB2CMessage,
};

#[derive(Debug, Clone, PartialEq, Properties)]
/// Properties for the `ReadableList` component.
pub struct ReadableListProps {
    #[prop_or(0)]
    /// The offset for the list of items to be read from the database.
    pub offset: u64,
    #[prop_or(256)]
    /// The limit for the list of items to be read from the database.
    pub limit: u64,
    /// The dispatch function for the application state.
    pub dispatch: Dispatch<AppState>,
}

impl From<&ReadableListProps> for Dispatch<AppState> {
    fn from(props: &ReadableListProps) -> Self {
        props.dispatch.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
/// Partial properties for the `ReadableList` component.
pub struct PartialReadableListProps {
    #[prop_or(0)]
    /// The offset for the list of items to be read from the database.
    pub offset: u64,
    #[prop_or(256)]
    /// The limit for the list of items to be read from the database.
    pub limit: u64,
}

impl DispatchableProperties for ReadableListProps {
    type PartialProperties = PartialReadableListProps;

    fn dispatchable(
        dispatch: Dispatch<AppState>,
        partial_properties: &Self::PartialProperties,
    ) -> Self {
        Self { offset: partial_properties.offset, limit: partial_properties.limit, dispatch }
    }
}

/// Submodule providing a list component which can be read from the database.
pub struct ReadableList<C: AssignedComponent> {
    items: Vec<Rc<C::Row>>,
    connector: Connector,
    _component: std::marker::PhantomData<C>,
}

impl<C> Component for ReadableList<C>
where
    C: AssignedComponent,
    Rc<Vec<Rc<C::Row>>>: TryFrom<Rows, Error = std::convert::Infallible>,
{
    type Message = ConnectorMessage;
    type Properties = ReadableListProps;

    fn create(ctx: &Context<Self>) -> Self {
        let connector: Connector = ctx.into();
        connector.send(CrudTableOperation::Read {
            table_name: <C::Row as StaticTabular>::static_table_name(),
            offset: ctx.props().offset,
            limit: ctx.props().limit,
        });
        Self { items: Vec::new(), connector, _component: std::marker::PhantomData }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ConnectorMessage::Worker(DB2CMessage::Rows(updated_rows, operation)) => {
                if updated_rows.is_empty() {
                    false
                } else {
                    let updated_rows: Rc<Vec<Rc<C::Row>>> = updated_rows.try_into().unwrap();
                    match operation {
                        CRUD::Read | CRUD::Create | CRUD::Update => {
                            self.items.upsert_sorted_vec(updated_rows.iter().cloned()).into()
                        }
                        CRUD::Delete => {
                            self.items.delete_from_sorted_vec(updated_rows.iter().cloned()) > 0
                        }
                    }
                }
            }
            other => self.connector.update(other),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <ul>
                { for self.items.iter().cloned().map(|row| {
                    let dispatch = self.connector.dispatch.clone();
                    html! {
                        <li><C row={row} dispatch={dispatch} /></li>
                    }
                })}
            </ul>
        }
    }
}
