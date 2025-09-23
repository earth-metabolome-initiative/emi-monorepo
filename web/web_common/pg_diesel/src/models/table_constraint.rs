use std::io::Write;

use diesel::{
    PgConnection, Queryable, QueryableByName, Selectable,
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{IsNull, Output, ToSql},
};

use crate::models::Table;

#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Debug, Clone, Ord, PartialOrd, Eq, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::table_constraints)]
/// A struct representing a table constraint
pub struct TableConstraint {
    /// The name of the constraint catalog
    pub constraint_catalog: String,
    /// The name of the constraint schema
    pub constraint_schema: String,
    /// The name of the constraint
    pub constraint_name: String,
    /// The name of the table catalog the constraint is associated with
    pub table_catalog: String,
    /// The name of the table schema the constraint is associated with
    pub table_schema: String,
    /// The name of the table the constraint is associated with
    pub table_name: String,
    /// The type of the constraint
    pub constraint_type: ConstraintType,
    /// Whether the constraint is deferrable
    pub is_deferrable: String,
    /// Whether the constraint is initially deferred
    pub initially_deferred: String,
    /// Whether the constraint is enforced
    pub enforced: String,
    /// The name of the column the constraint is associated with
    pub nulls_distinct: Option<String>,
}

#[derive(Debug, FromSqlRow, AsExpression, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(sql_type = diesel::sql_types::Text)]
/// An enum representing the type of a constraint
pub enum ConstraintType {
    /// A primary key constraint
    PrimaryKey,
    /// A foreign key constraint
    ForeignKey,
    /// A unique constraint
    Unique,
    /// A check constraint
    Check,
}

impl ConstraintType {
    pub fn is_primary_key(&self) -> bool {
        matches!(self, ConstraintType::PrimaryKey)
    }

    pub fn is_foreign_key(&self) -> bool {
        matches!(self, ConstraintType::ForeignKey)
    }

    pub fn is_unique(&self) -> bool {
        matches!(self, ConstraintType::Unique)
    }

    pub fn is_check(&self) -> bool {
        matches!(self, ConstraintType::Check)
    }
}

impl std::str::FromStr for ConstraintType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRIMARY KEY" => Ok(ConstraintType::PrimaryKey),
            "FOREIGN KEY" => Ok(ConstraintType::ForeignKey),
            "UNIQUE" => Ok(ConstraintType::Unique),
            "CHECK" => Ok(ConstraintType::Check),
            _ => Err(format!("Unknown constraint type: {s}")),
        }
    }
}

impl std::fmt::Display for ConstraintType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConstraintType::PrimaryKey => write!(f, "PRIMARY KEY"),
            ConstraintType::ForeignKey => write!(f, "FOREIGN KEY"),
            ConstraintType::Unique => write!(f, "UNIQUE"),
            ConstraintType::Check => write!(f, "CHECK"),
        }
    }
}

impl ToSql<diesel::sql_types::Text, diesel::pg::Pg> for ConstraintType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
        out.write_all(self.to_string().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<diesel::sql_types::Text, diesel::pg::Pg> for ConstraintType {
    fn from_sql(
        bytes: <diesel::pg::Pg as Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let text = std::str::from_utf8(bytes.as_bytes())?;
        match text {
            "PRIMARY KEY" => Ok(ConstraintType::PrimaryKey),
            "FOREIGN KEY" => Ok(ConstraintType::ForeignKey),
            "UNIQUE" => Ok(ConstraintType::Unique),
            "CHECK" => Ok(ConstraintType::Check),
            _ => Err("Unknown constraint type".into()),
        }
    }
}

impl TableConstraint {
    /// Returns the table associated with this table constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        Table::load(&self.table_name, &self.table_schema, &self.table_catalog, conn)
    }

    #[must_use]
    /// Returns whether the constraint is a primary key constraint
    pub fn is_primary_key(&self) -> bool {
        self.constraint_type.is_primary_key()
    }

    #[must_use]
    /// Returns whether the constraint is a foreign key constraint
    pub fn is_foreign_key(&self) -> bool {
        self.constraint_type.is_foreign_key()
    }

    #[must_use]
    /// Returns whether the constraint is a unique constraint
    pub fn is_unique(&self) -> bool {
        self.constraint_type.is_unique()
    }

    #[must_use]
    /// Returns whether the constraint is a check constraint
    pub fn is_check(&self) -> bool {
        self.constraint_type.is_check()
    }
}
