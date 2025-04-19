//! Implementation of the [`Translator`] trait for the
//! [`DataType`](sqlparser::ast::DataType) type.

use sqlparser::ast::DataType;

use crate::prelude::{Pg2Sqlite, Translator};

impl Translator for DataType {
    type Schema = Pg2Sqlite;
    type SQLiteEntry = DataType;

    fn translate(&self, _schema: &Self::Schema) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        match self {
            DataType::Text | DataType::Integer(None) | DataType::Real => Ok(self.clone()),
            DataType::SmallInt(None) => Ok(DataType::Integer(None)),
            DataType::Int(None) => Ok(DataType::Integer(None)),
            DataType::Float(None) => Ok(DataType::Real),
            DataType::Bytea => Ok(DataType::Blob(None)),
            DataType::Varchar(_) => Ok(DataType::Text),
            DataType::Boolean | DataType::Bool => Ok(DataType::Integer(None)),
            DataType::Uuid => {
                // SQLite does not have a UUID type, so we use BLOB instead as
                // a workaround, as described in the `rosetta-uuid` crate.
                Ok(DataType::Blob(None))},
            DataType::Timestamp(None, sqlparser::ast::TimezoneInfo::WithTimeZone) => {
                // SQLite does not support timezone information, and these type of
                // fields are commonly converted to TEXT.
                Ok(DataType::Text)
            }
            DataType::Timestamp(None, sqlparser::ast::TimezoneInfo::None) => {
                // SQLite does not support actually support timestamp, but emulates it
                // with several different types. Since in the `diesel` library the backend
                // type is `Text`, we will use that as well.
                Ok(DataType::Text)
            }
            DataType::Custom(name, ..) => {
                match name.0.first().and_then(|s| Some(s.as_ident()?.value.as_str())) {
                    Some("SERIAL") => Ok(DataType::Integer(None)),
                    Some("GEOGRAPHY") => {
                        // SQLite does not have postgis support, but we have implemented
                        // support in the `postgis-diesel` crate for the `geometry` and
                        // `geography` types, both of which use `BLOB` in SQLite.
                        Ok(DataType::Blob(None))
                    }
                    unimplemented => {
                        unimplemented!("The data type {:?} is not supported", unimplemented)
                    }
                }
            }
            unimplemented => unimplemented!("The data type {:?} is not supported", unimplemented),
        }
    }
}
