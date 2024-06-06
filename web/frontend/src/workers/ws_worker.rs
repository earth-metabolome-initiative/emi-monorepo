use crate::components::forms::FormBuildable;

use super::database_type::*;
use futures::{SinkExt, StreamExt};
use gloo::timers::callback::Timeout;
use gloo_net::websocket::futures::WebSocket;
use gluesql::prelude::*;
use web_common::database::*;
// use sql_minifier::macros::load_sql;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;
use web_common::api::ws::messages::BackendMessage;
use web_common::api::ws::messages::CloseReason;
use web_common::api::ws::messages::FrontendMessage;
use web_common::api::ApiError;
use yew::platform::spawn_local;
use yew_agent::worker::HandlerId;
use yew_agent::worker::Worker;

const NOMINAL_CLOSURE_CODE: u16 = 1000;

type UserId = i32;
type DatabaseMessage = (Uuid, Option<UserId>, Operation);

pub struct WebsocketWorker {
    subscribers: HashSet<HandlerId>,
    tasks: HashMap<Uuid, HandlerId>,
    user: Option<Rc<NestedUser>>,
    database_sender: futures::channel::mpsc::Sender<DatabaseMessage>,
    websocket_sender: Option<futures::channel::mpsc::Sender<FrontendMessage>>,
    reconnection_attempt: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Messages from the frontend to the web-worker.
pub enum ComponentMessage {
    Operation(Operation),
    UserState(Option<Rc<NestedUser>>),
    // Connect to the provided hostname.
    Connect(String),
}

impl ComponentMessage {
    pub(crate) fn insert<R: Serialize + FormBuildable>(row: &R) -> Self {
        Self::Operation(Operation::Insert(
            R::TABLE.to_string(),
            bincode::serialize(row).unwrap(),
        ))
    }

    pub(crate) fn update<R: Serialize + FormBuildable>(row: &R) -> Self {
        Self::Operation(Operation::Update(
            R::TABLE.to_string(),
            bincode::serialize(row).unwrap(),
        ))
    }

    pub(crate) fn all_by_updated_at<R: Tabular + Filtrable>(
        filter: Option<R::Filter>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self::Operation(Operation::Select(Select::all_by_updated_at(
            R::TABLE,
            filter.map(|filter| bincode::serialize(&filter).unwrap()),
            limit,
            offset,
        )))
    }

    pub(crate) fn get<R: Tabular>(primary_key: PrimaryKey) -> Self {
        Self::Operation(Operation::Select(Select::id(R::TABLE, primary_key)))
    }

    pub(crate) fn get_named<S: ToString, R: Tabular>(
        operation_name: S,
        primary_key: PrimaryKey,
    ) -> Self {
        Self::Operation(Operation::Select(Select::id_with_operation_name(
            R::TABLE,
            operation_name.to_string(),
            primary_key,
        )))
    }

    pub(crate) fn can_view<R: Tabular>(primary_key: PrimaryKey) -> Self {
        Self::Operation(Operation::Select(Select::can_view(R::TABLE, primary_key)))
    }

    pub(crate) fn can_update<R: Tabular>(primary_key: PrimaryKey) -> Self {
        Self::Operation(Operation::Select(Select::can_update(R::TABLE, primary_key)))
    }

    pub(crate) fn can_admin<R: Tabular>(primary_key: PrimaryKey) -> Self {
        Self::Operation(Operation::Select(Select::can_admin(R::TABLE, primary_key)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Messages from the websocket web-worker to the frontend.
pub enum WebsocketMessage {
    Notification(NotificationMessage),
    SearchTable(Vec<u8>),
    GetTable(Option<String>, Vec<u8>),
    AllTable(Vec<u8>),
    CanView(bool),
    CanUpdate(bool),
    CanDelete(bool),
    /// Contains the serialized object in the cases
    /// where the operation was an update of get, and
    /// None in the case of a delete operation.
    Completed(Option<Vec<u8>>),
    Error(ApiError),
    RefreshUser(Rc<NestedUser>),
}

pub enum InternalMessage {
    Backend(BackendMessage),
    Frontend(HandlerId, ComponentMessage),
    Disconnect(Option<CloseReason>),
    User(Option<Rc<NestedUser>>),
    Reconnect(String)
}

impl WebsocketWorker {
    fn connect_to_database(
        scope: yew_agent::prelude::WorkerScope<Self>,
    ) -> futures::channel::mpsc::Sender<DatabaseMessage> {
        let (sender, mut receiver) = futures::channel::mpsc::channel::<DatabaseMessage>(1000);

        spawn_local(async move {
            let mut database = create_database().await;

            while let Some((task_id, user_id, operation)) = receiver.next().await {
                // TODO!
                // HANDLE THAT THE USER IS AUTHORIZED TO DO THE OPERATION!
                // DO NOT ALLOW OFFLINE USERS TO DO ILLEGAL OPERATIONS!

                scope.send_message(InternalMessage::Backend(match operation {
                    Operation::Delete(table_name, primary_key) => {
                        let table: Table = table_name.try_into().unwrap();
                        match table.delete(primary_key, &mut database).await {
                            Ok(row) => {
                                todo!()
                            }
                            Err(err) => BackendMessage::Error(task_id, ApiError::from(err)),
                        }
                    }
                    Operation::Insert(table_name, serialized_row) => {
                        let table: Table = table_name.try_into().unwrap();
                        match table
                            .insert(serialized_row, user_id.unwrap(), &mut database)
                            .await
                        {
                            Ok(row) => {
                                todo!()
                            }
                            Err(err) => BackendMessage::Error(task_id, ApiError::from(err)),
                        }
                    }
                    Operation::Select(select) => match select {
                        Select::Id {
                            table_name,
                            operation_name,
                            primary_key,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            match table.get(primary_key, &mut database).await {
                                Ok(Some(row)) => {
                                    BackendMessage::GetTable(task_id, operation_name, row)
                                }
                                Ok(None) => BackendMessage::Error(
                                    task_id,
                                    ApiError::BadRequest(vec![
                                        "No row found with the given primary key".to_string(),
                                    ]),
                                ),
                                Err(err) => BackendMessage::Error(task_id, err),
                            }
                        }
                        Select::All {
                            table_name,
                            filter,
                            limit,
                            offset,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            todo!()
                        }
                        Select::AllByUpdatedAt {
                            table_name,
                            filter,
                            limit,
                            offset,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            todo!()
                        }
                        Select::SearchTable {
                            filter,
                            table_name,
                            query,
                            limit,
                            offset,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            todo!()
                        }
                        Select::SearchEditableTable {
                            filter,
                            table_name,
                            query,
                            limit,
                            offset,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            todo!()
                        }
                        Select::SearchAll { query, limit } => {
                            todo!()
                        }
                        Select::CanView {
                            table_name,
                            primary_key,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            todo!()
                        }
                        Select::CanUpdate {
                            table_name,
                            primary_key,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            todo!()
                        }
                        Select::CanDelete {
                            table_name,
                            primary_key,
                        } => {
                            let table: Table = table_name.try_into().unwrap();

                            todo!()
                        }
                    },
                    Operation::Update(table_name, serialized_row) => {
                        let table: Table = table_name.try_into().unwrap();
                        match table
                            .update(serialized_row, user_id.unwrap(), &mut database)
                            .await
                        {
                            Ok(row) => {
                                todo!()
                            }
                            Err(err) => BackendMessage::Error(task_id, ApiError::from(err)),
                        }
                    }
                }));
            }
            log::debug!("Database sender closed");
        });

        sender
    }

    fn connect_to_websocket(
        hostname: &str,
        scope: &yew_agent::prelude::WorkerScope<Self>,
    ) -> Result<futures::channel::mpsc::Sender<FrontendMessage>, String> {
        let endpoint = web_common::api::ws::FULL_ENDPOINT;

        let websocket = WebSocket::open(&format!(
            "wss://{}{}",
            hostname,
            endpoint
        ))
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
                        Ok(message) => match message.try_into() {
                            Ok(message) => {
                                scope.send_message(InternalMessage::Backend(message));
                            }
                            Err(err) => {
                                log::error!("Error deserializing message: {:?}", err);
                            }
                        },
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
                    BackendMessage::AllTable(task_id, rows) => {
                        log::debug!("Received all table message");
                        // We save locally the table data (maybe?)
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, WebsocketMessage::AllTable(rows));
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
                            // When the user is offline, but some operation requires authentication, we need to
                            // return an error.
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
