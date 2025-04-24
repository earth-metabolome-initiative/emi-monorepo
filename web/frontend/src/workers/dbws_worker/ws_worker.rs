//! Submodule proving the Websocket-related methods for the `DBWSWorker`
//! struct, including operations such as connecting to the WebSocket.

use super::internal_message::ws_internal_message::WSInternalMessage;

impl super::DBWSWorker {
    pub(super) fn update_websocket(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        ws_message: WSInternalMessage,
    ) {
    }
}
