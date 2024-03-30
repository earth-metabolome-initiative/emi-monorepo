use chrono::DateTime;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Archivable {
    pub id: Uuid,
    pub archived_at: NaiveDateTime,
    pub archived_by: Uuid,
}
#[cfg(feature = "frontend")]
impl Archivable {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("archivables")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, archived_at, archived_by")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            archived_at: match row.get("archived_at").unwrap() {
                gluesql::prelude::Value::Timestamp(archived_at) => archived_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            archived_by: match row.get("archived_by").unwrap() {
                gluesql::prelude::Value::Uuid(archived_by) => Uuid::from_u128(*archived_by),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContainerHorizontalRule {
    pub id: Uuid,
    pub item_type_id: Option<Uuid>,
    pub other_item_type_id: Option<Uuid>,
    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,
    pub minimum_humidity: Option<f64>,
    pub maximum_humidity: Option<f64>,
    pub minimum_pressure: Option<f64>,
    pub maximum_pressure: Option<f64>,
}
#[cfg(feature = "frontend")]
impl ContainerHorizontalRule {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_horizontal_rules")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_type_id, other_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_type_id: match row.get("item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_type_id) => Some(Uuid::from_u128(*item_type_id)),
                _ => unreachable!("Expected Uuid"),
            },
            other_item_type_id: match row.get("other_item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(other_item_type_id) => {
                    Some(Uuid::from_u128(*other_item_type_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            minimum_temperature: match row.get("minimum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_temperature) => {
                    Some(minimum_temperature.clone())
                }
                _ => unreachable!("Expected F64"),
            },
            maximum_temperature: match row.get("maximum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_temperature) => {
                    Some(maximum_temperature.clone())
                }
                _ => unreachable!("Expected F64"),
            },
            minimum_humidity: match row.get("minimum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_humidity) => Some(minimum_humidity.clone()),
                _ => unreachable!("Expected F64"),
            },
            maximum_humidity: match row.get("maximum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_humidity) => Some(maximum_humidity.clone()),
                _ => unreachable!("Expected F64"),
            },
            minimum_pressure: match row.get("minimum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_pressure) => Some(minimum_pressure.clone()),
                _ => unreachable!("Expected F64"),
            },
            maximum_pressure: match row.get("maximum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_pressure) => Some(maximum_pressure.clone()),
                _ => unreachable!("Expected F64"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContainerVerticalRule {
    pub id: Uuid,
    pub container_item_type_id: Option<Uuid>,
    pub contained_item_type_id: Option<Uuid>,
    pub minimum_temperature: Option<f64>,
    pub maximum_temperature: Option<f64>,
    pub minimum_humidity: Option<f64>,
    pub maximum_humidity: Option<f64>,
    pub minimum_pressure: Option<f64>,
    pub maximum_pressure: Option<f64>,
}
#[cfg(feature = "frontend")]
impl ContainerVerticalRule {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("container_vertical_rules")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, container_item_type_id, contained_item_type_id, minimum_temperature, maximum_temperature, minimum_humidity, maximum_humidity, minimum_pressure, maximum_pressure")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            container_item_type_id: match row.get("container_item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(container_item_type_id) => {
                    Some(Uuid::from_u128(*container_item_type_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            contained_item_type_id: match row.get("contained_item_type_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(contained_item_type_id) => {
                    Some(Uuid::from_u128(*contained_item_type_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            minimum_temperature: match row.get("minimum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_temperature) => {
                    Some(minimum_temperature.clone())
                }
                _ => unreachable!("Expected F64"),
            },
            maximum_temperature: match row.get("maximum_temperature").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_temperature) => {
                    Some(maximum_temperature.clone())
                }
                _ => unreachable!("Expected F64"),
            },
            minimum_humidity: match row.get("minimum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_humidity) => Some(minimum_humidity.clone()),
                _ => unreachable!("Expected F64"),
            },
            maximum_humidity: match row.get("maximum_humidity").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_humidity) => Some(maximum_humidity.clone()),
                _ => unreachable!("Expected F64"),
            },
            minimum_pressure: match row.get("minimum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(minimum_pressure) => Some(minimum_pressure.clone()),
                _ => unreachable!("Expected F64"),
            },
            maximum_pressure: match row.get("maximum_pressure").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(maximum_pressure) => Some(maximum_pressure.clone()),
                _ => unreachable!("Expected F64"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContinuousUnit {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl ContinuousUnit {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("continuous_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Describable {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
#[cfg(feature = "frontend")]
impl Describable {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("describables")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(description) => Some(description.clone()),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscreteUnit {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl DiscreteUnit {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("discrete_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DocumentFormat {
    pub id: Uuid,
    pub mime_type: String,
}
#[cfg(feature = "frontend")]
impl DocumentFormat {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("document_formats")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, mime_type")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            mime_type: match row.get("mime_type").unwrap() {
                gluesql::prelude::Value::Str(mime_type) => mime_type.clone(),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Document {
    pub id: Uuid,
    pub path: String,
    pub format_id: Uuid,
    pub bytes: i32,
}
#[cfg(feature = "frontend")]
impl Document {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("documents")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, path, format_id, bytes")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            path: match row.get("path").unwrap() {
                gluesql::prelude::Value::Str(path) => path.clone(),
                _ => unreachable!("Expected Str"),
            },
            format_id: match row.get("format_id").unwrap() {
                gluesql::prelude::Value::Uuid(format_id) => Uuid::from_u128(*format_id),
                _ => unreachable!("Expected Uuid"),
            },
            bytes: match row.get("bytes").unwrap() {
                gluesql::prelude::Value::I32(bytes) => bytes.clone(),
                _ => unreachable!("Expected I32"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Editable {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub created_by: Uuid,
}
#[cfg(feature = "frontend")]
impl Editable {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("editables")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, created_at, created_by")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::Uuid(created_by) => Uuid::from_u128(*created_by),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Edit {
    pub id: Uuid,
    pub editable_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Edit {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("edits")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, editable_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemCategory {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemCategory {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemCategoryRelationship {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub child_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemCategoryRelationship {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_category_relationships")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_id, child_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_id: match row.get("parent_id").unwrap() {
                gluesql::prelude::Value::Uuid(parent_id) => Uuid::from_u128(*parent_id),
                _ => unreachable!("Expected Uuid"),
            },
            child_id: match row.get("child_id").unwrap() {
                gluesql::prelude::Value::Uuid(child_id) => Uuid::from_u128(*child_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemCategoryUnit {
    pub id: Uuid,
    pub item_category_id: Uuid,
    pub unit_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemCategoryUnit {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_category_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_category_id, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_category_id) => {
                    Uuid::from_u128(*item_category_id)
                }
                _ => unreachable!("Expected Uuid"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Uuid(unit_id) => Uuid::from_u128(*unit_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemContinuousQuantity {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub weight: f64,
    pub unit_id: Option<Uuid>,
    pub sensor_id: Option<Uuid>,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ItemContinuousQuantity {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_continuous_quantities")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, weight, unit_id, sensor_id, measured_at, measured_by")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_id) => Some(Uuid::from_u128(*item_id)),
                _ => unreachable!("Expected Uuid"),
            },
            weight: match row.get("weight").unwrap() {
                gluesql::prelude::Value::F64(weight) => weight.clone(),
                _ => unreachable!("Expected F64"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
            sensor_id: match row.get("sensor_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(sensor_id) => Some(Uuid::from_u128(*sensor_id)),
                _ => unreachable!("Expected Uuid"),
            },
            measured_at: match row.get("measured_at").unwrap() {
                gluesql::prelude::Value::Timestamp(measured_at) => measured_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            measured_by: match row.get("measured_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(measured_by) => Some(Uuid::from_u128(*measured_by)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemDiscreteQuantity {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub quantity: i32,
    pub unit_id: Option<Uuid>,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ItemDiscreteQuantity {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_discrete_quantities")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, quantity, unit_id, measured_at, measured_by")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_id) => Some(Uuid::from_u128(*item_id)),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
            measured_at: match row.get("measured_at").unwrap() {
                gluesql::prelude::Value::Timestamp(measured_at) => measured_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            measured_by: match row.get("measured_by").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(measured_by) => Some(Uuid::from_u128(*measured_by)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemLocation {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub previous_location_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ItemLocation {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, location_id, previous_location_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(item_id) => Some(Uuid::from_u128(*item_id)),
                _ => unreachable!("Expected Uuid"),
            },
            location_id: match row.get("location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(location_id) => Some(Uuid::from_u128(*location_id)),
                _ => unreachable!("Expected Uuid"),
            },
            previous_location_id: match row.get("previous_location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(previous_location_id) => {
                    Some(Uuid::from_u128(*previous_location_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ItemUnit {
    pub id: Uuid,
    pub item_id: Uuid,
    pub unit_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ItemUnit {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("item_units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, item_id, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_id) => Uuid::from_u128(*item_id),
                _ => unreachable!("Expected Uuid"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Uuid(unit_id) => Uuid::from_u128(*unit_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Item {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl Item {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("items")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_id: match row.get("parent_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_id) => Some(Uuid::from_u128(*parent_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct LocationState {
    pub id: Uuid,
    pub font_awesome_icon: Option<String>,
}
#[cfg(feature = "frontend")]
impl LocationState {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("location_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, font_awesome_icon")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(font_awesome_icon) => Some(font_awesome_icon.clone()),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Location {
    pub id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub address: Option<String>,
    pub geolocalization_device_id: Option<Uuid>,
    pub altitude_device_id: Option<Uuid>,
    pub parent_location_id: Option<Uuid>,
    pub state_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Location {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, latitude, longitude, altitude, address, geolocalization_device_id, altitude_device_id, parent_location_id, state_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            latitude: match row.get("latitude").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(latitude) => Some(latitude.clone()),
                _ => unreachable!("Expected F64"),
            },
            longitude: match row.get("longitude").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(longitude) => Some(longitude.clone()),
                _ => unreachable!("Expected F64"),
            },
            altitude: match row.get("altitude").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(altitude) => Some(altitude.clone()),
                _ => unreachable!("Expected F64"),
            },
            address: match row.get("address").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(address) => Some(address.clone()),
                _ => unreachable!("Expected Str"),
            },
            geolocalization_device_id: match row.get("geolocalization_device_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(geolocalization_device_id) => {
                    Some(Uuid::from_u128(*geolocalization_device_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            altitude_device_id: match row.get("altitude_device_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(altitude_device_id) => {
                    Some(Uuid::from_u128(*altitude_device_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            parent_location_id: match row.get("parent_location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_location_id) => {
                    Some(Uuid::from_u128(*parent_location_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::Uuid(state_id) => Uuid::from_u128(*state_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct LoginProvider {
    pub id: Uuid,
    pub name: String,
    pub font_awesome_icon: String,
    pub client_id_var_name: String,
    pub redirect_uri_var_name: String,
    pub oauth_url: String,
    pub scope: String,
}
#[cfg(feature = "frontend")]
impl LoginProvider {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("login_providers")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, font_awesome_icon, client_id_var_name, redirect_uri_var_name, oauth_url, scope")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Str(font_awesome_icon) => font_awesome_icon.clone(),
                _ => unreachable!("Expected Str"),
            },
            client_id_var_name: match row.get("client_id_var_name").unwrap() {
                gluesql::prelude::Value::Str(client_id_var_name) => client_id_var_name.clone(),
                _ => unreachable!("Expected Str"),
            },
            redirect_uri_var_name: match row.get("redirect_uri_var_name").unwrap() {
                gluesql::prelude::Value::Str(redirect_uri_var_name) => {
                    redirect_uri_var_name.clone()
                }
                _ => unreachable!("Expected Str"),
            },
            oauth_url: match row.get("oauth_url").unwrap() {
                gluesql::prelude::Value::Str(oauth_url) => oauth_url.clone(),
                _ => unreachable!("Expected Str"),
            },
            scope: match row.get("scope").unwrap() {
                gluesql::prelude::Value::Str(scope) => scope.clone(),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ManufacturedItemCategory {
    pub id: Uuid,
    pub cost: f64,
    pub cost_per_day: f64,
    pub currency: String,
    pub manifacturer_id: Uuid,
}
#[cfg(feature = "frontend")]
impl ManufacturedItemCategory {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("manufactured_item_categories")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, cost, cost_per_day, currency, manifacturer_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            cost: match row.get("cost").unwrap() {
                gluesql::prelude::Value::F64(cost) => cost.clone(),
                _ => unreachable!("Expected F64"),
            },
            cost_per_day: match row.get("cost_per_day").unwrap() {
                gluesql::prelude::Value::F64(cost_per_day) => cost_per_day.clone(),
                _ => unreachable!("Expected F64"),
            },
            currency: match row.get("currency").unwrap() {
                gluesql::prelude::Value::Str(currency) => currency.clone(),
                _ => unreachable!("Expected Str"),
            },
            manifacturer_id: match row.get("manifacturer_id").unwrap() {
                gluesql::prelude::Value::Uuid(manifacturer_id) => Uuid::from_u128(*manifacturer_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub operation: String,
    pub table_name: String,
    pub row_id: Option<Uuid>,
    pub read: bool,
}
#[cfg(feature = "frontend")]
impl Notification {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, user_id, operation, table_name, row_id, read")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::Uuid(user_id) => Uuid::from_u128(*user_id),
                _ => unreachable!("Expected Uuid"),
            },
            operation: match row.get("operation").unwrap() {
                gluesql::prelude::Value::Str(operation) => operation.clone(),
                _ => unreachable!("Expected Str"),
            },
            table_name: match row.get("table_name").unwrap() {
                gluesql::prelude::Value::Str(table_name) => table_name.clone(),
                _ => unreachable!("Expected Str"),
            },
            row_id: match row.get("row_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(row_id) => Some(Uuid::from_u128(*row_id)),
                _ => unreachable!("Expected Uuid"),
            },
            read: match row.get("read").unwrap() {
                gluesql::prelude::Value::Bool(read) => read.clone(),
                _ => unreachable!("Expected Bool"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationAuthorization {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[cfg(feature = "frontend")]
impl OrganizationAuthorization {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_authorizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, organization_id, editable_id, role_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            organization_id: match row.get("organization_id").unwrap() {
                gluesql::prelude::Value::Uuid(organization_id) => Uuid::from_u128(*organization_id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::Uuid(role_id) => Uuid::from_u128(*role_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationLocation {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub previous_location_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl OrganizationLocation {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_locations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, organization_id, location_id, previous_location_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            organization_id: match row.get("organization_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(organization_id) => {
                    Some(Uuid::from_u128(*organization_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            location_id: match row.get("location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(location_id) => Some(Uuid::from_u128(*location_id)),
                _ => unreachable!("Expected Uuid"),
            },
            previous_location_id: match row.get("previous_location_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(previous_location_id) => {
                    Some(Uuid::from_u128(*previous_location_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationState {
    pub id: Uuid,
    pub font_awesome_icon: Option<String>,
}
#[cfg(feature = "frontend")]
impl OrganizationState {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organization_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, font_awesome_icon")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(font_awesome_icon) => Some(font_awesome_icon.clone()),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Organization {
    pub id: Uuid,
    pub state_id: Option<Uuid>,
    pub parent_organization_id: Option<Uuid>,
    pub logo_id: Option<Uuid>,
    pub website_url: Option<String>,
}
#[cfg(feature = "frontend")]
impl Organization {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("organizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, state_id, parent_organization_id, logo_id, website_url")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(state_id) => Some(Uuid::from_u128(*state_id)),
                _ => unreachable!("Expected Uuid"),
            },
            parent_organization_id: match row.get("parent_organization_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_organization_id) => {
                    Some(Uuid::from_u128(*parent_organization_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            logo_id: match row.get("logo_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(logo_id) => Some(Uuid::from_u128(*logo_id)),
                _ => unreachable!("Expected Uuid"),
            },
            website_url: match row.get("website_url").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(website_url) => Some(website_url.clone()),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PrimaryUserEmail {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl PrimaryUserEmail {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("primary_user_emails")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProcedureContinuousRequirement {
    pub id: Uuid,
    pub procedure_id: Uuid,
    pub item_category_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProcedureContinuousRequirement {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_continuous_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, procedure_id, item_category_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            procedure_id: match row.get("procedure_id").unwrap() {
                gluesql::prelude::Value::Uuid(procedure_id) => Uuid::from_u128(*procedure_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_category_id) => {
                    Uuid::from_u128(*item_category_id)
                }
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::F64(quantity) => quantity.clone(),
                _ => unreachable!("Expected F64"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProcedureDiscreteRequirement {
    pub id: Uuid,
    pub procedure_id: Uuid,
    pub item_category_id: Uuid,
    pub quantity: i32,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProcedureDiscreteRequirement {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedure_discrete_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, procedure_id, item_category_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            procedure_id: match row.get("procedure_id").unwrap() {
                gluesql::prelude::Value::Uuid(procedure_id) => Uuid::from_u128(*procedure_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_category_id: match row.get("item_category_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_category_id) => {
                    Uuid::from_u128(*item_category_id)
                }
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::I32(quantity) => quantity.clone(),
                _ => unreachable!("Expected I32"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Procedure {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl Procedure {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("procedures")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectContinuousRequirement {
    pub id: Uuid,
    pub project_id: Uuid,
    pub item_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProjectContinuousRequirement {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_continuous_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, project_id, item_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::Uuid(project_id) => Uuid::from_u128(*project_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_id) => Uuid::from_u128(*item_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::F64(quantity) => quantity.clone(),
                _ => unreachable!("Expected F64"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectDiscreteRequirement {
    pub id: Uuid,
    pub project_id: Uuid,
    pub item_id: Uuid,
    pub quantity: f64,
    pub unit_id: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl ProjectDiscreteRequirement {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_discrete_requirements")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, project_id, item_id, quantity, unit_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::Uuid(project_id) => Uuid::from_u128(*project_id),
                _ => unreachable!("Expected Uuid"),
            },
            item_id: match row.get("item_id").unwrap() {
                gluesql::prelude::Value::Uuid(item_id) => Uuid::from_u128(*item_id),
                _ => unreachable!("Expected Uuid"),
            },
            quantity: match row.get("quantity").unwrap() {
                gluesql::prelude::Value::F64(quantity) => quantity.clone(),
                _ => unreachable!("Expected F64"),
            },
            unit_id: match row.get("unit_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(unit_id) => Some(Uuid::from_u128(*unit_id)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectMilestone {
    pub id: Uuid,
    pub project_id: Uuid,
    pub due_date: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}
#[cfg(feature = "frontend")]
impl ProjectMilestone {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_milestones")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, project_id, due_date, completed_at")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            project_id: match row.get("project_id").unwrap() {
                gluesql::prelude::Value::Uuid(project_id) => Uuid::from_u128(*project_id),
                _ => unreachable!("Expected Uuid"),
            },
            due_date: match row.get("due_date").unwrap() {
                gluesql::prelude::Value::Timestamp(due_date) => due_date.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            completed_at: match row.get("completed_at").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(completed_at) => Some(completed_at.clone()),
                _ => unreachable!("Expected Timestamp"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectState {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub font_awesome_icon: String,
    pub icon_color: String,
}
#[cfg(feature = "frontend")]
impl ProjectState {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("project_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, font_awesome_icon, icon_color")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Str(font_awesome_icon) => font_awesome_icon.clone(),
                _ => unreachable!("Expected Str"),
            },
            icon_color: match row.get("icon_color").unwrap() {
                gluesql::prelude::Value::Str(icon_color) => icon_color.clone(),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: Uuid,
    pub parent_project_id: Option<Uuid>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub currency: Option<String>,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}
#[cfg(feature = "frontend")]
impl Project {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("projects")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, public, state_id, parent_project_id, budget, expenses, currency, created_by, created_at, expected_end_date, end_date")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str"),
            },
            public: match row.get("public").unwrap() {
                gluesql::prelude::Value::Bool(public) => public.clone(),
                _ => unreachable!("Expected Bool"),
            },
            state_id: match row.get("state_id").unwrap() {
                gluesql::prelude::Value::Uuid(state_id) => Uuid::from_u128(*state_id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_project_id: match row.get("parent_project_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_project_id) => {
                    Some(Uuid::from_u128(*parent_project_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            budget: match row.get("budget").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(budget) => Some(budget.clone()),
                _ => unreachable!("Expected F64"),
            },
            expenses: match row.get("expenses").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::F64(expenses) => Some(expenses.clone()),
                _ => unreachable!("Expected F64"),
            },
            currency: match row.get("currency").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(currency) => Some(currency.clone()),
                _ => unreachable!("Expected Str"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::Uuid(created_by) => Uuid::from_u128(*created_by),
                _ => unreachable!("Expected Uuid"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            expected_end_date: match row.get("expected_end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(expected_end_date) => {
                    Some(expected_end_date.clone())
                }
                _ => unreachable!("Expected Timestamp"),
            },
            end_date: match row.get("end_date").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Timestamp(end_date) => Some(end_date.clone()),
                _ => unreachable!("Expected Timestamp"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Role {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl Role {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("roles")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SampleTaxa {
    pub id: Uuid,
    pub sample_id: Uuid,
    pub taxon_id: Uuid,
}
#[cfg(feature = "frontend")]
impl SampleTaxa {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sample_taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, sample_id, taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::Uuid(taxon_id) => Uuid::from_u128(*taxon_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SampledIndividualTaxa {
    pub id: Uuid,
    pub sampled_individual_id: Uuid,
    pub taxon_id: Uuid,
}
#[cfg(feature = "frontend")]
impl SampledIndividualTaxa {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individual_taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, sampled_individual_id, taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            sampled_individual_id: match row.get("sampled_individual_id").unwrap() {
                gluesql::prelude::Value::Uuid(sampled_individual_id) => {
                    Uuid::from_u128(*sampled_individual_id)
                }
                _ => unreachable!("Expected Uuid"),
            },
            taxon_id: match row.get("taxon_id").unwrap() {
                gluesql::prelude::Value::Uuid(taxon_id) => Uuid::from_u128(*taxon_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SampledIndividual {
    pub id: Uuid,
}
#[cfg(feature = "frontend")]
impl SampledIndividual {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("sampled_individuals")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Sample {
    pub id: Uuid,
    pub derived_from: Option<Uuid>,
}
#[cfg(feature = "frontend")]
impl Sample {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("samples")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, derived_from")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            derived_from: match row.get("derived_from").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(derived_from) => Some(Uuid::from_u128(*derived_from)),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Spectra {
    pub id: Uuid,
    pub spectra_collection_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Spectra {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, spectra_collection_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            spectra_collection_id: match row.get("spectra_collection_id").unwrap() {
                gluesql::prelude::Value::Uuid(spectra_collection_id) => {
                    Uuid::from_u128(*spectra_collection_id)
                }
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SpectraCollection {
    pub id: Uuid,
    pub sample_id: Uuid,
}
#[cfg(feature = "frontend")]
impl SpectraCollection {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("spectra_collection")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, sample_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            sample_id: match row.get("sample_id").unwrap() {
                gluesql::prelude::Value::Uuid(sample_id) => Uuid::from_u128(*sample_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Taxa {
    pub id: Uuid,
    pub name: String,
    pub ncbi_taxon_id: Option<i32>,
}
#[cfg(feature = "frontend")]
impl Taxa {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("taxa")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, ncbi_taxon_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            ncbi_taxon_id: match row.get("ncbi_taxon_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::I32(ncbi_taxon_id) => Some(ncbi_taxon_id.clone()),
                _ => unreachable!("Expected I32"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamAuthorization {
    pub id: Uuid,
    pub team_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[cfg(feature = "frontend")]
impl TeamAuthorization {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_authorizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, team_id, editable_id, role_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            team_id: match row.get("team_id").unwrap() {
                gluesql::prelude::Value::Uuid(team_id) => Uuid::from_u128(*team_id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::Uuid(role_id) => Uuid::from_u128(*role_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamState {
    pub id: Uuid,
    pub font_awesome_icon: String,
}
#[cfg(feature = "frontend")]
impl TeamState {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, font_awesome_icon")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            font_awesome_icon: match row.get("font_awesome_icon").unwrap() {
                gluesql::prelude::Value::Str(font_awesome_icon) => font_awesome_icon.clone(),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Team {
    pub id: Uuid,
    pub parent_team_id: Option<Uuid>,
    pub team_state_id: Uuid,
}
#[cfg(feature = "frontend")]
impl Team {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("teams")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, parent_team_id, team_state_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            parent_team_id: match row.get("parent_team_id").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Uuid(parent_team_id) => {
                    Some(Uuid::from_u128(*parent_team_id))
                }
                _ => unreachable!("Expected Uuid"),
            },
            team_state_id: match row.get("team_state_id").unwrap() {
                gluesql::prelude::Value::Uuid(team_state_id) => Uuid::from_u128(*team_state_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Unit {
    pub id: Uuid,
    pub symbol: String,
}
#[cfg(feature = "frontend")]
impl Unit {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("units")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, symbol")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            symbol: match row.get("symbol").unwrap() {
                gluesql::prelude::Value::Str(symbol) => symbol.clone(),
                _ => unreachable!("Expected Str"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserAuthorization {
    pub id: Uuid,
    pub user_id: Uuid,
    pub editable_id: Uuid,
    pub role_id: Uuid,
}
#[cfg(feature = "frontend")]
impl UserAuthorization {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_authorizations")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, user_id, editable_id, role_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::Uuid(user_id) => Uuid::from_u128(*user_id),
                _ => unreachable!("Expected Uuid"),
            },
            editable_id: match row.get("editable_id").unwrap() {
                gluesql::prelude::Value::Uuid(editable_id) => Uuid::from_u128(*editable_id),
                _ => unreachable!("Expected Uuid"),
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::Uuid(role_id) => Uuid::from_u128(*role_id),
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct UserEmail {
    pub id: Uuid,
    pub email: String,
    pub user_id: Uuid,
    pub login_provider_id: Uuid,
}
#[cfg(feature = "frontend")]
impl UserEmail {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("user_emails")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, email, user_id, login_provider_id")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            email: match row.get("email").unwrap() {
                gluesql::prelude::Value::Str(email) => email.clone(),
                _ => unreachable!("Expected Str"),
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::Uuid(user_id) => Uuid::from_u128(*user_id),
                _ => unreachable!("Expected Uuid"),
            },
            login_provider_id: match row.get("login_provider_id").unwrap() {
                gluesql::prelude::Value::Uuid(login_provider_id) => {
                    Uuid::from_u128(*login_provider_id)
                }
                _ => unreachable!("Expected Uuid"),
            },
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[cfg(feature = "frontend")]
impl User {
    pub async fn get<C>(
        id: Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, gluesql::prelude::Error>
    where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        use gluesql::core::ast_builder::*;
        let select_row = table("users")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, first_name, middle_name, last_name, created_at, updated_at")
            .limit(1)
            .execute(connection)
            .await?;
        let row = select_row.select().unwrap().collect::<Vec<_>>().pop();
        Ok(row.map(|row| Self::from_row(row)))
    }

    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::Uuid(id) => Uuid::from_u128(*id),
                _ => unreachable!("Expected Uuid"),
            },
            first_name: match row.get("first_name").unwrap() {
                gluesql::prelude::Value::Str(first_name) => first_name.clone(),
                _ => unreachable!("Expected Str"),
            },
            middle_name: match row.get("middle_name").unwrap() {
                gluesql::prelude::Value::Null => None,
                gluesql::prelude::Value::Str(middle_name) => Some(middle_name.clone()),
                _ => unreachable!("Expected Str"),
            },
            last_name: match row.get("last_name").unwrap() {
                gluesql::prelude::Value::Str(last_name) => last_name.clone(),
                _ => unreachable!("Expected Str"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
            updated_at: match row.get("updated_at").unwrap() {
                gluesql::prelude::Value::Timestamp(updated_at) => updated_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq)]
pub enum Table {
    Archivable,
    ContainerHorizontalRule,
    ContainerVerticalRule,
    ContinuousUnit,
    Describable,
    DiscreteUnit,
    DocumentFormat,
    Document,
    Editable,
    Edit,
    ItemCategory,
    ItemCategoryRelationship,
    ItemCategoryUnit,
    ItemContinuousQuantity,
    ItemDiscreteQuantity,
    ItemLocation,
    ItemUnit,
    Item,
    LocationState,
    Location,
    LoginProvider,
    ManufacturedItemCategory,
    Notification,
    OrganizationAuthorization,
    OrganizationLocation,
    OrganizationState,
    Organization,
    PrimaryUserEmail,
    ProcedureContinuousRequirement,
    ProcedureDiscreteRequirement,
    Procedure,
    ProjectContinuousRequirement,
    ProjectDiscreteRequirement,
    ProjectMilestone,
    ProjectState,
    Project,
    Role,
    SampleTaxa,
    SampledIndividualTaxa,
    SampledIndividual,
    Sample,
    Spectra,
    SpectraCollection,
    Taxa,
    TeamAuthorization,
    TeamState,
    Team,
    Unit,
    UserAuthorization,
    UserEmail,
    User,
}

impl Table {
    pub fn name(&self) -> &'static str {
        match self {
            Table::Archivable => "archivables",
            Table::ContainerHorizontalRule => "container_horizontal_rules",
            Table::ContainerVerticalRule => "container_vertical_rules",
            Table::ContinuousUnit => "continuous_units",
            Table::Describable => "describables",
            Table::DiscreteUnit => "discrete_units",
            Table::DocumentFormat => "document_formats",
            Table::Document => "documents",
            Table::Editable => "editables",
            Table::Edit => "edits",
            Table::ItemCategory => "item_categories",
            Table::ItemCategoryRelationship => "item_category_relationships",
            Table::ItemCategoryUnit => "item_category_units",
            Table::ItemContinuousQuantity => "item_continuous_quantities",
            Table::ItemDiscreteQuantity => "item_discrete_quantities",
            Table::ItemLocation => "item_locations",
            Table::ItemUnit => "item_units",
            Table::Item => "items",
            Table::LocationState => "location_states",
            Table::Location => "locations",
            Table::LoginProvider => "login_providers",
            Table::ManufacturedItemCategory => "manufactured_item_categories",
            Table::Notification => "notifications",
            Table::OrganizationAuthorization => "organization_authorizations",
            Table::OrganizationLocation => "organization_locations",
            Table::OrganizationState => "organization_states",
            Table::Organization => "organizations",
            Table::PrimaryUserEmail => "primary_user_emails",
            Table::ProcedureContinuousRequirement => "procedure_continuous_requirements",
            Table::ProcedureDiscreteRequirement => "procedure_discrete_requirements",
            Table::Procedure => "procedures",
            Table::ProjectContinuousRequirement => "project_continuous_requirements",
            Table::ProjectDiscreteRequirement => "project_discrete_requirements",
            Table::ProjectMilestone => "project_milestones",
            Table::ProjectState => "project_states",
            Table::Project => "projects",
            Table::Role => "roles",
            Table::SampleTaxa => "sample_taxa",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::SampledIndividual => "sampled_individuals",
            Table::Sample => "samples",
            Table::Spectra => "spectra",
            Table::SpectraCollection => "spectra_collection",
            Table::Taxa => "taxa",
            Table::TeamAuthorization => "team_authorizations",
            Table::TeamState => "team_states",
            Table::Team => "teams",
            Table::Unit => "units",
            Table::UserAuthorization => "user_authorizations",
            Table::UserEmail => "user_emails",
            Table::User => "users",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum TableRow {
    Archivable(Archivable),
    ContainerHorizontalRule(ContainerHorizontalRule),
    ContainerVerticalRule(ContainerVerticalRule),
    ContinuousUnit(ContinuousUnit),
    Describable(Describable),
    DiscreteUnit(DiscreteUnit),
    DocumentFormat(DocumentFormat),
    Document(Document),
    Editable(Editable),
    Edit(Edit),
    ItemCategory(ItemCategory),
    ItemCategoryRelationship(ItemCategoryRelationship),
    ItemCategoryUnit(ItemCategoryUnit),
    ItemContinuousQuantity(ItemContinuousQuantity),
    ItemDiscreteQuantity(ItemDiscreteQuantity),
    ItemLocation(ItemLocation),
    ItemUnit(ItemUnit),
    Item(Item),
    LocationState(LocationState),
    Location(Location),
    LoginProvider(LoginProvider),
    ManufacturedItemCategory(ManufacturedItemCategory),
    Notification(Notification),
    OrganizationAuthorization(OrganizationAuthorization),
    OrganizationLocation(OrganizationLocation),
    OrganizationState(OrganizationState),
    Organization(Organization),
    PrimaryUserEmail(PrimaryUserEmail),
    ProcedureContinuousRequirement(ProcedureContinuousRequirement),
    ProcedureDiscreteRequirement(ProcedureDiscreteRequirement),
    Procedure(Procedure),
    ProjectContinuousRequirement(ProjectContinuousRequirement),
    ProjectDiscreteRequirement(ProjectDiscreteRequirement),
    ProjectMilestone(ProjectMilestone),
    ProjectState(ProjectState),
    Project(Project),
    Role(Role),
    SampleTaxa(SampleTaxa),
    SampledIndividualTaxa(SampledIndividualTaxa),
    SampledIndividual(SampledIndividual),
    Sample(Sample),
    Spectra(Spectra),
    SpectraCollection(SpectraCollection),
    Taxa(Taxa),
    TeamAuthorization(TeamAuthorization),
    TeamState(TeamState),
    Team(Team),
    Unit(Unit),
    UserAuthorization(UserAuthorization),
    UserEmail(UserEmail),
    User(User),
}

impl From<&str> for Table {
    fn from(item: &str) -> Self {
        match item {
            "archivables" => Table::Archivable,
            "container_horizontal_rules" => Table::ContainerHorizontalRule,
            "container_vertical_rules" => Table::ContainerVerticalRule,
            "continuous_units" => Table::ContinuousUnit,
            "describables" => Table::Describable,
            "discrete_units" => Table::DiscreteUnit,
            "document_formats" => Table::DocumentFormat,
            "documents" => Table::Document,
            "editables" => Table::Editable,
            "edits" => Table::Edit,
            "item_categories" => Table::ItemCategory,
            "item_category_relationships" => Table::ItemCategoryRelationship,
            "item_category_units" => Table::ItemCategoryUnit,
            "item_continuous_quantities" => Table::ItemContinuousQuantity,
            "item_discrete_quantities" => Table::ItemDiscreteQuantity,
            "item_locations" => Table::ItemLocation,
            "item_units" => Table::ItemUnit,
            "items" => Table::Item,
            "location_states" => Table::LocationState,
            "locations" => Table::Location,
            "login_providers" => Table::LoginProvider,
            "manufactured_item_categories" => Table::ManufacturedItemCategory,
            "notifications" => Table::Notification,
            "organization_authorizations" => Table::OrganizationAuthorization,
            "organization_locations" => Table::OrganizationLocation,
            "organization_states" => Table::OrganizationState,
            "organizations" => Table::Organization,
            "primary_user_emails" => Table::PrimaryUserEmail,
            "procedure_continuous_requirements" => Table::ProcedureContinuousRequirement,
            "procedure_discrete_requirements" => Table::ProcedureDiscreteRequirement,
            "procedures" => Table::Procedure,
            "project_continuous_requirements" => Table::ProjectContinuousRequirement,
            "project_discrete_requirements" => Table::ProjectDiscreteRequirement,
            "project_milestones" => Table::ProjectMilestone,
            "project_states" => Table::ProjectState,
            "projects" => Table::Project,
            "roles" => Table::Role,
            "sample_taxa" => Table::SampleTaxa,
            "sampled_individual_taxa" => Table::SampledIndividualTaxa,
            "sampled_individuals" => Table::SampledIndividual,
            "samples" => Table::Sample,
            "spectra" => Table::Spectra,
            "spectra_collection" => Table::SpectraCollection,
            "taxa" => Table::Taxa,
            "team_authorizations" => Table::TeamAuthorization,
            "team_states" => Table::TeamState,
            "teams" => Table::Team,
            "units" => Table::Unit,
            "user_authorizations" => Table::UserAuthorization,
            "user_emails" => Table::UserEmail,
            "users" => Table::User,
            _ => panic!("Unknown table name"),
        }
    }
}
impl TableRow {
    pub fn table(&self) -> &'static Table {
        match self {
            TableRow::Archivable(_) => &Table::Archivable,
            TableRow::ContainerHorizontalRule(_) => &Table::ContainerHorizontalRule,
            TableRow::ContainerVerticalRule(_) => &Table::ContainerVerticalRule,
            TableRow::ContinuousUnit(_) => &Table::ContinuousUnit,
            TableRow::Describable(_) => &Table::Describable,
            TableRow::DiscreteUnit(_) => &Table::DiscreteUnit,
            TableRow::DocumentFormat(_) => &Table::DocumentFormat,
            TableRow::Document(_) => &Table::Document,
            TableRow::Editable(_) => &Table::Editable,
            TableRow::Edit(_) => &Table::Edit,
            TableRow::ItemCategory(_) => &Table::ItemCategory,
            TableRow::ItemCategoryRelationship(_) => &Table::ItemCategoryRelationship,
            TableRow::ItemCategoryUnit(_) => &Table::ItemCategoryUnit,
            TableRow::ItemContinuousQuantity(_) => &Table::ItemContinuousQuantity,
            TableRow::ItemDiscreteQuantity(_) => &Table::ItemDiscreteQuantity,
            TableRow::ItemLocation(_) => &Table::ItemLocation,
            TableRow::ItemUnit(_) => &Table::ItemUnit,
            TableRow::Item(_) => &Table::Item,
            TableRow::LocationState(_) => &Table::LocationState,
            TableRow::Location(_) => &Table::Location,
            TableRow::LoginProvider(_) => &Table::LoginProvider,
            TableRow::ManufacturedItemCategory(_) => &Table::ManufacturedItemCategory,
            TableRow::Notification(_) => &Table::Notification,
            TableRow::OrganizationAuthorization(_) => &Table::OrganizationAuthorization,
            TableRow::OrganizationLocation(_) => &Table::OrganizationLocation,
            TableRow::OrganizationState(_) => &Table::OrganizationState,
            TableRow::Organization(_) => &Table::Organization,
            TableRow::PrimaryUserEmail(_) => &Table::PrimaryUserEmail,
            TableRow::ProcedureContinuousRequirement(_) => &Table::ProcedureContinuousRequirement,
            TableRow::ProcedureDiscreteRequirement(_) => &Table::ProcedureDiscreteRequirement,
            TableRow::Procedure(_) => &Table::Procedure,
            TableRow::ProjectContinuousRequirement(_) => &Table::ProjectContinuousRequirement,
            TableRow::ProjectDiscreteRequirement(_) => &Table::ProjectDiscreteRequirement,
            TableRow::ProjectMilestone(_) => &Table::ProjectMilestone,
            TableRow::ProjectState(_) => &Table::ProjectState,
            TableRow::Project(_) => &Table::Project,
            TableRow::Role(_) => &Table::Role,
            TableRow::SampleTaxa(_) => &Table::SampleTaxa,
            TableRow::SampledIndividualTaxa(_) => &Table::SampledIndividualTaxa,
            TableRow::SampledIndividual(_) => &Table::SampledIndividual,
            TableRow::Sample(_) => &Table::Sample,
            TableRow::Spectra(_) => &Table::Spectra,
            TableRow::SpectraCollection(_) => &Table::SpectraCollection,
            TableRow::Taxa(_) => &Table::Taxa,
            TableRow::TeamAuthorization(_) => &Table::TeamAuthorization,
            TableRow::TeamState(_) => &Table::TeamState,
            TableRow::Team(_) => &Table::Team,
            TableRow::Unit(_) => &Table::Unit,
            TableRow::UserAuthorization(_) => &Table::UserAuthorization,
            TableRow::UserEmail(_) => &Table::UserEmail,
            TableRow::User(_) => &Table::User,
        }
    }

    pub fn table_name(&self) -> &'static str {
        self.table().name()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq)]
pub enum SearcheableTable {
    ProjectState,
    Project,
    Taxa,
    User,
}
