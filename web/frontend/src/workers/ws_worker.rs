use super::database_type::*;
use futures::{SinkExt, StreamExt};
use gloo::timers::callback::Timeout;
use gloo_net::websocket::futures::WebSocket;
use gluesql::prelude::*;
// use sql_minifier::macros::load_sql;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;
use web_common::api::ws::messages::BackendMessage;
use web_common::api::ws::messages::CloseReason;
use web_common::api::ws::messages::FrontendMessage;
use web_common::api::ApiError;
use web_common::database::Notification;
use web_common::database::NotificationMessage;
use web_common::database::Operation;
use web_common::database::Select;
use web_common::database::Table;
use web_common::database::Task;
use yew::platform::spawn_local;
use yew_agent::worker::HandlerId;
use yew_agent::worker::Worker;

const NOMINAL_CLOSURE_CODE: u16 = 1000;

pub struct WebsocketWorker {
    subscribers: HashSet<HandlerId>,
    tasks: HashMap<Uuid, HandlerId>,
    database: Option<Database>,
    database_sender: futures::channel::mpsc::Sender<FrontendMessage>,
    websocket_sender: Option<futures::channel::mpsc::Sender<FrontendMessage>>,
    reconnection_attempt: u32,
}

pub enum InternalMessage {
    Backend(BackendMessage),
    Frontend(HandlerId, FrontendMessage),
    Disconnect(Option<CloseReason>),
    Reconnect,
}

impl WebsocketWorker {
    fn connect_to_database(
        scope: yew_agent::prelude::WorkerScope<Self>,
    ) -> futures::channel::mpsc::Sender<FrontendMessage> {
        let (sender, mut receiver) = futures::channel::mpsc::channel::<FrontendMessage>(1000);

        spawn_local(async move {
            let mut database = create_database().await;

            while let Some(message) = receiver.next().await {
                match message {
                    FrontendMessage::Task(Task {
                        id,
                        start,
                        attempts,
                        operation,
                    }) => {
                        scope.send_message(InternalMessage::Backend(match operation {
                            Operation::Delete(table, primary_key) => {
                                match table.delete(primary_key, &mut database).await {
                                    Ok(row) => BackendMessage::Notification(
                                        NotificationMessage::without_row(Notification {
                                            id: 0,
                                            user_id: -1,
                                            operation: "DELETE".to_string(),
                                            table_name: table.to_string(),
                                            read: false,
                                        }),
                                    ),
                                    Err(err) => BackendMessage::Error(id, ApiError::from(err)),
                                }
                            }
                            Operation::Insert(insert) => {
                                // table.insert(insert)
                                todo!()
                            }
                            Operation::Select(select) => {
                                match select {
                                    Select::Id {
                                        table_name,
                                        primary_key,
                                    } => {
                                        let table: Table = table_name.try_into().unwrap();

                                        match table.get(primary_key, &mut database).await {
                                            Ok(Some(row)) => BackendMessage::GetTable(id, row),
                                            Ok(None) => BackendMessage::Error(
                                                id,
                                                ApiError::BadRequest(vec![
                                                    "No row found with the given primary key"
                                                        .to_string(),
                                                ]),
                                            ),
                                            Err(err) => {
                                                BackendMessage::Error(id, err)
                                            }
                                        }
                                    }
                                    Select::All {
                                        table_name,
                                        limit,
                                        offset,
                                    } => {
                                        let table: Table = table_name.try_into().unwrap();

                                        match table
                                            .all(Some(limit), Some(offset), &mut database)
                                            .await
                                        {
                                            Ok(rows) => BackendMessage::AllTable(id, rows),
                                            Err(err) => {
                                                BackendMessage::Error(id, err)
                                            }
                                        }
                                    }
                                    Select::SearchTable {
                                        table_name,
                                        query,
                                        number_of_results,
                                    } => {
                                        let table: Table = table_name.try_into().unwrap();

                                        // Since GlueSQL does not support search queries, and we do not expect for the
                                        // frontend side to ever need to search a very large table, we always return all
                                        // of the rows in the table and then we let the datalist UI component handle the
                                        // search directly. Still, just in case something unexpected happens, we limit the
                                        // number of rows returned to 1_000.
                                        match table.all(Some(1_000), None, &mut database).await {
                                            Ok(rows) => BackendMessage::SearchTable(id, rows),
                                            Err(err) => {
                                                BackendMessage::Error(id, err)
                                            }
                                        }
                                    }
                                }
                            }
                            Operation::Update(update) => {
                                // table.update(update)
                                todo!()
                            }
                        }));
                    }
                    FrontendMessage::Close(_) => {}
                }
            }
            log::debug!("Database sender closed");
        });

        sender
    }

    fn connect_to_websocket(
        scope: &yew_agent::prelude::WorkerScope<Self>,
    ) -> Result<futures::channel::mpsc::Sender<FrontendMessage>, String> {
        let endpoint = web_common::api::ws::FULL_ENDPOINT;

        let websocket = WebSocket::open(&format!(
            "wss://{}{}",
            include_str!("../../.domain"),
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
                scope.send_message(InternalMessage::Reconnect);
            });
        }

        Ok(sender)
    }
}

impl Worker for WebsocketWorker {
    type Message = InternalMessage;
    type Input = FrontendMessage;
    type Output = BackendMessage;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        Self {
            subscribers: HashSet::new(),
            tasks: HashMap::new(),
            websocket_sender: Self::connect_to_websocket(&scope).ok(),
            database_sender: Self::connect_to_database(scope.clone()),
            database: None,
            reconnection_attempt: 0,
        }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        match internal_message {
            InternalMessage::Backend(backend_message) => {
                match backend_message {
                    BackendMessage::Notification(_notification) => {
                        // TODO! HANDLE UPDATE OF THE DATABASE!
                    }
                    BackendMessage::Completed(task_id) => {
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, BackendMessage::Completed(task_id));
                        }
                    }
                    BackendMessage::Error(task_id, error) => {
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, BackendMessage::Error(task_id, error));
                        }
                    }
                    BackendMessage::Close(close_reason) => {
                        // We need to close the websocket connection.
                        scope.send_message(InternalMessage::Disconnect(close_reason));
                    }
                    BackendMessage::GetTable(task_id, row) => {
                        // We save locally the table data (maybe?)
                        // We can remove this task from the queue.
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, BackendMessage::GetTable(task_id, row));
                        }
                    }
                    BackendMessage::AllTable(task_id, rows) => {
                        // We save locally the table data (maybe?)
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, BackendMessage::AllTable(task_id, rows));
                        }
                    }
                    BackendMessage::SearchTable(task_id, rows) => {
                        // We save locally the search results (maybe?)
                        if let Some(subscriber_id) = self.tasks.remove(&task_id) {
                            scope.respond(subscriber_id, BackendMessage::SearchTable(task_id, rows));
                        }
                    }
                    BackendMessage::RefreshUser(user) => {
                        // We need to update the access token in the user state.
                        for sub in &self.subscribers {
                            scope.respond(*sub, BackendMessage::RefreshUser(user.clone()));
                        }
                    }
                };
            }
            InternalMessage::Frontend(subscribed_id, message) => {
                if let FrontendMessage::Task(task) = &message {
                    self.tasks.insert(task.id(), subscribed_id);
                }
                if self
                    .websocket_sender
                    .as_mut()
                    .map_or(true, |sender| sender.try_send(message.clone()).is_err())
                {
                    self.database_sender.try_send(message).unwrap();
                }
            }
            InternalMessage::Disconnect(_closure_code) => {
                if let Some(mut sender) = self.websocket_sender.take() {
                    spawn_local(async move {
                        let _ = sender.close().await;
                    });
                }
            }
            InternalMessage::Reconnect => {
                if let Some(mut sender) = self.websocket_sender.take() {
                    spawn_local(async move {
                        let _ = sender.close().await;
                    });
                }
                if let Ok(sender) = Self::connect_to_websocket(scope) {
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
