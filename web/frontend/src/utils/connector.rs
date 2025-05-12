//! Submodule providing the `Connector` struct and its associated methods.

use std::{fmt::Debug, rc::Rc};

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::{CrudPrimaryKeyOperation, CrudTableOperation};
use web_sys::console;
use ws_messages::DBMessage;
use yew::{Callback, Component, Context, Properties};
use yew_agent::{prelude::WorkerBridgeHandle, scope_ext::AgentScopeExt, worker::Worker};
use yewdux::Dispatch;

use super::dispatcher::DispatchableProperties;
use crate::{
    stores::app_state::AppState,
    workers::{DBWSWorker, dbws_worker::ComponentMessage},
};

#[derive(Properties, PartialEq)]
/// Default properties for a component which uses the `Connector`.
///
/// While such components may have richer properties, in several cases
/// they only require the `dispatch` property to communicate with the
/// application state. This struct provides a default implementation
/// of the `Properties` trait for such components.
pub struct ConnectorProps {
    /// The application state connector.
    pub dispatch: Dispatch<AppState>,
}

#[derive(Properties, PartialEq)]
/// Default properties for a component which uses the `Connector`.
///
/// While such components may have richer properties, in several cases
/// they only require the `dispatch` property to communicate with the
/// application state. This struct provides a default implementation
/// of the `Properties` trait for such components.
pub struct AssignedConnectorProps<Row>
where
    Row: PartialEq,
{
    /// The application state connector.
    pub dispatch: Dispatch<AppState>,
    /// The component row type.
    pub row: Row,
}

impl DispatchableProperties for ConnectorProps {
    type PartialProperties = ();

    fn dispatchable(
        dispatch: Dispatch<AppState>,
        _partial_properties: &Self::PartialProperties,
    ) -> Self {
        Self { dispatch }
    }
}

impl<'a> From<&'a ConnectorProps> for Dispatch<AppState> {
    fn from(props: &'a ConnectorProps) -> Self {
        props.dispatch.clone()
    }
}

impl From<ConnectorProps> for Dispatch<AppState> {
    fn from(props: ConnectorProps) -> Self {
        props.dispatch
    }
}

impl<'a, Row> From<&'a AssignedConnectorProps<Row>> for Dispatch<AppState>
where
    Row: PartialEq,
{
    fn from(props: &'a AssignedConnectorProps<Row>) -> Self {
        props.dispatch.clone()
    }
}

impl<Row> AsRef<Row> for AssignedConnectorProps<Row>
where
    Row: PartialEq,
{
    fn as_ref(&self) -> &Row {
        &self.row
    }
}

#[derive(Clone, PartialEq)]
/// Default message type for a component which uses the `Connector`.
///
/// While such components may have richer message types, in several cases
/// they only require the `ConnectorMessage` type to communicate with the
/// application state. This struct provides a default implementation
/// of the `Message` trait for such components.
pub enum ConnectorMessage {
    /// When the application state changes.
    State(Rc<AppState>),
    /// When the WebSocket worker sends a message.
    Worker(DBMessage),
    /// When the component sends a message to the worker.
    Component(ComponentMessage),
}

impl From<Rc<AppState>> for ConnectorMessage {
    fn from(state: Rc<AppState>) -> Self {
        ConnectorMessage::State(state)
    }
}

impl From<<DBWSWorker as Worker>::Output> for ConnectorMessage {
    fn from(output: <DBWSWorker as Worker>::Output) -> Self {
        ConnectorMessage::Worker(output)
    }
}

/// Connector struct that bridges the WebSocket worker and the Yew application.
pub(crate) struct Connector {
    websocket: WorkerBridgeHandle<DBWSWorker>,
    pub dispatch: Dispatch<AppState>,
    state: Rc<AppState>,
}

impl<'context, C> From<&'context Context<C>> for Connector
where
    C: Component,
    C::Message: From<ConnectorMessage>,
    for<'a> Dispatch<AppState>: From<&'a C::Properties>,
{
    fn from(ctx: &'context Context<C>) -> Self {
        Connector::new(ctx)
    }
}

impl From<CrudPrimaryKeyOperation<Row>> for ConnectorMessage {
    fn from(message: CrudPrimaryKeyOperation<Row>) -> Self {
        ConnectorMessage::Component(message.into())
    }
}

impl From<CrudTableOperation<Rows>> for ConnectorMessage {
    fn from(message: CrudTableOperation<Rows>) -> Self {
        ConnectorMessage::Component(message.into())
    }
}

impl web_common_traits::crud::Connector for Connector {
    type Row = Row;
    type Rows = Rows;
    type ComponentMessage = ComponentMessage;

    fn send<M>(&self, message: M)
    where
        M: Into<Self::ComponentMessage>,
    {
        self.websocket.send(message.into());
    }
}

impl Connector {
    /// Creates a new `Connector` instance.
    pub(crate) fn new<C: Component>(ctx: &Context<C>) -> Self
    where
        C::Message: From<ConnectorMessage>,
        for<'a> Dispatch<AppState>: From<&'a C::Properties>,
    {
        let dispatch: Dispatch<AppState> = ctx.props().into();
        console::log_1(&"Creating connector".to_string().into());

        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message| {
                    link.send_message(C::Message::from(ConnectorMessage::from(message)));
                }
            })),
            state: dispatch.get(),
            dispatch: dispatch.subscribe(
                ctx.link().callback(|state| C::Message::from(ConnectorMessage::from(state))),
            ),
        }
    }

    /// Sends a message to the WebSocket worker.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send to the worker.
    pub(crate) fn send<M>(&self, message: M)
    where
        M: Into<<DBWSWorker as Worker>::Input> + Debug,
    {
        console::log_1(&format!("Sending message: {message:?}").into());
        self.websocket.send(message.into());
    }

    #[must_use]
    /// Updates the `Connector` instance according to the provided message.
    pub(crate) fn update(&mut self, message: ConnectorMessage) -> bool {
        match message {
            ConnectorMessage::State(state) => {
                if self.state == state {
                    false
                } else {
                    self.state = state;
                    true
                }
            }
            ConnectorMessage::Component(component_message) => {
                self.websocket.send(component_message);
                false
            }
            ConnectorMessage::Worker(message) => {
                unimplemented!(
                    "The component should implement the `update` method to handle the worker message: `{message:?}`."
                );
            }
        }
    }
}
