use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::selects::Select;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum PrimaryKey {
    Uuid(Uuid),
    Uuid2(Uuid, Uuid),
    Uuid3(Uuid, Uuid, Uuid),
    Int(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Mixed(Uuid, i32),
    Mixed2(Uuid, i32, i32),
}

impl PrimaryKey {
    /// Returns the extended version of the primary key with the provided UUID.
    ///
    /// # Arguments
    /// * `uuid` - The UUID to extend the primary key with.
    pub fn extend_uuid(self, uuid: Uuid) -> PrimaryKey {
        match self {
            PrimaryKey::Uuid(uuid1) => PrimaryKey::Uuid2(uuid1, uuid),
            PrimaryKey::Uuid2(uuid1, uuid2) => PrimaryKey::Uuid3(uuid1, uuid2, uuid),
            _ => unreachable!("Cannot extend PrimaryKey with Uuid: {:?}", self),
        }
    }

    /// Returns the extended version of the primary key with the provided integer.
    ///
    /// # Arguments
    /// * `int` - The integer to extend the primary key with.
    pub fn extend_int(self, int: i32) -> PrimaryKey {
        match self {
            PrimaryKey::Int(int1) => PrimaryKey::Int2(int1, int),
            PrimaryKey::Int2(int1, int2) => PrimaryKey::Int3(int1, int2, int),
            PrimaryKey::Uuid(uuid) => PrimaryKey::Mixed(uuid, int),
            PrimaryKey::Mixed(uuid, int1) => PrimaryKey::Mixed2(uuid, int1, int),
            _ => unreachable!("Cannot extend PrimaryKey with i32: {:?}", self),
        }
    }
}

impl From<PrimaryKey> for Uuid {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Uuid(uuid) => uuid,
            _ => unreachable!("Cannot convert PrimaryKey to Uuid: {:?}", pk),
        }
    }
}

impl From<PrimaryKey> for (Uuid, i32) {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Mixed(uuid, int) => (uuid, int),
            _ => unreachable!("Cannot convert PrimaryKey to (Uuid, i32): {:?}", pk),
        }
    }
}

impl From<PrimaryKey> for (Uuid, i32, i32) {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Mixed2(uuid, int1, int2) => (uuid, int1, int2),
            _ => unreachable!("Cannot convert PrimaryKey to (Uuid, i32, i32): {:?}", pk),
        }
    }
}

impl From<PrimaryKey> for (Uuid, Uuid) {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Uuid2(uuid1, uuid2) => (uuid1, uuid2),
            _ => unreachable!("Cannot convert PrimaryKey to (Uuid, Uuid): {:?}", pk),
        }
    }
}

impl From<PrimaryKey> for (Uuid, Uuid, Uuid) {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Uuid3(uuid1, uuid2, uuid3) => (uuid1, uuid2, uuid3),
            _ => unreachable!("Cannot convert PrimaryKey to (Uuid, Uuid, Uuid): {:?}", pk),
        }
    }
}

impl From<PrimaryKey> for i32 {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Int(int) => int,
            _ => unreachable!("Cannot convert PrimaryKey to i32: {:?}", pk),
        }
    }
}

impl From<PrimaryKey> for (i32, i32) {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Int2(int1, int2) => (int1, int2),
            _ => unreachable!("Cannot convert PrimaryKey to (i32, i32): {:?}", pk),
        }
    }
}

impl From<PrimaryKey> for (i32, i32, i32) {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Int3(int1, int2, int3) => (int1, int2, int3),
            _ => unreachable!("Cannot convert PrimaryKey to (i32, i32, i32): {:?}", pk),
        }
    }
}

impl From<Uuid> for PrimaryKey {
    fn from(uuid: Uuid) -> Self {
        PrimaryKey::Uuid(uuid)
    }
}

impl From<(Uuid, i32)> for PrimaryKey {
    fn from(mixed: (Uuid, i32)) -> Self {
        PrimaryKey::Mixed(mixed.0, mixed.1)
    }
}

impl From<(Uuid, Uuid)> for PrimaryKey {
    fn from(uuids: (Uuid, Uuid)) -> Self {
        PrimaryKey::Uuid2(uuids.0, uuids.1)
    }
}

impl From<(Uuid, Uuid, Uuid)> for PrimaryKey {
    fn from(uuids: (Uuid, Uuid, Uuid)) -> Self {
        PrimaryKey::Uuid3(uuids.0, uuids.1, uuids.2)
    }
}

impl From<i32> for PrimaryKey {
    fn from(int: i32) -> Self {
        PrimaryKey::Int(int)
    }
}

impl From<(i32, i32)> for PrimaryKey {
    fn from(ints: (i32, i32)) -> Self {
        PrimaryKey::Int2(ints.0, ints.1)
    }
}

impl From<(i32, i32, i32)> for PrimaryKey {
    fn from(ints: (i32, i32, i32)) -> Self {
        PrimaryKey::Int3(ints.0, ints.1, ints.2)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Operation {
    // When the frontend selects something to show to
    // the user, it sends a Select message to the backend
    // for a specific view.
    Select(Select),
    // When the frontend wants to delete a row from a table
    // it sends a Delete message to the backend.
    Delete(String, PrimaryKey),
    // When the frontend wants to update a row from a table
    // it sends a variant of the Update enumeration.
    Update(String, Vec<u8>),
    // When the frontend wants to insert a row into a table
    // it sends a variant of the Insert enumeration.
    Insert(String, Vec<u8>),
}

impl Operation {
    /// Returns whether the current operation is an insert.
    pub fn is_insert(&self) -> bool {
        match self {
            Operation::Insert(_, _) => true,
            _ => false,
        }
    }

    /// Returns whether the current operation is a delete.
    pub fn is_delete(&self) -> bool {
        match self {
            Operation::Delete(_, _) => true,
            _ => false,
        }
    }

    /// Returns whether the current operation is an update.
    pub fn is_update(&self) -> bool {
        match self {
            Operation::Update(_, _) => true,
            _ => false,
        }
    }

    pub fn requires_authentication(&self) -> bool {
        match self {
            Operation::Select(select) => select.requires_authentication(),
            _ => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: Uuid,
    pub start: chrono::DateTime<chrono::Utc>,
    pub attempts: u8,
    pub operation: Operation,
}

impl From<Operation> for Task {
    fn from(operation: Operation) -> Self {
        Task {
            id: Uuid::new_v4(),
            start: chrono::Utc::now(),
            attempts: 0,
            operation,
        }
    }
}

impl Task {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn increase_attemps(&mut self) {
        self.attempts += 1;
    }

    /// Returns whether the task should be retried.
    ///
    /// # Implementative details
    /// Depending on whether enough time has passed, determined as
    /// 2^attempts * 5 seconds, the task should be retried.
    pub fn should_retry(&self) -> bool {
        if self.attempts == 0 {
            return true;
        }
        let elapsed = chrono::Utc::now() - self.start;
        let retry_time = (2u32.pow(self.attempts as u32) * 20).max(60 * 10);
        elapsed.num_seconds() > retry_time as i64
    }
}
