use futures::StreamExt;
use gluesql::prelude::*;
use sql_minifier::macros::load_sql;
use std::collections::HashSet;
use wasm_bindgen::UnwrapThrowExt;
use yew::platform::spawn_local;
use yew_agent::worker::HandlerId;
use yew_agent::worker::Worker;

pub type Database = Glue<IdbStorage>;

pub enum DataBaseMessage {
    QueryResult(Vec<Payload>),
}

impl From<Vec<Payload>> for DataBaseMessage {
    fn from(payloads: Vec<Payload>) -> Self {
        Self::QueryResult(payloads)
    }
}

pub struct DBWorker {
    subscribers: HashSet<HandlerId>,
    sender: futures::channel::mpsc::Sender<String>,
}

impl DBWorker {
    fn connect(
        scope: yew_agent::prelude::WorkerScope<Self>,
    ) -> futures::channel::mpsc::Sender<String> {
        let (sender, mut receiver) = futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            // We connect to the database here
            let mut database = Glue::new(
                IdbStorage::new(Some("Earth Metabolome Initiative".to_string()))
                    .await
                    .unwrap(),
            );

            // Execute the initial SQL to create the tables
            // in the IndexedDB
            database
                .execute(
                    [load_sql!(
                        "../backend/migrations/00000000000012_create_taxa_table/up.sql"
                    )]
                    .join(";"),
                )
                .await
                .unwrap();

            // Then we start listening for queries
            while let Some(query) = receiver.next().await {
                scope.send_message::<Vec<Payload>>(database.execute(query).await.unwrap().into());
            }
        });

        sender
    }
}

impl Worker for DBWorker {
    type Message = DataBaseMessage;
    type Input = String;
    type Output = Vec<Payload>;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        Self {
            subscribers: HashSet::new(),
            sender: Self::connect(scope.clone()),
        }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        match internal_message {
            DataBaseMessage::QueryResult(payloads) => {
                for subscriber in &self.subscribers {
                    scope.respond(*subscriber, payloads.clone());
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

    fn received(
        &mut self,
        _scope: &yew_agent::prelude::WorkerScope<Self>,
        query: Self::Input,
        _id: HandlerId,
    ) {
        self.sender.try_send(query).unwrap_throw();
    }
}
