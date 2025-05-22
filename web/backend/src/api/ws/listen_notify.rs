use std::collections::HashMap;

use core_structures::tables::{table_names::TableName, table_primary_keys::TablePrimaryKey};
use rand::Rng;
use tokio::sync::oneshot;
use web_common_traits::database::{Row as _, Tabular};
use ws_messages::{DBMessage, Subscription};

use crate::errors::BackendError;

/// Listen Notify command messages.
///
/// These are commands sent from the sessions to the listen notify server.
pub(crate) enum LNCommand {
    Subscribe(Subscription, u64, oneshot::Sender<()>),
    Unsubscribe(Subscription, u64, oneshot::Sender<()>),
    DB(DBMessage, oneshot::Sender<()>),
    Connect(tokio::sync::mpsc::UnboundedSender<DBMessage>, oneshot::Sender<u64>),
    Disconnect(u64, oneshot::Sender<()>),
}

#[allow(clippy::type_complexity)]
/// Listen Notify server.
pub struct ListenNotifyServer {
    sessions: HashMap<
        u64,
        (tokio::sync::mpsc::UnboundedSender<DBMessage>, Vec<TableName>, Vec<TablePrimaryKey>),
    >,
    table_listeners: HashMap<TableName, Vec<u64>>,
    row_listeners: HashMap<TablePrimaryKey, Vec<u64>>,
    /// Subscriptions and unsubscriptions receiver.
    subscriptions_receiver: tokio::sync::mpsc::UnboundedReceiver<LNCommand>,
}

impl ListenNotifyServer {
    #[must_use]
    /// Create a new Listen Notify server.
    pub fn new() -> (Self, ListenNotifyHandle) {
        let (subscriptions_sender, subscriptions_receiver) = tokio::sync::mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                table_listeners: HashMap::new(),
                row_listeners: HashMap::new(),
                subscriptions_receiver,
            },
            subscriptions_sender.into(),
        )
    }

    /// Register new session and assign unique ID to this session
    fn connect(&mut self, db_sender: tokio::sync::mpsc::UnboundedSender<DBMessage>) -> u64 {
        // register session with random connection ID
        let mut thread_rng = rand::thread_rng();
        let mut new_session_id = thread_rng.r#gen();
        while self.sessions.contains_key(&new_session_id) {
            new_session_id = thread_rng.r#gen();
        }

        log::info!("{new_session_id} joined");

        self.sessions.insert(new_session_id, (db_sender, Vec::new(), Vec::new()));

        // send new_session_id back
        new_session_id
    }

    /// Unregister connection from room map and broadcast disconnection message.
    fn disconnect(&mut self, session_id: u64) {
        log::info!("{session_id} left");

        // remove session from room map
        if let Some((_, table_listeners, row_listeners)) = self.sessions.remove(&session_id) {
            for table_name in table_listeners {
                if let Some(listeners) = self.table_listeners.get_mut(&table_name) {
                    listeners.retain(|&id| id != session_id);
                }
            }
            for row_key in row_listeners {
                if let Some(listeners) = self.row_listeners.get_mut(&row_key) {
                    listeners.retain(|&id| id != session_id);
                }
            }
        }
    }

    fn subscribe(&mut self, subscription: &Subscription, session_id: u64) {
        match subscription {
            Subscription::Table(table_name) => {
                if let Some((_, table_listeners, _)) = self.sessions.get_mut(&session_id) {
                    table_listeners.push(*table_name);
                    self.table_listeners.entry(*table_name).or_default().push(session_id);
                }
            }
            Subscription::Row(row_key) => {
                if let Some((_, _, row_listeners)) = self.sessions.get_mut(&session_id) {
                    row_listeners.push(*row_key);
                    self.row_listeners.entry(*row_key).or_default().push(session_id);
                }
            }
        }
    }

    fn unsubscribe(&mut self, subscription: &Subscription, session_id: u64) {
        match subscription {
            Subscription::Table(table_name) => {
                if let Some((_, table_listeners, _)) = self.sessions.get_mut(&session_id) {
                    table_listeners.retain(|name| name != table_name);
                    let mut orphan_sorce = false;
                    if let Some(listeners) = self.table_listeners.get_mut(table_name) {
                        listeners.retain(|&id| id != session_id);
                        if listeners.is_empty() {
                            orphan_sorce = true;
                        }
                    }
                    if orphan_sorce {
                        self.table_listeners.remove(table_name);
                    }
                }
            }
            Subscription::Row(row_key) => {
                if let Some((_, _, row_listeners)) = self.sessions.get_mut(&session_id) {
                    row_listeners.retain(|key| key != row_key);
                    let mut orphan_sorce = false;
                    if let Some(listeners) = self.row_listeners.get_mut(row_key) {
                        listeners.retain(|&id| id != session_id);
                        if listeners.is_empty() {
                            orphan_sorce = true;
                        }
                    }
                    if orphan_sorce {
                        self.row_listeners.remove(row_key);
                    }
                }
            }
        }
    }

    fn notify(&self, message: DBMessage) {
        match message {
            DBMessage::Row(row, crud) => {
                let table_name = row.table_name();
                if let Some(table_listeners) = self.table_listeners.get(&table_name) {
                    for session_id in table_listeners {
                        if let Some((db_sender, _, _)) = self.sessions.get(session_id) {
                            let _ = db_sender.send((row.clone(), crud).into());
                        }
                    }
                }
                if let Some(row_listeners) = self.row_listeners.get(&row.primary_key()) {
                    for session_id in row_listeners {
                        if let Some((db_sender, _, _)) = self.sessions.get(session_id) {
                            let _ = db_sender.send((row.clone(), crud).into());
                        }
                    }
                }
            }
            DBMessage::Rows(rows, crud) => {
                let table_name = rows.table_name();
                if let Some(table_listeners) = self.table_listeners.get(&table_name) {
                    for session_id in table_listeners {
                        if let Some((db_sender, _, _)) = self.sessions.get(session_id) {
                            let _ = db_sender.send((rows.clone(), crud).into());
                        }
                    }
                }
                for row in rows {
                    if let Some(row_listeners) = self.row_listeners.get(&row.primary_key()) {
                        for session_id in row_listeners {
                            if let Some((db_sender, _, _)) = self.sessions.get(session_id) {
                                let _ = db_sender.send((row.clone(), crud).into());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Run the listen notify server.
    ///
    /// # Errors
    ///
    /// * `std::io::Error` - If the server could not be started.
    pub async fn run(mut self) -> std::io::Result<()> {
        while let Some(cmd) = self.subscriptions_receiver.recv().await {
            match cmd {
                LNCommand::Connect(db_sender, id_sender) => {
                    let session_id = self.connect(db_sender);
                    let _ = id_sender.send(session_id);
                }

                LNCommand::Disconnect(session_id, sender) => {
                    self.disconnect(session_id);
                    let _ = sender.send(());
                }

                LNCommand::Subscribe(subscription, session_id, sender) => {
                    self.subscribe(&subscription, session_id);
                    let _ = sender.send(());
                }

                LNCommand::Unsubscribe(subscription, session_id, sender) => {
                    self.unsubscribe(&subscription, session_id);
                    let _ = sender.send(());
                }

                LNCommand::DB(message, sender) => {
                    self.notify(message);
                    let _ = sender.send(());
                }
            }
        }

        Ok(())
    }
}

/// Handle and command sender for chat server.
///
/// Reduces boilerplate of setting up response channels in WebSocket handlers.
#[derive(Debug, Clone)]
pub struct ListenNotifyHandle {
    subscriptions_sender: tokio::sync::mpsc::UnboundedSender<LNCommand>,
}

impl From<tokio::sync::mpsc::UnboundedSender<LNCommand>> for ListenNotifyHandle {
    fn from(subscriptions_sender: tokio::sync::mpsc::UnboundedSender<LNCommand>) -> Self {
        Self { subscriptions_sender }
    }
}

impl ListenNotifyHandle {
    /// Register client message sender and obtain connection ID.
    ///
    /// # Arguments
    ///
    /// * `conn_tx` - The message sender to register.
    ///
    /// # Errors
    ///
    /// * `BackendError` - If the connection could not be established.
    pub async fn connect(
        &self,
        conn_tx: tokio::sync::mpsc::UnboundedSender<DBMessage>,
    ) -> Result<u64, BackendError> {
        let (sender, receiver) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.subscriptions_sender.send(LNCommand::Connect(conn_tx, sender))?;

        // unwrap: chat server does not drop out response channel
        Ok(receiver.await?)
    }

    /// Unregister message sender and broadcast disconnection message to current
    /// room.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the session to disconnect.
    ///
    /// # Errors
    ///
    /// * `BackendError` - If the disconnection could not be completed.
    pub async fn disconnect(&self, id: u64) -> Result<(), BackendError> {
        let (sender, receiver) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.subscriptions_sender.send(LNCommand::Disconnect(id, sender))?;

        // unwrap: chat server does not drop our response channel
        Ok(receiver.await?)
    }

    /// Registers a new subscription to the given table / row.
    ///
    /// # Arguments
    ///
    /// * `subscription` - The subscription to register.
    /// * `session_id` - The session ID of the user to register the subscription
    ///
    /// # Errors
    ///
    /// * `BackendError` - If the subscription could not be registered.
    pub async fn subscribe(
        &self,
        subscription: Subscription,
        session_id: u64,
    ) -> Result<(), BackendError> {
        let (sender, receiver) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.subscriptions_sender.send(LNCommand::Subscribe(subscription, session_id, sender))?;

        // unwrap: chat server does not drop our response channel
        Ok(receiver.await?)
    }

    /// Registers a new subscription to the given table / row.
    ///
    /// # Arguments
    ///
    /// * `subscription` - The subscription to register.
    /// * `session_id` - The session ID of the user to register the subscription
    ///   for.
    ///
    /// # Errors
    ///
    /// * `BackendError` - If the subscription could not be registered.
    pub async fn unsubscribe(
        &self,
        subscription: Subscription,
        session_id: u64,
    ) -> Result<(), BackendError> {
        let (sender, receiver) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.subscriptions_sender.send(LNCommand::Unsubscribe(subscription, session_id, sender))?;

        // unwrap: chat server does not drop our response channel
        Ok(receiver.await?)
    }

    /// Send a message to the chat server.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send.
    ///
    /// # Errors
    ///
    /// * `BackendError` - If the message could not be sent.
    pub async fn notify(&self, message: DBMessage) -> Result<(), BackendError> {
        let (sender, receiver) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.subscriptions_sender.send(LNCommand::DB(message, sender))?;

        // unwrap: chat server does not drop our response channel
        Ok(receiver.await?)
    }
}
