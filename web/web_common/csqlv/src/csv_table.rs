use crate::csv_columns::CSVColumn;
use crate::errors::CSVSchemaError;
use crate::metadata::CSVTableMetadata;
use crate::CSVSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a CSV table.
pub struct CSVTable<'a> {
    pub(crate) schema: &'a CSVSchema,
    pub(crate) table_metadata: &'a CSVTableMetadata,
}

impl<'a> CSVTable<'a> {
    #[must_use]
    /// Returns the name of the table.
    pub fn name(&self) -> &str {
        &self.table_metadata.name
    }

    #[must_use]
    /// Returns representations of the columns in the table.
    pub fn columns(&self) -> Vec<CSVColumn<'_>> {
        self.table_metadata
            .columns
            .iter()
            .map(|column_metadata| CSVColumn {
                table: self,
                column_metadata,
            })
            .collect()
    }

    #[must_use]
    /// Returns the dependant tables of the table.
    pub fn dependant_tables(&self) -> Vec<CSVTable<'_>> {
        self.schema
            .tables()
            .into_iter()
            .filter(|table| {
                table.columns().iter().any(|column| {
                    column
                        .foreign_table()
                        .is_some_and(|t| t.name() == self.name())
                })
            })
            .collect()
    }

    #[must_use]
    /// Returns the primary key of the table.
    ///
    /// # Panics
    /// * If the table is in an invalid state and does not have a primary key.
    pub fn primary_key(&self) -> CSVColumn<'_> {
        self.table_metadata
            .columns
            .iter()
            .find(|column| column.primary_key)
            .map(|column_metadata| CSVColumn {
                table: self,
                column_metadata,
            })
            .unwrap()
    }

    /// Returns the column associated with the provided name.
    ///
    /// # Errors
    /// * If the column name is invalid.
    pub fn column_from_name(&self, column_name: &str) -> Result<CSVColumn<'_>, CSVSchemaError> {
        let column_metadata = self
            .table_metadata
            .columns
            .iter()
            .find(|column| column.name == column_name)
            .ok_or(CSVSchemaError::InvalidColumnName(column_name.to_string()))?;
        Ok(CSVColumn {
            table: self,
            column_metadata,
        })
    }

    #[must_use]
    /// Returns the name of the table.
    pub fn to_postgres(&self) -> String {
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (\n",
            self.table_metadata.name
        );
        for column in &self.columns() {
            sql.push_str(&format!(
                "    {} {}{}{}{}{},\n",
                column.name(),
                if let Some(foreign_table) = &column.foreign_table() {
                    foreign_table.primary_key().data_type().to_postgres()
                } else {
                    column.data_type().to_postgres()
                },
                if column.is_primary_key() {
                    " PRIMARY KEY"
                } else {
                    ""
                },
                if column.is_unique() { " UNIQUE" } else { "" },
                if column.is_nullable() {
                    ""
                } else {
                    " NOT NULL"
                },
                if let Some(foreign_table) = &column.foreign_table() {
                    format!(
                        " REFERENCES {}({})",
                        foreign_table.name(),
                        foreign_table.primary_key().name()
                    )
                } else {
                    String::new()
                }
            ));
        }
        sql.pop();
        sql.pop();
        sql.push_str("\n);");
        sql
    }

    #[must_use]
    /// Returns postgres SQL operation to delete the table.
    pub fn to_postgres_delete(&self) -> String {
        format!("DROP TABLE IF EXISTS {} CASCADE;", self.table_metadata.name)
    }

    /// Returns the SQL to generate the temporary variant of the table.
    fn temporary_table(&self) -> String {
        let temporary_table_name = self.table_metadata.temporary_table_name();

        let mut sql = format!("CREATE TEMPORARY TABLE {temporary_table_name} (\n");
        for column in &self.columns() {
            if column.is_artificial() {
                continue;
            }
            if let Some(foreign_table) = column.foreign_table() {
                sql.push_str(&format!(
                    "    {}_{} {},\n",
                    foreign_table.name(),
                    column.foreign_column_name().unwrap(),
                    column.data_type().to_postgres(),
                ));
                continue;
            }

            sql.push_str(&format!(
                "    {} {}{}{},\n",
                column.name(),
                column.data_type().to_postgres(),
                if column.is_unique() { " UNIQUE" } else { "" },
                if column.is_nullable() {
                    ""
                } else {
                    " NOT NULL"
                },
            ));
        }

        sql.pop();
        sql.pop();

        sql.push_str("\n);\n");

        if self.table_metadata.gzip() {
            sql.push_str(&format!(
                "\nCOPY {temporary_table_name} FROM PROGRAM 'gzip -dc {}' DELIMITER ',' CSV HEADER;\n",
                self.table_metadata.path
            ));
        } else {
            sql.push_str(&format!(
                "\nCOPY {temporary_table_name} FROM '{}' DELIMITER ',' CSV HEADER;\n",
                self.table_metadata.path
            ));
        }

        sql
    }

    #[must_use]
    /// Returns the SQL to populate the table.
    /// 
    /// # Panics
    /// * If the schema is in an invalid state and the foreign table does not exist.
    pub fn populate(&self) -> String {
        let mut sql = self.temporary_table();
        sql.push_str(&format!("\nINSERT INTO {} (\n", self.table_metadata.name));
        for column in &self.table_metadata.columns {
            if column.artificial {
                continue;
            }
            sql.push_str(&format!("    {},\n", column.name));
        }

        sql.pop();
        sql.pop();

        sql.push_str("\n) SELECT\n");
        let temporary_table_name = self.table_metadata.temporary_table_name();

        for column in &self.columns() {
            if column.is_artificial() {
                continue;
            }
            if let Some(foreign_table) = column.foreign_table() {
                sql.push_str(&format!(
                    "    {}.{},\n",
                    foreign_table.name(),
                    foreign_table.primary_key().name()
                ));
            } else {
                sql.push_str(&format!(
                    "    {}.{},\n",
                    temporary_table_name,
                    column.name()
                ));
            }
        }
        sql.pop();
        sql.pop();

        sql.push_str("\nFROM\n");
        sql.push_str(&format!("    {temporary_table_name}"));
        for column in &self.columns() {
            if let Some(foreign_table) = column.foreign_table() {
                sql.push_str(&format!(
                    "\n    JOIN {} ON {}.{}_{} = {}.{}",
                    foreign_table.name(),
                    temporary_table_name,
                    foreign_table.name(),
                    column.foreign_column_name().unwrap(),
                    foreign_table.name(),
                    column.foreign_column_name().unwrap()
                ));
            }
        }

        sql.push_str(";\n");

        sql.push_str(&format!("\nDROP TABLE {temporary_table_name};\n",));

        sql
    }
}
