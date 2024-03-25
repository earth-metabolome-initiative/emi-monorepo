//! Test module to better understand how serde serialization works for conversion to and from JSON.
pub use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use web_common::api::database::{operations::Operation, selects::Query};


#[test]
fn test_update_serde_serialization() {
    let id: Uuid = Uuid::new_v4();
    let operation = Operation::Select(Query::PublicUser(id.into()));
    let serialized = serde_json::to_string(&operation).unwrap();
    let deserialized: Operation = serde_json::from_str(&serialized).unwrap();
    assert_eq!(operation, deserialized);
    let expected = json!({"teams": {"UPDATE": id.to_string()}});
    let actual = serde_json::to_value(&operation).unwrap();
    assert_eq!(expected, actual);
}