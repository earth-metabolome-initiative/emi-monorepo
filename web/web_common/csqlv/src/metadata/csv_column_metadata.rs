//! Submodule providing the CSV Columns struct and associated functions.
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use crate::{CSVSchema, data_types::DataType, errors::CSVSchemaError};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
/// Struct representing a CSV column.
pub struct CSVColumnMetadata {
    pub(crate) name: Option<String>,
    pub(crate) foreign_table_name: Option<String>,
    pub(crate) foreign_column_name: Option<String>,
    pub(crate) data_type: DataType,
    pub(crate) nullable: bool,
    pub(crate) artificial: bool,
    pub(crate) primary_key: bool,
    pub(crate) unique: bool,
}

impl CSVColumnMetadata {
    /// Create a new primary key column.
    ///
    /// # Errors
    /// * If the data type cannot be converted to a serial data type.
    pub(crate) fn new_primary_key(data_type: DataType) -> Result<Self, CSVSchemaError> {
        Ok(Self {
            name: Some("id".to_string()),
            foreign_table_name: None,
            foreign_column_name: None,
            data_type: data_type.into_serial()?,
            nullable: false,
            artificial: true,
            primary_key: true,
            unique: false,
        })
    }

    pub(crate) fn name(&self, schema: &CSVSchema) -> Result<String, CSVSchemaError> {
        Ok(if let Some(name) = &self.name {
            name.clone()
        } else {
            let foreign_table_name = self.foreign_table_name.as_ref().unwrap();
            let foreign_table = schema.table_from_name(foreign_table_name)?;
            format!("{}_id", foreign_table.foreign_table_name())
        })
    }
}

impl TryFrom<CSVColumnMetadataBuilder> for CSVColumnMetadata {
    type Error = CSVSchemaError;

    fn try_from(mut builder: CSVColumnMetadataBuilder) -> Result<Self, Self::Error> {
        if builder.data_type_counts.is_empty() {
            return Err(CSVSchemaError::EmptyColumn);
        }

        let Some(mut data_type) = builder
            .data_type_counts
            .into_iter()
            .filter(|(data_type, _)| !data_type.is_null())
            .max_by_key(|(data_type, count)| (*count, Reverse(data_type.min_dimension())))
            .map(|(data_type, _)| data_type)
        else {
            return Err(CSVSchemaError::EmptyColumn);
        };

        if builder.sequential {
            data_type = data_type.into_serial()?;
            builder.primary_key = true;
        } else if builder.unique && !builder.nullable && data_type.is_key_like() {
            builder.primary_key = true;
        }

        if builder.primary_key && !builder.unique {
            return Err(CSVSchemaError::NonUniquePrimaryKey {
                column_name: builder.column_name.unwrap_or_default(),
                table_name: None
            });
        }

        Ok(CSVColumnMetadata {
            name: builder.column_name,
            foreign_table_name: builder.foreign_table_name,
            foreign_column_name: builder.foreign_column_name,
            data_type,
            artificial: false,
            nullable: builder.nullable,
            primary_key: builder.primary_key,
            unique: builder.unique,
        })
    }
}

#[allow(clippy::struct_excessive_bools)]
/// Struct representing a CSV column builder.
pub struct CSVColumnMetadataBuilder {
    pub(crate) column_name: Option<String>,
    pub(crate) foreign_table_name: Option<String>,
    pub(crate) foreign_column_name: Option<String>,
    pub(crate) data_type_counts: HashMap<DataType, u64>,
    pub(crate) nullable: bool,
    pub(crate) primary_key: bool,
    pub(crate) unique: bool,
    pub(crate) sequential: bool,
    pub(crate) last_value: Option<u64>,
    pub(crate) unique_values: Option<HashSet<String>>,
}

impl CSVColumnMetadataBuilder {
    /// Create a new `CSVColumnMetadataBuilder`.
    pub fn new(mut original_name: &str) -> Result<Self, CSVSchemaError> {
        if original_name.is_empty() {
            return Err(CSVSchemaError::InvalidColumnName(
                "Column name cannot be empty".to_string(),
            ));
        }

        if original_name.contains(' ') {
            return Err(CSVSchemaError::InvalidColumnName(format!(
                "Column name '{original_name}' cannot contain spaces"
            )));
        }

        // The expected original name syntax is
        // "column_name:foreign_table_name.foreign_column_name" or "column_name"
        // if there is no foreign key, or "foreign_table_name.foreign_column_name"
        // if the expected column name is '{foreign_table_name}_id'.

        let mut foreign_table_name = None;
        let mut foreign_column_name = None;
        let mut is_primary_key = false;

        if let Some(stripped_original_name) = original_name.strip_prefix("pk::") {
            is_primary_key = true;
            original_name = stripped_original_name;
        }

        let mut column_name = Some(original_name.to_owned());

        if original_name.contains(':') {
            let parts: Vec<&str> = original_name.split(':').collect();
            if parts.len() != 2 {
                return Err(CSVSchemaError::InvalidColumnName(
                    "Invalid column name syntax".to_string(),
                ));
            }
            column_name = Some(parts[0].to_string());
            let foreign_parts: Vec<&str> = parts[1].split('.').collect();
            if foreign_parts.len() != 2 {
                return Err(CSVSchemaError::InvalidColumnName(
                    "Invalid column name syntax".to_string(),
                ));
            }
            foreign_table_name = Some(foreign_parts[0].to_string());
            foreign_column_name = Some(foreign_parts[1].to_string());
        } else if original_name.contains('.') {
            let parts: Vec<&str> = original_name.split('.').collect();
            if parts.len() != 2 {
                return Err(CSVSchemaError::InvalidColumnName(
                    "Invalid column name syntax".to_string(),
                ));
            }
            foreign_table_name = Some(parts[0].to_string());
            foreign_column_name = Some(parts[1].to_string());
            column_name = None;
        }

        Ok(CSVColumnMetadataBuilder {
            column_name,
            foreign_table_name,
            foreign_column_name,
            data_type_counts: HashMap::new(),
            nullable: false,
            primary_key: is_primary_key,
            unique: true,
            unique_values: Some(HashSet::new()),
            sequential: true,
            last_value: Some(0),
        })
    }

    /// Update the column builder with a new value.
    pub fn update(&mut self, value: &str) {
        for data_type in DataType::from_value(value) {
            if data_type.is_null() {
                self.nullable = true;
            }

            *self.data_type_counts.entry(data_type).or_insert(0) += 1;
        }
        if self.unique_values.is_some()
            && !self.unique_values.as_mut().unwrap().insert(value.to_string())
        {
            // If we find a duplicate value, we disable the unique constraint and
            // free the memory used to store the unique values.
            self.unique = false;
            self.unique_values = None;
        }
        if let Some(last_value) = self.last_value {
            if let Ok(value) = value.parse::<u64>() {
                if value == last_value + 1 {
                    self.last_value = Some(value);
                } else {
                    self.sequential = false;
                    self.last_value = None;
                }
            } else {
                self.sequential = false;
                self.last_value = None;
            }
        }
    }
}
