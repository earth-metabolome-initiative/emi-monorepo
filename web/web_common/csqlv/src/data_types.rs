//! Submodule providing a data type enumeration.
use std::str::FromStr;

use rosetta_uuid::Uuid;

use crate::errors::CSVSchemaError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Serialize, serde::Deserialize)]
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
    #[cfg(feature = "iso_codes")]
    CountryCode,
    #[cfg(feature = "font_awesome_icons")]
    FAIcon,
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
            #[cfg(feature = "iso_codes")]
            DataType::CountryCode => 2,
            #[cfg(feature = "font_awesome_icons")]
            DataType::FAIcon => 2,
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

        let mut candidate_types = vec![DataType::Text, DataType::VarChar(value.len())];

        #[cfg(feature = "iso_codes")]
        {
            use iso_codes::CountryCode;
            if CountryCode::try_from(value).is_ok() {
                candidate_types.push(DataType::CountryCode);
            }
        }

        #[cfg(feature = "font_awesome_icons")]
        {
            use font_awesome_icons::FAIcon;
            if FAIcon::from_str(value).is_ok() {
                candidate_types.push(DataType::FAIcon);
            }
        }

        if Uuid::from_str(value).is_ok() {
            candidate_types.push(DataType::Uuid);
        }

        if value.parse::<i16>().is_ok() {
            candidate_types.push(DataType::SmallInt);
        }

        if value.parse::<i32>().is_ok() {
            candidate_types.push(DataType::Integer);
        }

        if value.parse::<i64>().is_ok() {
            candidate_types.push(DataType::BigInt);
        }

        if value.parse::<f32>().is_ok() {
            candidate_types.push(DataType::Real);
        }

        if value.parse::<f64>().is_ok() {
            candidate_types.push(DataType::Double);
        }

        if value.to_ascii_lowercase().eq_ignore_ascii_case("true")
            || value.to_ascii_lowercase().eq_ignore_ascii_case("false")
        {
            candidate_types.push(DataType::Boolean);
        }

        candidate_types
    }

    /// Converts into the serial variant of the data type.
    pub fn into_serial(self) -> Result<DataType, CSVSchemaError> {
        match self {
            DataType::SmallInt | DataType::SmallSerial => Ok(DataType::SmallSerial),
            DataType::Integer | DataType::Serial => Ok(DataType::Serial),
            DataType::BigSerial | DataType::BigInt => Ok(DataType::BigSerial),
            error => {
                Err(CSVSchemaError::UnknownDataType(format!(
                    "Unknown Serial variant for {error:?}",
                )))
            }
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
    pub fn to_sql(self) -> String {
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
            #[cfg(feature = "iso_codes")]
            DataType::CountryCode => "CountryCode".to_owned(),
            #[cfg(feature = "font_awesome_icons")]
            DataType::FAIcon => "FAIcon".to_owned(),
        }
    }

    /// Returns whether the data type may be used as a primary key.
    pub fn is_key_like(&self) -> bool {
        matches!(self, DataType::SmallInt | DataType::Integer | DataType::BigInt | DataType::Uuid)
    }
}
