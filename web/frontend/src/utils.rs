//! Submodule providing utility functions for the frontend.

mod connector;
pub(crate) use connector::{AssignedConnectorProps, Connector, ConnectorMessage, ConnectorProps};
mod dispatcher;
pub(crate) use dispatcher::{DispatchableProperties, Dispatcher};
