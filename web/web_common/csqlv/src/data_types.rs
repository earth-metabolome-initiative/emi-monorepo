//! Submodule providing a data type enumeration.
use crate::errors::CSVSchemaError;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DataType {
    Text,
    VarChar(usize),
    Uuid,
    SmallInt,
    Integer,
    BigInt,
    Real,
    Double,
    Null,
    SmallSerial,
    Serial,
    BigSerial,
    Boolean,
}

impl DataType {
    /// Returns the minimum dimension of the data type.
    pub fn min_dimension(&self) -> u64 {
        match self {
            DataType::Text => 32,
            DataType::VarChar(_) => 24,
            DataType::Uuid => 16,
            DataType::SmallInt | DataType::SmallSerial => 2,
            DataType::Integer | DataType::Real | DataType::Serial => 4,
            DataType::Double | DataType::BigInt | DataType::BigSerial => 8,
            DataType::Null => 0,
            DataType::Boolean => 1,
        }
    }

    /// Determines if the data type is a serial type.
    pub fn is_null(&self) -> bool {
        matches!(self, DataType::Null)
    }

    /// Determines the data type of a value.
    pub fn from_value(mut value: &str) -> Vec<DataType> {
        value = value.trim();
        if value.is_empty() {
            return vec![DataType::Null];
        }

        if Uuid::parse_str(value).is_ok() {
            return vec![
                DataType::Uuid,
                DataType::Text,
                DataType::VarChar(value.len()),
            ];
        }

        if value.parse::<i16>().is_ok() {
            return vec![
                DataType::SmallInt,
                DataType::Integer,
                DataType::BigInt,
                DataType::Text,
                DataType::VarChar(value.len()),
            ];
        }

        if value.parse::<i32>().is_ok() {
            return vec![
                DataType::Integer,
                DataType::BigInt,
                DataType::Text,
                DataType::VarChar(value.len()),
            ];
        }

        if value.parse::<i64>().is_ok() {
            return vec![
                DataType::BigInt,
                DataType::Text,
                DataType::VarChar(value.len()),
            ];
        }

        if value.parse::<f32>().is_ok() {
            return vec![
                DataType::Real,
                DataType::Double,
                DataType::Text,
                DataType::VarChar(value.len()),
            ];
        }

        if value.parse::<f64>().is_ok() {
            return vec![
                DataType::Double,
                DataType::Text,
                DataType::VarChar(value.len()),
            ];
        }

        if value.to_ascii_lowercase().eq_ignore_ascii_case("true")
            || value.to_ascii_lowercase().eq_ignore_ascii_case("false")
        {
            return vec![
                DataType::Boolean,
                DataType::Text,
                DataType::VarChar(value.len()),
            ];
        }

        vec![DataType::Text, DataType::VarChar(value.len())]
    }

    /// Converts into the serial variant of the data type.
    pub fn into_serial(self) -> Result<DataType, CSVSchemaError> {
        match self {
            DataType::SmallInt => Ok(DataType::SmallSerial),
            DataType::Integer => Ok(DataType::Serial),
            DataType::BigInt => Ok(DataType::BigSerial),
            DataType::Serial => Ok(DataType::Serial),
            DataType::SmallSerial => Ok(DataType::SmallSerial),
            DataType::BigSerial => Ok(DataType::BigSerial),
            error => Err(CSVSchemaError::UnknownDataType(format!(
                "Unknown Serial variant for {error:?}",
            ))),
        }
    }

    /// Returns the non-serial variant of the data type.
    pub fn into_non_serial(self) -> DataType {
        match self {
            DataType::SmallSerial => DataType::SmallInt,
            DataType::Serial => DataType::Integer,
            DataType::BigSerial => DataType::BigInt,
            other => other,
        }
    }

    /// Converts the data type to a string for use in SQL queries.
    pub fn to_postgres(self) -> String {
        match self {
            DataType::Text => "TEXT".to_owned(),
            DataType::VarChar(size) => format!("VARCHAR({size})"),
            DataType::Uuid => "UUID".to_owned(),
            DataType::SmallInt => "SMALLINT".to_owned(),
            DataType::Integer => "INTEGER".to_owned(),
            DataType::BigInt => "BIGINT".to_owned(),
            DataType::Real => "REAL".to_owned(),
            DataType::Double => "DOUBLE PRECISION".to_owned(),
            DataType::Null => "NULL".to_owned(),
            DataType::SmallSerial => "SMALLSERIAL".to_owned(),
            DataType::Serial => "SERIAL".to_owned(),
            DataType::BigSerial => "BIGSERIAL".to_owned(),
            DataType::Boolean => "BOOLEAN".to_owned(),
        }
    }
}
