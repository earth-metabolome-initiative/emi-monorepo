use crate::csv_columns::CSVColumn;
use crate::errors::CSVSchemaError;
use crate::metadata::CSVTableMetadata;
use crate::CSVSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CSVTable<'a> {
    pub(crate) schema: &'a CSVSchema,
    pub(crate) table_metadata: &'a CSVTableMetadata,
}

impl<'a> CSVTable<'a> {
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

    pub fn into_postgres(&self) -> String {
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (\n",
            self.table_metadata.name
        );
        for column in &self.table_metadata.columns {
            sql.push_str(&format!(
                "    {} {}{}{}{}{},\n",
                column.name,
                column.data_type.into_postgres(),
                if column.primary_key {
                    " PRIMARY KEY"
                } else {
                    ""
                },
                if column.unique { " UNIQUE" } else { "" },
                if column.nullable { "" } else { " NOT NULL" },
                if let Some(foreign_table_name) = &column.foreign_table_name {
                    format!(
                        " REFERENCES {}({})",
                        foreign_table_name,
                        column.foreign_column_name.as_ref().unwrap()
                    )
                } else {
                    "".to_string()
                }
            ));
        }
        sql.push_str(");");
        sql
    }

    pub fn temporary_table(&self) -> Result<String, CSVSchemaError> {
        let temporary_table_name = self.table_metadata.temporary_table_name()?;

        let mut sql = format!("CREATE TEMPORARY TABLE {} (\n", temporary_table_name);
        for column in &self.table_metadata.columns {
            if column.artificial {
                continue;
            }
            sql.push_str(&format!(
                "    {} {}{}{},\n",
                column.name,
                column.data_type.into_postgres(),
                if column.unique { " UNIQUE" } else { "" },
                if column.nullable { "" } else { " NOT NULL" },
            ));
        }

        sql.pop();
        sql.pop();

        sql.push_str("\n);\n");

        if self.table_metadata.gzip() {
            sql.push_str(&format!(
                "\nCOPY {} FROM PROGRAM 'gzip -dc {}' DELIMITER ',' CSV HEADER;\n",
                temporary_table_name, self.table_metadata.path
            ));
        } else {
            sql.push_str(&format!(
                "\nCOPY {} FROM '{}' DELIMITER ',' CSV HEADER;\n",
                temporary_table_name, self.table_metadata.path
            ));
        }

        Ok(sql)
    }

    fn copy_from(&self) -> String {
        if self.table_metadata.gzip() {
            format!(
                "COPY {} FROM PROGRAM 'gzip -dc {}' DELIMITER ',' CSV HEADER;",
                self.table_metadata.name, self.table_metadata.path
            )
        } else {
            format!(
                "COPY {} FROM '{}' DELIMITER ',' CSV HEADER;",
                self.table_metadata.name, self.table_metadata.path
            )
        }
    }

    pub fn populate(&self) -> Result<String, CSVSchemaError> {
        if let Ok(mut sql) = self.temporary_table() {
            sql.push_str(&format!("\nINSERT INTO {} (\n", self.table_metadata.name));
            for column in &self.table_metadata.columns {
                if column.artificial {
                    continue;
                }
                sql.push_str(&format!("    {},\n", column.name));
            }

            sql.push_str(") SELECT\n");
            let temporary_table_name = self.table_metadata.temporary_table_name()?;

            for column in &self.table_metadata.columns {
                if column.artificial {
                    continue;
                }
                if let (Some(foreign_table_name), Some(foreign_column_name)) =
                    (&column.foreign_table_name, &column.foreign_column_name)
                {
                    sql.push_str(&format!(
                        "    {}.{}\n",
                        foreign_table_name, foreign_column_name
                    ));
                } else {
                    sql.push_str(&format!("    {}.{}\n", temporary_table_name, column.name));
                }
            }

            sql.push_str("FROM\n");
            sql.push_str(&format!("    {}", temporary_table_name));
            for column in &self.table_metadata.columns {
                if let (Some(foreign_table_name), Some(foreign_column_name)) =
                    (&column.foreign_table_name, &column.foreign_column_name)
                {
                    sql.push_str(&format!(
                        "\n    JOIN {} ON {}.{} = {}.{}",
                        foreign_table_name,
                        temporary_table_name,
                        column.name,
                        foreign_table_name,
                        foreign_column_name
                    ));
                }
            }

            sql.push_str(";\n");

            sql.push_str(&format!("\nDROP TABLE {temporary_table_name};\n",));

            Ok(sql)
        } else {
            Ok(self.copy_from())
        }
    }
}
