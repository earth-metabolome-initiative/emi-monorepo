use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use futures::{SinkExt, StreamExt};
use gloo::timers::callback::Timeout;
use gloo_net::websocket::futures::WebSocket;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_common::api::{
    ws::messages::{BackendMessage, CloseReason, FrontendMessage},
    ApiError,
};
use yew::platform::spawn_local;
use yew_agent::worker::{HandlerId, Worker};
use core_structures::User;

const NOMINAL_CLOSURE_CODE: u16 = 1000;

type UserId = i32;
type DatabaseMessage = (Uuid, Option<UserId>, Operation);

pub struct WebsocketWorker {
    subscribers: HashSet<HandlerId>,
    tasks: HashMap<Uuid, HandlerId>,
    user: Option<Rc<User>>,
    database_sender: futures::channel::mpsc::Sender<DatabaseMessage>,
    websocket_sender: Option<futures::channel::mpsc::Sender<FrontendMessage>>,
    reconnection_attempt: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Messages from the frontend to the web-worker.
pub enum ComponentMessage {
    Operation(Operation),
    UserState(Option<Rc<User>>),
    // Connect to the provided hostname.
    Connect(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Messages from the websocket web-worker to the frontend.
pub enum WebsocketMessage {
    Notification(NotificationMessage),
    SearchTable(Vec<u8>),
    GetTable(Option<String>, Vec<u8>),
    CanView(bool),
    CanUpdate(bool),
    CanDelete(bool),
    /// Contains the serialized object in the cases
    /// where the operation was an update of get, and
    /// None in the case of a delete operation.
    Completed(Option<Vec<u8>>),
    Error(ApiError),
    RefreshUser(Rc<User>),
}

pub enum InternalMessage {
    Backend(BackendMessage),
    Frontend(HandlerId, ComponentMessage),
    Disconnect(Option<CloseReason>),
    User(Option<Rc<User>>),
    Reconnect(String),
}

impl WebsocketWorker {
    fn connect_to_websocket(
        hostname: &str,
        scope: &yew_agent::prelude::WorkerScope<Self>,
    ) -> Result<futures::channel::mpsc::Sender<FrontendMessage>, String> {
        let endpoint = web_common::api::ws::FULL_ENDPOINT;

        let websocket = WebSocket::open(&format!("wss://{}{}", hostname, endpoint))
            .map_err(|err| format!("Error opening websocket connection: {:?}", err))?;

        match websocket.state() {
            gloo_net::websocket::State::Open => {}
            gloo_net::websocket::State::Connecting => {}
            _ => {
                return Err("Websocket connection is not open".to_string());
            }
        }

        let (mut write, mut read) = websocket.split();

        let (sender, mut receiver) = futures::channel::mpsc::channel::<FrontendMessage>(1000);

        spawn_local(async move {
            while let Some(frontend_message) = receiver.next().await {
                if write.send(frontend_message.into()).await.is_err() {
                    log::error!("Error sending to websocket");
                    break;
                }
            }
            log::debug!("Websocket sender closed");
        });

        let hostname = hostname.to_owned();

        {
            let scope = scope.clone();
            spawn_local(async move {
                while let Some(backend_message) = read.next().await {
                    match backend_message {
                        Ok(message) => {
                            match message.try_into() {
                                Ok(message) => {
                                    scope.send_message(InternalMessage::Backend(message));
                                }
                                Err(err) => {
                                    log::error!("Error deserializing message: {:?}", err);
                                }
                            }
                        }
                        Err(err) => {
                            log::error!("Error reading from websocket: {:?}", err);
                            break;
                        }
                    }
                }
                scope.send_message(InternalMessage::Reconnect(hostname));
            });
        }

        Ok(sender)
    }
}

impl Worker for WebsocketWorker {
    type Message = InternalMessage;
    type Input = ComponentMessage;
    type Output = WebsocketMessage;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        Self {
            subscribers: HashSet::new(),
            tasks: HashMap::new(),
            user: None,
            websocket_sender: None,
            database_sender: Self::connect_to_database(scope.clone()),
            reconnection_attempt: 0,
        }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        match internal_message {
            InternalMessage::User(user) => {
                self.user = user;
            }
            InternalMessage::Backend(backend_message) => {
                match backend_message {
                    BackendMessage::CanView(task_id, can_view) => {
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, WebsocketMessage::CanView(can_view));
                        }
                    }
                    BackendMessage::CanUpdate(task_id, can_update) => {
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, WebsocketMessage::CanUpdate(can_update));
                        }
                    }
                    BackendMessage::CanDelete(task_id, can_admin) => {
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, WebsocketMessage::CanDelete(can_admin));
                        }
                    }
                    BackendMessage::Notification(notification) => {
                        // TODO! HANDLE UPDATE OF THE DATABASE!
                        log::debug!("Notification received: {:?}", notification);
                    }
                    BackendMessage::Completed(task_id, maybe_row) => {
                        log::debug!("Task completed: {:?}", task_id);
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, WebsocketMessage::Completed(maybe_row));
                        }
                    }
                    BackendMessage::Error(task_id, error) => {
                        log::debug!("Task failed: {:?}", error);
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, WebsocketMessage::Error(error));
                        }
                    }
                    BackendMessage::Close(close_reason) => {
                        // We need to close the websocket connection.
                        scope.send_message(InternalMessage::Disconnect(close_reason));
                    }
                    BackendMessage::GetTable(task_id, task_name, row) => {
                        // We save locally the table data (maybe?)
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope
                                .respond(subscriber_id, WebsocketMessage::GetTable(task_name, row));
                        }
                    }
                    BackendMessage::SearchTable(task_id, rows) => {
                        // We save locally the search results (maybe?)
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, WebsocketMessage::SearchTable(rows));
                        }
                    }
                    BackendMessage::RefreshUser(user) => {
                        let rc_user = Rc::from(user);
                        self.user = Some(rc_user.clone());

                        // We need to update the access token in the user state.
                        for sub in &self.subscribers {
                            scope.respond(*sub, WebsocketMessage::RefreshUser(rc_user.clone()));
                        }
                    }
                };
            }
            InternalMessage::Frontend(subscriber_id, message) => {
                match message {
                    ComponentMessage::Connect(hostname) => {
                        scope.send_message(InternalMessage::Reconnect(hostname));
                    }
                    ComponentMessage::UserState(user) => {
                        scope.send_message(InternalMessage::User(user));
                    }
                    ComponentMessage::Operation(operation) => {
                        if operation.requires_authentication() && self.user.is_none() {
                            // When the user is offline, but some operation requires authentication,
                            // we need to return an error.
                            log::error!("Unauthorized operation: {:?}", operation);
                            scope.respond(
                                subscriber_id,
                                WebsocketMessage::Error(ApiError::Unauthorized),
                            );
                            return;
                        }

                        // TODO! Add here the task to the client database!

                        let task_id = Uuid::new_v4();
                        self.tasks.insert(task_id, subscriber_id);

                        if self.websocket_sender.as_mut().map_or(true, |sender| {
                            sender
                                .try_send(FrontendMessage::Task(task_id, operation.clone()))
                                .is_err()
                        }) {
                            self.database_sender
                                .try_send((
                                    task_id,
                                    self.user.as_ref().map(|user| user.inner.id),
                                    operation,
                                ))
                                .unwrap();
                        }
                    }
                }
            }
            InternalMessage::Disconnect(_closure_code) => {
                if let Some(mut sender) = self.websocket_sender.take() {
                    spawn_local(async move {
                        let _ = sender.close().await;
                    });
                }
            }
            InternalMessage::Reconnect(hostname) => {
                if let Some(mut sender) = self.websocket_sender.take() {
                    spawn_local(async move {
                        let _ = sender.close().await;
                    });
                }
                if let Ok(sender) = Self::connect_to_websocket(&hostname, scope) {
                    log::debug!("Reconnected to websocket");
                    self.reconnection_attempt = 0;
                    self.websocket_sender = Some(sender);
                    // TODO! RESEND ALL TASKS!
                } else {
                    log::debug!(
                        "Failed to reconnect to websocket, attempting again in {} seconds",
                        2_u32.pow(self.reconnection_attempt)
                    );
                    self.reconnection_attempt += 1;
                    let scope = scope.clone();
                    Timeout::new(2_u32.pow(self.reconnection_attempt) * 1000, move || {
                        scope.send_message(InternalMessage::Reconnect(hostname));
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
        scope.send_message(InternalMessage::Disconnect(Some(CloseReason::new(
            NOMINAL_CLOSURE_CODE,
            Some("Worker destroyed"),
        ))));
    }

    fn received(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        frontend_message: Self::Input,
        subscriber_id: HandlerId,
    ) {
        scope.send_message(InternalMessage::Frontend(subscriber_id, frontend_message));
    }
}
