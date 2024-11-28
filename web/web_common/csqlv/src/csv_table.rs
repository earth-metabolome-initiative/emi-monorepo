use crate::metadata::CSVTableMetadata;
use crate::CSVSchema;

pub struct CSVTable<'a> {
    pub(crate) schema: &'a CSVSchema,
    pub(crate) table_metadata: &'a CSVTableMetadata,
}

impl<'a> CSVTable<'a> {
    pub fn into_postgres(&self) -> String {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", self.table_metadata.name);
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
                if column.unique {
                    " UNIQUE"
                } else {
                    ""
                },
                if column.nullable {
                    ""
                } else {
                    " NOT NULL"
                },
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
}