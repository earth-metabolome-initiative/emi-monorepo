use crate::{
    CSVSchema, csv_columns::CSVColumn, errors::CSVSchemaError, metadata::CSVTableMetadata,
};

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
    /// Returns the foreign table name of the table.
    pub fn foreign_table_name(&self) -> String {
        self.table_metadata.foreign_table_name()
    }

    /// Returns representations of the columns in the table.
    pub fn columns(&self) -> impl Iterator<Item = CSVColumn<'_>> {
        self.table_metadata
            .columns
            .iter()
            .map(|column_metadata| CSVColumn { table: self, column_metadata })
    }

    #[must_use]
    /// Returns whether the table has a column.
    pub fn has_column(&self, column_name: &str) -> bool {
        self.column_from_name(column_name).is_ok()
    }

    /// Returns the dependant tables of the table.
    pub fn dependant_tables(&self) -> impl Iterator<Item = CSVTable<'_>> {
        self.schema.tables().filter(|table| {
            table
                .columns()
                .any(|column| column.foreign_table().is_some_and(|t| t.name() == self.name()))
        })
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
            .map(|column_metadata| CSVColumn { table: self, column_metadata })
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
            .find(|column| column.name(self.schema).unwrap() == column_name)
            .ok_or(CSVSchemaError::InvalidColumnName(column_name.to_string()))?;
        Ok(CSVColumn { table: self, column_metadata })
    }

    /// Returns the name of the table.
    pub fn to_sql(&self) -> Result<String, CSVSchemaError> {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", self.table_metadata.name);
        for column in self.columns() {
            sql.push_str(&format!(
                "    {} {}{}{}{}{},\n",
                column.name()?,
                if let Some(foreign_table) = &column.foreign_table() {
                    foreign_table.primary_key().data_type().into_non_serial().to_sql()
                } else {
                    column.data_type().to_sql()
                },
                if column.is_primary_key() { " PRIMARY KEY" } else { "" },
                if column.is_unique() { " UNIQUE" } else { "" },
                if column.is_nullable() || column.is_primary_key() { "" } else { " NOT NULL" },
                if let Some(foreign_table) = &column.foreign_table() {
                    format!(
                        " REFERENCES {}({})",
                        foreign_table.name(),
                        foreign_table.primary_key().name()?
                    )
                } else {
                    String::new()
                }
            ));
        }
        sql.pop();
        sql.pop();
        sql.push_str("\n);");
        Ok(sql)
    }

    #[must_use]
    /// Returns postgres SQL operation to delete the table.
    pub fn to_sql_delete(&self) -> String {
        format!("DROP TABLE IF EXISTS {} CASCADE;", self.table_metadata.name)
    }

    /// Returns the SQL to generate the temporary variant of the table.
    fn temporary_table(&self) -> Result<String, CSVSchemaError> {
        let temporary_table_name = self.table_metadata.temporary_table_name();

        let mut sql = format!("CREATE TEMPORARY TABLE {temporary_table_name} (\n");
        for column in self.columns() {
            if column.is_artificial() {
                continue;
            }
            if let Some(foreign_table) = column.foreign_table() {
                sql.push_str(&format!(
                    "    {}_{} {},\n",
                    foreign_table.name(),
                    column.foreign_column_name().unwrap(),
                    column.data_type().to_sql(),
                ));
                continue;
            }

            sql.push_str(&format!(
                "    {} {}{}{},\n",
                column.name()?,
                column.data_type().to_sql(),
                if column.is_unique() { " UNIQUE" } else { "" },
                if column.is_nullable() { "" } else { " NOT NULL" },
            ));
        }

        sql.pop();
        sql.pop();

        sql.push_str("\n);\n");

        let delimiter = self.table_metadata.delimiter();

        if self.table_metadata.gzip() {
            sql.push_str(&format!(
                "\nCOPY {temporary_table_name} FROM PROGRAM 'gzip -dc {}' DELIMITER '{delimiter}' CSV HEADER NULL '';\n",
                self.table_metadata.path
            ));
        } else {
            sql.push_str(&format!(
                "\nCOPY {temporary_table_name} FROM '{}' DELIMITER '{delimiter}' CSV HEADER NULL '';\n",
                self.table_metadata.path
            ));
        }

        Ok(sql)
    }

    /// Returns the SQL to populate the table.
    ///
    /// # Errors
    /// 
    /// * If the format of the table is invalid.
    /// 
    /// # Panics
    /// 
    /// * If the schema is in an invalid state and the foreign table does not
    ///   exist.
    pub fn populate(&self) -> Result<String, CSVSchemaError> {
        use std::fmt::Write;
        let mut sql = self.temporary_table()?;

        writeln!(sql, "\nTRUNCATE TABLE {} RESTART IDENTITY CASCADE;\n", self.table_metadata.name)?;
        writeln!(sql, "\nINSERT INTO {} (\n", self.table_metadata.name)?;
        for column in &self.table_metadata.columns {
            if column.artificial {
                continue;
            }
            writeln!(sql, "    {},\n", column.name(self.schema)?)?;
        }

        sql.pop();
        sql.pop();

        writeln!(sql, "\n) SELECT")?;
        let temporary_table_name = self.table_metadata.temporary_table_name();

        for column in self.columns() {
            if column.is_artificial() {
                continue;
            }
            if let Some(foreign_table) = column.foreign_table() {
                writeln!(
                    sql,
                    "    {}.{},\n",
                    foreign_table.name(),
                    foreign_table.primary_key().name()?
                )?;
            } else {
                writeln!(sql, "    {}.{},\n", temporary_table_name, column.name()?)?;
            }
        }
        sql.pop();
        sql.pop();

        writeln!(sql, "\nFROM\n    {temporary_table_name}")?;
        for column in self.columns() {
            if let Some(foreign_table) = column.foreign_table() {
                writeln!(
                    sql,
                    "\n    JOIN {} ON {}.{}_{} = {}.{}",
                    foreign_table.name(),
                    temporary_table_name,
                    foreign_table.name(),
                    column.foreign_column_name().unwrap(),
                    foreign_table.name(),
                    column.foreign_column_name().unwrap()
                )?;
            }
        }

        writeln!(sql, ";",)?;
        writeln!(sql, "\nDROP TABLE {temporary_table_name};",)?;

        Ok(sql)
    }
}
