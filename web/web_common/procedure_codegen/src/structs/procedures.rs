//! Submodule defining a function to return the set of `Procedure`s tables
//! defined in the database.

use diesel::PgConnection;
use webcodegen::{Column, KeyColumnUsage, Table};

use crate::{
    ProcedureTemplate, errors::ProcedureError, is_asset_foreign_key, is_asset_model_foreign_key,
    is_procedure_assets_foreign_key, is_procedure_template_asset_model_foreign_key,
    procedure_templates::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// The name of the procedure table.
pub const PROCEDURES_TABLE_NAME: &str = "procedures";

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// Struct representing a procedure table.
pub(crate) struct Procedure {
    /// The underlying table.
    pub(super) table: Table,
}

impl From<Procedure> for Table {
    fn from(procedure: Procedure) -> Self {
        procedure.table
    }
}

impl From<Procedure> for Box<Table> {
    fn from(procedure: Procedure) -> Self {
        Box::new(procedure.table)
    }
}

impl AsRef<Table> for Procedure {
    fn as_ref(&self) -> &Table {
        &self.table
    }
}

/// Returns the foreign key linking the given procedure table to its
/// procedure template.
///
/// # Arguments
///
/// * `procedure_table` - The procedure table.
/// * `conn` - A mutable reference to a PostgreSQL connection.
///
/// # Errors
///
/// * If the database query fails, returns a `diesel::result::Error`.
/// * If the table is not recognized as a procedure, returns a custom error.
fn procedure_template_foreign_key(
    procedure_table: &Table,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, crate::errors::Error> {
    if procedure_table.table_name == PROCEDURES_TABLE_NAME {
        return Ok(None);
    }

    for foreign_key in procedure_table.ancestral_same_as_foreign_keys(conn)? {
        let local_columns = foreign_key.columns(conn)?;
        if local_columns.len() != 2 {
            continue;
        }
        for foreign_key in local_columns[1].foreign_primary_keys(conn)? {
            let foreign_table = foreign_key.foreign_table(conn)?;
            if ProcedureTemplate::from_table(foreign_table.as_ref().clone(), conn).is_ok() {
                return Ok(Some(foreign_key));
            }
        }
    }

    Err(ProcedureError::NotAProcedureTable(Box::new(procedure_table.clone())).into())
}

impl Procedure {
    /// Returns the root procedure table.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn root(
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, crate::errors::Error> {
        let table = Table::load(conn, PROCEDURES_TABLE_NAME, "public", table_catalog)?;
        Ok(Self { table: table.as_ref().clone() })
    }

    /// Returns whether the current procedure is the abstract.
    pub(crate) fn is_abstract(&self) -> bool {
        self.table.table_name == PROCEDURES_TABLE_NAME
    }

    /// Returns whether the given table is a procedure table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table to check.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table is not recognized as a procedure, returns a custom error.
    ///
    /// # Implementation details
    ///
    /// * A procedure table is defined as a table in the `procedures` schema.
    /// * It contains a column which is a foreign key to a procedure template
    ///   table.
    pub(crate) fn must_be_procedure_table(
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        if table.table_name == PROCEDURES_TABLE_NAME {
            return Ok(());
        }

        let procedure = Table::load(conn, PROCEDURES_TABLE_NAME, "public", &table.table_catalog)?;

        if !table.is_extending(&procedure, conn)? {
            return Err(ProcedureError::NotAProcedureTable(Box::new(table.clone())).into());
        }

        procedure_template_foreign_key(table, conn)?;

        Ok(())
    }

    /// Returns the columns which are foreign keys to procedure template asset
    /// models in the procedure table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure_template_asset_models(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, KeyColumnUsage)>, crate::errors::Error> {
        let mut asset_model_fk_columns = Vec::new();
        for column in self.table.columns(conn)?.iter() {
            if let Some(fk) = is_procedure_template_asset_model_foreign_key(&column, conn)? {
                asset_model_fk_columns.push((column.clone(), fk));
            }
        }
        Ok(asset_model_fk_columns)
    }

    /// Returns the columns which are foreign keys to asset models in the
    /// procedure table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn asset_models(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, KeyColumnUsage)>, crate::errors::Error> {
        let mut asset_model_fk_columns = Vec::new();
        for column in self.table.columns(conn)?.iter() {
            if let Some(fk) = is_asset_model_foreign_key(&column, conn)? {
                asset_model_fk_columns.push((column.clone(), fk));
            }
        }
        Ok(asset_model_fk_columns)
    }

    /// Returns the columns which are foreign keys to assets in the
    /// procedure table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn assets(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, KeyColumnUsage)>, crate::errors::Error> {
        let mut asset_fk_columns = Vec::new();
        for column in self.table.columns(conn)?.iter() {
            if let Some(fk) = is_asset_foreign_key(&column, conn)? {
                asset_fk_columns.push((column.clone(), fk));
            }
        }
        Ok(asset_fk_columns)
    }

    /// Returns the columns which are foreign keys to procedure assets
    /// in the procedure table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure_assets(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, KeyColumnUsage)>, crate::errors::Error> {
        let mut asset_fk_columns = Vec::new();
        for column in self.table.columns(conn)?.iter() {
            if let Some(fk) = is_procedure_assets_foreign_key(&column, conn)? {
                asset_fk_columns.push((column.clone(), fk));
            }
        }
        Ok(asset_fk_columns)
    }

    /// Returns the foreign keys to rule tables.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn rules(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, crate::errors::Error> {
        let mut rule_table_fks = Vec::new();
        for foreign_key in self.table.foreign_keys(conn)?.iter() {
            let foreign_table = foreign_key.foreign_table(conn)?;
            if foreign_table.table_name.ends_with("_compatibility_rules") {
                rule_table_fks.push(foreign_key.clone());
            }
        }
        Ok(rule_table_fks)
    }

    /// Returns the foreign key linking this procedure to its procedure
    /// template.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure_template_foreign_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, crate::errors::Error> {
        procedure_template_foreign_key(&self.table, conn)
    }

    /// Returns the `ProcedureTemplate` associated with this procedure.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure_template(
        &self,
        conn: &mut PgConnection,
    ) -> Result<ProcedureTemplate, crate::errors::Error> {
        ProcedureTemplate::from_table(
            if let Some(foreign_key) = self.procedure_template_foreign_key(conn)? {
                foreign_key.foreign_table(conn)?
            } else {
                Table::load(
                    conn,
                    PROCEDURE_TEMPLATES_TABLE_NAME,
                    "public",
                    &self.table.table_catalog,
                )?
            }
            .as_ref()
            .clone(),
            conn,
        )
    }

    /// Loads a `Procedure` by name.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `table_name` - The name of the procedure table.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table associated with the procedure is not recognized as a
    ///   procedure, returns a custom error.
    pub(crate) fn load_by_name(
        table_catalog: &str,
        table_name: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, crate::errors::Error> {
        let table = Table::load(conn, table_name, "public", table_catalog)?;
        Self::must_be_procedure_table(&table, conn)?;
        Ok(Self { table: table.as_ref().clone() })
    }

    /// Returns the set of `Procedure`s tables defined in the database.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn load_all(
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, crate::errors::Error> {
        let mut procedures = Vec::new();

        for table in Table::load_all(conn, table_catalog, "public")?.as_ref() {
            if Self::must_be_procedure_table(&table, conn).is_err() {
                continue;
            }
            procedures.push(Self { table: table.clone() });
        }
        Ok(procedures)
    }

    /// Returns the pairs of coupled procedure assets and procedure template
    /// asset models for the current procedure.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn coupled_pa_and_ptams(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, Column)>, crate::errors::Error> {
        let procedure_assets = self.procedure_assets(conn)?;
        let procedure_template_asset_models = self.procedure_template_asset_models(conn)?;

        let mut coupled = Vec::new();

        for (procedure_asset_column, _) in procedure_assets {
            let expected_name = format!(
                "{}_model",
                procedure_asset_column.column_name.as_str().replacen(
                    "procedure_",
                    "procedure_template_",
                    1,
                )
            );
            let (procedure_template_asset_model, _) = procedure_template_asset_models.iter().find(|(ptam_column, _)| {
                ptam_column.column_name == expected_name
            }).expect(&format!("Procedure asset column {procedure_asset_column} should have a matching procedure template asset model column called \"{expected_name}\""));
            coupled.push((procedure_asset_column, procedure_template_asset_model.clone()));
        }

        Ok(coupled)
    }
}
