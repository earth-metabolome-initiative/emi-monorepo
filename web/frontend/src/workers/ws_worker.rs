use futures::{SinkExt, StreamExt};
use gloo::timers::callback::Timeout;
use gloo_net::websocket::futures::WebSocket;
use std::collections::HashSet;
use std::fmt::Debug;
use wasm_bindgen::UnwrapThrowExt;
use web_common::api::ws::messages::*;
use web_common::api::ws::FULL_ENDPOINT;
use yew::platform::spawn_local;
use yew_agent::worker::HandlerId;
use yew_agent::worker::Worker;

const NOMINAL_CLOSURE_CODE: u16 = 1000;

pub struct WebsocketWorker<FM, BM> {
    subscribers: HashSet<HandlerId>,
    sender: Option<futures::channel::mpsc::Sender<FM>>,
    reconnection_attempt: u32,
    _phantom: std::marker::PhantomData<BM>,
}

pub enum InternalMessage<BM> {
    Backend(BM),
    Disconnect(Option<u16>),
    Reconnect,
}

impl<FM, BM> WebsocketWorker<FM, BM>
where
    FM: Into<gloo_net::websocket::Message> + Clone + 'static + Debug + AuthenticationMessage,
    BM: From<gloo_net::websocket::Message> + Clone + 'static + Debug + AuthenticatedMessage,
{
    fn connect(
        scope: &yew_agent::prelude::WorkerScope<Self>,
    ) -> Result<futures::channel::mpsc::Sender<FM>, String> {
        let websocket = WebSocket::open(&format!("wss://emi.local{}", FULL_ENDPOINT))
            .map_err(|err| format!("Error opening websocket connection: {:?}", err))?;

        match websocket.state() {
            gloo_net::websocket::State::Open => {}
            gloo_net::websocket::State::Connecting => {}
            _ => {
                return Err("Websocket connection is not open".to_string());
            }
        }

        let (mut write, mut read) = websocket.split();

        let (sender, mut receiver) = futures::channel::mpsc::channel::<FM>(1000);

        spawn_local(async move {
            while let Some(frontend_message) = receiver.next().await {
                log::debug!("Sending message to websocket: {:?}", frontend_message);
                if write.send(frontend_message.into()).await.is_err() {
                    log::error!("Error sending to websocket");
                    break;
                }
            }
            log::debug!("Websocket sender closed");
        });

        {
            let scope = scope.clone();
            spawn_local(async move {
                while let Some(backend_message) = read.next().await {
                    match backend_message {
                        Ok(message) => {
                            scope.send_message(InternalMessage::Backend(message.into()));
                        }
                        Err(err) => {
                            log::error!("Error reading from websocket: {:?}", err);
                            break;
                        }
                    }
                }
                scope.send_message(InternalMessage::Reconnect);
            });
        }

        Ok(sender)
    }

    pub fn is_connected(&self) -> bool {
        self.sender.is_some()
    }
}

impl<FM, BM> Worker for WebsocketWorker<FM, BM>
where
    FM: Into<gloo_net::websocket::Message> + Clone + 'static + Debug + AuthenticationMessage,
    BM: From<gloo_net::websocket::Message> + Clone + 'static + Debug + AuthenticatedMessage,
{
    type Message = InternalMessage<BM>;
    type Input = FM;
    type Output = BM;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        Self {
            subscribers: HashSet::new(),
            sender: Some(Self::connect(scope).unwrap_throw()),
            reconnection_attempt: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        match internal_message {
            InternalMessage::Backend(backend_message) => {
                log::debug!("Received message from websocket: {:?}", backend_message);
                for sub in &self.subscribers {
                    scope.respond(*sub, backend_message.clone());
                }
            }
            InternalMessage::Disconnect(_closure_code) => {
                if let Some(mut sender) = self.sender.take() {
                    spawn_local(async move {
                        let _ = sender.close().await;
                    });
                }
            }
            InternalMessage::Reconnect => {
                if let Some(mut sender) = self.sender.take() {
                    spawn_local(async move {
                        let _ = sender.close().await;
                    });
                }
                if let Ok(sender) = Self::connect(scope) {
                    log::debug!("Reconnected to websocket");
                    self.reconnection_attempt = 0;
                    self.sender = Some(sender);
                } else {
                    log::debug!(
                        "Failed to reconnect to websocket, attempting again in {} seconds",
                        2_u32.pow(self.reconnection_attempt)
                    );
                    self.reconnection_attempt += 1;
                    let scope = scope.clone();
                    Timeout::new(2_u32.pow(self.reconnection_attempt) * 1000, move || {
                        scope.send_message(InternalMessage::Reconnect);
                    })
                    .forget();
                }
            }
        }
    }

    fn connected(
        &mut self,
        _scope: &yew_agent::prelude::WorkerScope<Self>,
        id: yew_agent::worker::HandlerId,
    ) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, _scope: &yew_agent::prelude::WorkerScope<Self>, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn destroy(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        _destruct: yew_agent::worker::WorkerDestroyHandle<Self>,
    ) {
        log::debug!("Destroying websocket worker");
        scope.send_message(InternalMessage::Disconnect(Some(NOMINAL_CLOSURE_CODE)));
    }

    fn received(
        &mut self,
        _scope: &yew_agent::prelude::WorkerScope<Self>,
        frontend_message: Self::Input,
        _id: HandlerId,
    ) {
        log::debug!(
            "Attempting to send message from frontend: {:?}",
            frontend_message
        );
        if let Some(sender) = &mut self.sender {
            match sender.try_send(frontend_message) {
                Ok(()) => {}
                Err(err) => {
                    log::error!("Error sending message to websocket: {:?}", err);
                }
            }
        }
    }
}
