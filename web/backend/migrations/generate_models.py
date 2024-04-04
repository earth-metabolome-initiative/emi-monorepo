"""Python script to run a patched version of the diesel extended CLI to generate models.

Implementation details
----------------------
The diesel extended CLI can be used to generated the structs associated to the database tables. However, the
generated structs are not complete most commonly, as it does not come equipped with some of the postgres types.
Fortunately, this can be easily patched with some replace statements.

We start by running the extended CLI command:

```bash
diesel_ext --model --add-table-name > src/models.rs
```

Then, we need to handle the following replacements, plus adding on the top of the file the associated imports.
The imports need to be added solely when the replacements are effective, i.e. there is actually a change in the file,
otherwise we would add unnecessary imports and cause compilation warnings.

The replacements are defined in the file `replacements.json` and are applied to the generated file `src/models.rs`.
The structure of the JSON file is as follows:

```json
[
    {
        "search": "search_string",
        "replace": "replace_string",
        "imports": ["import1", "import2"]
    }
]
```

With a more concrete example:

```json
[
    {
        "search": "Option</* TODO: unknown type Nullable<Numrange> */>",
        "replace": "Option<Range<Numeric>>",
        "imports": [
            "use diesel::sql_types::Numeric;",
            "use diesel::sql_types::Range;"
        ]
    }
]
```

Since we also need to interface with the Frontend database which are NOT based
on Postgres, we also need to duplicate the code in the web commons and generate
the From implementations for the structs in the `src/models.rs` file. The `web_common`
structs will be generated in the `src/database/tables.rs` file in the `web_common` crate.
Since these structs are field-by-field identical, we can simply copy the structs while
ignoring the `#[derive(...)]` and `#[table_name = "..."]` attributes which would not be
applicable in the `web_common` crate. The `From` implementations will be generated in the
`src/models.rs` file in the `backend` crate, below each of the diesel-generated structs and
will make use of the full path to the struct in the `web_common` crate so to avoid conflicts.

"""

from typing import List, Dict
import psycopg2
import compress_json
import os
from dotenv import load_dotenv
from retrieve_ncbi_taxon import retrieve_ncbi_taxon


class PGIndex:

    def __init__(self, name: str, table_name: str, columns: List[str]):
        self.name = name
        self.table_name = table_name
        self.columns = columns

    def human_readable_columns(self) -> str:
        """Return the columns in a human-readable format."""
        last_column = self.columns[-1]
        if len(self.columns) == 1:
            return last_column
        return f"{', '.join(self.columns[:-1])} and {last_column}"


class PGIndices:

    def __init__(self, indices: List[PGIndex]):
        self.indices = indices

    def has_table(self, table_name: str) -> bool:
        for index in self.indices:
            if index.table_name == table_name:
                return True
        return False

    def get_table(self, table_name: str) -> PGIndex:
        for index in self.indices:
            if index.table_name == table_name:
                return index
        return None


def get_cursor():
    """Get the cursor to the database."""
    dbname = os.getenv("POSTGRES_DB")
    user = os.getenv("POSTGRES_USER")
    password = os.getenv("POSTGRES_PASSWORD")
    url = os.getenv("POSTGRES_URL")

    # Establishing a connection to the PostgreSQL database
    conn = psycopg2.connect(
        dbname=dbname,
        user=user,
        password=password,
        host="localhost",
        port="5432",
    )

    return conn, conn.cursor()


def find_pg_trgm_indices() -> PGIndices:
    """Returns the list of indices that are of type `pg_trgm`."""
    conn, cursor = get_cursor()
    cursor.execute(
        """
        SELECT
            indexname AS index_name,
            tablename AS table_name,
            substring(indexdef from '\((.*)\)') AS columns_involved
        FROM
            pg_indexes
        WHERE
            indexdef ILIKE '%using gin%'
            AND indexdef ILIKE '%gin_trgm_ops%';
        """
    )
    indices = cursor.fetchall()
    pg_indices = []
    for index in indices:
        sanitized_coumn_names = []
        for column in index[2].split(", "):
            sanitized_coumn_names.append(column.split(" ")[0])

        pg_indices.append(PGIndex(index[0], index[1], sanitized_coumn_names))

    cursor.close()
    conn.close()

    return PGIndices(pg_indices)


def write_from_impls(
    path: str, table_type: str, table_structs: Dict[str, Dict[str, str]]
):
    """Write the `From` implementations for the structs in the `src/models.rs` file."""

    if table_type not in ["tables", "views"]:
        raise ValueError("The table type must be either 'tables' or 'views'.")

    similarity_indices: PGIndices = find_pg_trgm_indices()

    with open(path, "r") as file:
        content = file.read()

    # After each struct ends, as defined by the `}` character, after
    # we have found a `struct` keyword, we write the `From` implementation
    # for the struct where we implement the conversion to the struct in the
    # `web_common` crate.

    impl_from_line = "impl From<{struct_name}> for web_common::database::{table_type}::{struct_name} {{\n"
    reverse_from = "impl From<web_common::database::{table_type}::{struct_name}> for {struct_name} {{\n"

    struct_name = None
    struct_field_names = []
    new_content = ""

    for line in content.split("\n"):
        new_content += line + "\n"

        if "struct" in line:
            # We have found a line such as "pub struct Archivable {"
            # and we need to extract the struct name.
            struct_name = line.split(" ")[2]

        # We are currently inside a struct
        if struct_name is not None:
            # And we have found the end of the struct
            if "}" in line:
                # We have found the end of the struct, and we write the
                # `From` implementation.
                new_content += "\n"
                new_content += impl_from_line.format(
                    struct_name=struct_name, table_type=table_type
                )
                new_content += f"    fn from(item: {struct_name}) -> Self {{\n"
                new_content += "        Self {\n"
                for field_name in struct_field_names:
                    new_content += f"            {field_name}: item.{field_name},\n"
                new_content += "        }\n"
                new_content += "    }\n"
                new_content += "}\n\n"

                new_content += reverse_from.format(
                    struct_name=struct_name, table_type=table_type
                )
                new_content += f"    fn from(item: web_common::database::{table_type}::{struct_name}) -> Self {{\n"
                new_content += "        Self {\n"
                for field_name in struct_field_names:
                    new_content += f"            {field_name}: item.{field_name},\n"
                new_content += "        }\n"
                new_content += "    }\n"
                new_content += "}\n\n"

                # We now generate the `get` method for the diesel struct.
                # This method receives the ID of the struct and returns the
                # struct from the database.
                #
                # ```rust
                # pub fn get(
                #     id: Uuid,
                #     connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
                # ) -> Result<Self, diesel::Error> {
                #     login_providers::dsl::login_providers
                #         .filter(login_providers::dsl::id.eq(provider_id))
                #         .first::<Self>(&mut conn)
                # }
                # ```

                table_data = table_structs[struct_name]
                table_name = table_data["table_name"]

                new_content += f"impl {struct_name} {{\n"
                new_content += f"    /// Get the struct from the database by its ID.\n"
                new_content += f"    ///\n"
                new_content += f"    /// # Arguments\n"
                new_content += f"    /// * `id` - The ID of the struct to get.\n"
                new_content += (
                    f"    /// * `connection` - The connection to the database.\n"
                )
                new_content += f"    ///\n"
                new_content += f"    pub fn get(\n"
                new_content += f"        id: Uuid,\n"
                new_content += f"        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                new_content += f"    ) -> Result<Self, diesel::result::Error> {{\n"
                new_content += f"        {table_name}::dsl::{table_name}\n"
                new_content += f"            .filter({table_name}::dsl::id.eq(id))\n"
                new_content += f"            .first::<Self>(connection)\n"
                new_content += f"    }}\n"

                # If this table implements the `pg_trgm` index, we also
                # provide the `search` method to search for the struct
                # by a given string. The method also receives a limit
                # parameter to limit the number of results and a threshold
                # parameter to set the similarity threshold.
                if similarity_indices.has_table(table_name):
                    index = similarity_indices.get_table(table_name)
                    index_columns = index.columns
                    new_content += f"    /// Search for the struct by a given string.\n"
                    new_content += f"    ///\n"
                    new_content += f"    /// # Arguments\n"
                    new_content += f"    /// * `query` - The string to search for.\n"
                    new_content += f"    /// * `limit` - The maximum number of results, by default `10`.\n"
                    new_content += f"    /// * `threshold` - The similarity threshold, by default `0.6`.\n"
                    new_content += (
                        f"    /// * `connection` - The connection to the database.\n"
                    )
                    new_content += f"    ///\n"
                    new_content += f"    pub fn search(\n"
                    new_content += f"        query: &str,\n"
                    new_content += f"        limit: Option<i32>,\n"
                    new_content += f"        threshold: Option<f64>,\n"
                    new_content += f"        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
                    new_content += (
                        f"    ) -> Result<Vec<Self>, diesel::result::Error> {{\n"
                    )

                    if table_type == "tables":
                        new_content += f"        use crate::schema::{table_name};\n"
                    elif table_type == "views":
                        new_content += (
                            f"        use crate::views::schema::{table_name};\n"
                        )
                    else:
                        raise NotImplementedError(
                            "The table type must be either 'tables' or 'views'."
                        )

                    new_content += f"        let limit = limit.unwrap_or(10);\n"
                    new_content += (
                        f"        let threshold = threshold.unwrap_or(0.6);\n"
                    )

                    # Since Diesel does not support the `similarity` Postgres function natively
                    # as part of the DSL query builder, we are forced to build the query manually
                    # in raw SQL. We use the `sql_query` function to execute the raw SQL query.
                    # Since the `sql_query` function needs to run a raw SQL query, we need to
                    # sanitize the input to avoid SQL injection attacks.

                    joined_field_names = ", ".join(struct_field_names)

                    new_content += f"        let similarity_query = format!(concat!(\n"
                    new_content += f'            r#"SELECT {joined_field_names} FROM {table_name} WHERE",\n'
                    new_content += f"            \"similarity({', '.join(index_columns)}, '$1') > $2\",\n"
                    new_content += f"            \"ORDER BY similarity({', '.join(index_columns)}, '$1') DESC LIMIT $3;\"#\n"
                    new_content += f"        ));\n"

                    new_content += f"        diesel::sql_query(similarity_query)\n"
                    new_content += (
                        f"            .bind::<diesel::sql_types::Text, _>(query)\n"
                    )
                    new_content += f"            .bind::<diesel::sql_types::Float8, _>(threshold)\n"
                    new_content += (
                        f"            .bind::<diesel::sql_types::Integer, _>(limit)\n"
                    )
                    new_content += f"            .load(connection)\n"

                    new_content += f"}}\n"

                # Finally, we cluse the struct implementation.
                new_content += f"}}\n"

                struct_name = None
                struct_field_names = []

        # We are currently inside a struct, and we are looking for
        # the field names.
        if struct_name is not None and "pub" in line and ":" in line:
            # We have found a line such as `pub name: String,`
            # and we need to extract the field name.
            field_name = line.strip().split(" ")[1].strip(":")
            struct_field_names.append(field_name)

    # Finally, we create an enumeration for the tables (or views)
    # depending on the table type. The enumeration, being for the
    # Rows of the tables, will be called `TableRow` or `ViewRow`.
    # This enumeration implements the get method for each of the variants,
    # receiving the ID of the row, the connection to the database, and
    # the identifier of either the Table or View, which is defined in the
    # `Table` or `View` enumeration in web_common. To avoid potential collisions,
    # we use the extended import path to web_common. Furthermore, we implement
    # the bidirectional From for the TableRow or ViewRow of their respective
    # structs in the `web_common` crate.

    if table_type == "tables":
        capitalized_table_type = "Table"
    elif table_type == "views":
        capitalized_table_type = "View"

    table_deribes = "#[derive(" "Deserialize, Serialize, Clone, Debug, PartialEq" ")]\n"

    # We start by writing the enumeration
    new_content += table_deribes
    new_content += f"pub enum {capitalized_table_type}Row {{\n"
    for struct_name in table_structs.keys():
        new_content += f"    {struct_name}({struct_name}),\n"
    new_content += f"}}\n\n"

    # Next up, we implement the bidirectional From for the TableRow or ViewRow
    # of their respective structs in the `web_common` crate.
    new_content += f"impl From<web_common::database::{table_type}::{capitalized_table_type}Row> for {capitalized_table_type}Row {{\n"
    new_content += f"    fn from(item: web_common::database::{table_type}::{capitalized_table_type}Row) -> Self {{\n"
    new_content += f"        match item {{\n"
    for struct_name in table_structs.keys():
        new_content += f"            web_common::database::{table_type}::{capitalized_table_type}Row::{struct_name}(item) => {capitalized_table_type}Row::{struct_name}(item.into()),\n"
    new_content += f"        }}\n"
    new_content += f"    }}\n"
    new_content += f"}}\n"

    new_content += f"impl From<{capitalized_table_type}Row> for web_common::database::{table_type}::{capitalized_table_type}Row {{\n"
    new_content += f"    fn from(item: {capitalized_table_type}Row) -> Self {{\n"
    new_content += f"        match item {{\n"
    for struct_name in table_structs.keys():
        new_content += f"            {capitalized_table_type}Row::{struct_name}(item) => web_common::database::{table_type}::{capitalized_table_type}Row::{struct_name}(item.into()),\n"
    new_content += f"        }}\n"
    new_content += f"    }}\n"
    new_content += f"}}\n"

    # For each of the structs, we implement the From method so that it is possible to easily convert
    # any of the Row structs into the ViewRow or TableRow structs.

    for struct_name in table_structs.keys():
        new_content += f"impl From<{struct_name}> for {capitalized_table_type}Row {{\n"
        new_content += f"    fn from(item: {struct_name}) -> Self {{\n"
        new_content += f"        {capitalized_table_type}Row::{struct_name}(item)\n"
        new_content += f"    }}\n"
        new_content += f"}}\n"

    # Now we write the enum for the table or view names, analogous to the `Table` or `View`
    # enumeration in the `web_common` crate. We implement also the bidirectional From method
    # for the enumeration and the table or view name. Furthermore, we implement the Display
    # trait for the enumeration, returning the table or view name for each of the variants.
    # We implement the `get` method for the enumeration, which receives the ID of the row, and the
    # connection to the database. Since we already have the `&self` reference to which variant
    # the enumeration is, we can use the `match` statement to call the `get` method for the respective
    # struct without needing to provide the table or view name.

    derives = [
        "Deserialize",
        "Serialize",
        "Clone",
        "Debug",
        "PartialEq",
        "Copy",
        "Eq",
    ]

    new_content += f"#[derive("
    for derive in derives:
        new_content += f"{derive}, "
    new_content += f")]\n"

    new_content += f"pub enum {capitalized_table_type} {{\n"
    for struct_name in table_structs.keys():
        new_content += f"    {struct_name},\n"
    new_content += f"}}\n\n"

    new_content += f"impl {capitalized_table_type} {{\n"
    new_content += f"    pub fn name(&self) -> &'static str {{\n"
    new_content += f"        match self {{\n"
    for struct_name, table_data in table_structs.items():
        table_name = table_data["table_name"]
        new_content += (
            f'            {capitalized_table_type}::{struct_name} => "{table_name}",\n'
        )
    new_content += f"        }}\n"
    new_content += f"    }}\n"

    new_content += f"    /// Get the struct from the database by its ID.\n"
    new_content += f"    ///\n"
    new_content += f"    /// # Arguments\n"
    new_content += f"    /// * `id` - The ID of the struct to get.\n"
    new_content += f"    /// * `connection` - The connection to the database.\n"
    new_content += f"    ///\n"
    new_content += f"    pub fn get(\n"
    new_content += f"        &self,\n"
    new_content += f"        id: Uuid,\n"
    new_content += f"        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
    new_content += (
        f"    ) -> Result<{capitalized_table_type}Row, diesel::result::Error> {{\n"
    )
    new_content += f"        Ok(match self {{\n"
    for struct_name in table_structs.keys():
        new_content += f"            {capitalized_table_type}::{struct_name} => {capitalized_table_type}Row::{struct_name}({struct_name}::get(id, connection)?),\n"
    new_content += f"        }})\n"
    new_content += f"    }}\n"
    new_content += f"}}\n"

    new_content += f"impl std::fmt::Display for {capitalized_table_type} {{\n"
    new_content += (
        f"    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n"
    )
    new_content += f'        write!(f, "{{}}", self.name())\n'
    new_content += f"    }}\n"
    new_content += f"}}\n"

    new_content += f"impl From<&str> for {capitalized_table_type} {{\n"
    new_content += f"    fn from(item: &str) -> Self {{\n"
    new_content += f"        match item {{\n"
    for struct_name, table_data in table_structs.items():
        table_name = table_data["table_name"]
        new_content += (
            f'            "{table_name}" => {capitalized_table_type}::{struct_name},\n'
        )
    new_content += f'            _ => panic!("Unknown {table_type} name"),\n'
    new_content += f"        }}\n"
    new_content += f"    }}\n"
    new_content += f"}}\n"

    # We implement the bidirectional From statement for the Table or View
    # enumerations with their correspective from the web_common crate.
    new_content += f"impl From<web_common::database::{table_type}::{capitalized_table_type}> for {capitalized_table_type} {{\n"
    new_content += f"    fn from(item: web_common::database::{table_type}::{capitalized_table_type}) -> Self {{\n"
    new_content += f"        match item {{\n"
    for struct_name in table_structs.keys():
        new_content += f"            web_common::database::{table_type}::{capitalized_table_type}::{struct_name} => {capitalized_table_type}::{struct_name},\n"
    new_content += f"        }}\n"
    new_content += f"    }}\n"
    new_content += f"}}\n"

    new_content += f"impl From<{capitalized_table_type}> for web_common::database::{table_type}::{capitalized_table_type} {{\n"
    new_content += f"    fn from(item: {capitalized_table_type}) -> Self {{\n"
    new_content += f"        match item {{\n"
    for struct_name in table_structs.keys():
        new_content += f"            {capitalized_table_type}::{struct_name} => web_common::database::{table_type}::{capitalized_table_type}::{struct_name},\n"
    new_content += f"        }}\n"
    new_content += f"    }}\n"
    new_content += f"}}\n"

    # Finally, we create the SearcheableTable or SearcheableView enumeration to cover all of the
    # tables or views that implement the `pg_trgm` index. We implement the `search` method for the
    # enumeration, which receives the query string, the limit, the threshold, and the connection to
    # the database. The method returns a vector of the respective TableRow or ViewRow. We also implement
    # the bidirectional From for the SearcheableTable or SearcheableView and their corresponding version
    # present in the web_common crate.

    has_any_searchables = any(
        similarity_indices.has_table(table_data["table_name"])
        for table_data in table_structs.values()
    )

    if has_any_searchables:
        new_content += f"#[derive("
        for derive in derives:
            new_content += f"{derive}, "
        new_content += f")]\n"

        new_content += f"pub enum Searcheable{capitalized_table_type} {{\n"
        for struct_name in table_structs.keys():
            if similarity_indices.has_table(table_structs[struct_name]["table_name"]):
                new_content += f"    {struct_name},\n"
        new_content += f"}}\n\n"

        new_content += f"impl Searcheable{capitalized_table_type} {{\n"
        new_content += f"    /// Search for the struct by a given string.\n"
        new_content += f"    ///\n"
        new_content += f"    /// # Arguments\n"
        new_content += f"    /// * `query` - The string to search for.\n"
        new_content += (
            f"    /// * `limit` - The maximum number of results, by default `10`.\n"
        )
        new_content += (
            f"    /// * `threshold` - The similarity threshold, by default `0.6`.\n"
        )
        new_content += f"    /// * `connection` - The connection to the database.\n"
        new_content += f"    ///\n"
        new_content += f"    pub fn search(\n"
        new_content += f"        &self,\n"
        new_content += f"        query: &str,\n"
        new_content += f"        limit: Option<i32>,\n"
        new_content += f"        threshold: Option<f64>,\n"
        new_content += f"        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>\n"
        new_content += f"    ) -> Result<Vec<{capitalized_table_type}Row>, diesel::result::Error> {{\n"
        new_content += f"        Ok(match self {{\n"
        for struct_name in table_structs.keys():
            if similarity_indices.has_table(table_structs[struct_name]["table_name"]):
                new_content += f"            Searcheable{capitalized_table_type}::{struct_name} => {struct_name}::search(query, limit, threshold, connection)?.into_iter().map({capitalized_table_type}Row::from).collect::<Vec<{capitalized_table_type}Row>>(),\n"
        new_content += f"        }})\n"
        new_content += f"    }}\n"
        new_content += f"}}\n"

        new_content += f"impl From<&str> for Searcheable{capitalized_table_type} {{\n"
        new_content += f"    fn from(item: &str) -> Self {{\n"
        new_content += f"        match item {{\n"
        for struct_name in table_structs.keys():
            if similarity_indices.has_table(table_structs[struct_name]["table_name"]):
                new_content += f'            "{table_structs[struct_name]["table_name"]}" => Searcheable{capitalized_table_type}::{struct_name},\n'
        new_content += f'            _ => panic!("Unknown {table_type} name"),\n'
        new_content += f"        }}\n"
        new_content += f"    }}\n"
        new_content += f"}}\n"

        new_content += f"impl From<Searcheable{capitalized_table_type}> for web_common::database::{table_type}::Searcheable{capitalized_table_type} {{\n"
        new_content += (
            f"    fn from(item: Searcheable{capitalized_table_type}) -> Self {{\n"
        )
        new_content += f"        match item {{\n"
        for struct_name in table_structs.keys():
            if similarity_indices.has_table(table_structs[struct_name]["table_name"]):
                new_content += f"            Searcheable{capitalized_table_type}::{struct_name} => web_common::database::{table_type}::Searcheable{capitalized_table_type}::{struct_name},\n"
        new_content += f"        }}\n"
        new_content += f"    }}\n"
        new_content += f"}}\n"

        new_content += f"impl From<web_common::database::{table_type}::Searcheable{capitalized_table_type}> for Searcheable{capitalized_table_type} {{\n"
        new_content += f"    fn from(item: web_common::database::{table_type}::Searcheable{capitalized_table_type}) -> Self {{\n"
        new_content += f"        match item {{\n"
        for struct_name in table_structs.keys():
            if similarity_indices.has_table(table_structs[struct_name]["table_name"]):
                new_content += f"            web_common::database::{table_type}::Searcheable{capitalized_table_type}::{struct_name} => Searcheable{capitalized_table_type}::{struct_name},\n"
        new_content += f"        }}\n"
        new_content += f"    }}\n"
        new_content += f"}}\n"

    with open(path, "w") as file:
        file.write(new_content)


def write_web_common_structs(
    path: str, target: str, enumeration: str
) -> Dict[str, Dict[str, str]]:
    """Write the structs in the target file in the `web_common` crate.

    Parameters
    ----------
    path : str
        The path from where to load the structs for the tables or views.
    target : str
        The path where to write the structs in the `web_common` crate.
    enumeration : str
        The name of the enumeration to write in the target file.
    """
    # The derive statements to include in the `src/database/tables.rs` document
    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
        "use chrono::Utc;",
    ]

    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = ["Deserialize", "Serialize", "Clone", "Debug", "PartialEq"]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith("backend"):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/{target}.rs", "w")

    similarity_indices: PGIndices = find_pg_trgm_indices()

    with open(path, "r") as file:
        models = file.read()

    for import_statement in imports:
        struct_imported = import_statement.split(":")[-1].strip(";")
        if struct_imported not in models:
            continue
        tables.write(f"{import_statement}\n")

    inside_struct = False

    # A dictionary to store the table names and their
    # respective structs.
    table_names = {}
    last_table_name = None
    struct_has_just_finished = False

    for line in models.split("\n"):
        # We skip all lines beginning with `//!` as they are comments
        if line.startswith("//!"):
            continue

        # We find the table name by searching lines like
        # #[diesel(table_name = item_continuous_quantities)]
        if "table_name =" in line:
            last_table_name = line.split("=")[1].strip(" )]")

        # We determine whether a new struct has started
        # by checking if the `struct` keyword is present
        # in the line.
        if "struct" in line:
            struct_metadata = {
                "table_name": None,
                "struct_name": None,
                "contains_options": False,
                "attributes": [],
            }
            struct_name = line.split(" ")[2]
            struct_metadata["table_name"] = last_table_name
            struct_metadata["struct_name"] = struct_name

            inside_struct = True

        if inside_struct:
            # If the current line contains the id field,
            # we store the type of the id field.
            if "pub" in line and ":" in line:
                field_name = line.strip().split(" ")[1].strip(":")
                field_type = line.split(":")[1].strip(", ")
                struct_metadata["attributes"].append(
                    {
                        "field_name": field_name,
                        "field_type": field_type,
                    }
                )
                if "Option" in field_type:
                    struct_metadata["contains_options"] = True

            # We determine whether the struct has ended
            # by checking if the `}` keyword is present
            # in the line.
            if "}" in line:
                inside_struct = False
                struct_has_just_finished = True

        if struct_has_just_finished:
            struct_has_just_finished = False
            # If the struct has finished, we can now begin with the
            # implementations for this struct.
            tables.write(f"#[derive(")
            for derive in derives:
                tables.write(f"{derive}, ")
            tables.write(f")]\n")
            tables.write(f"pub struct {struct_name} {{\n")
            for attribute in struct_metadata["attributes"]:
                tables.write(
                    f"    pub {attribute['field_name']}: {attribute['field_type']},\n"
                )
            tables.write("}\n")

            table_names[struct_name] = struct_metadata.copy()

            if enumeration == "Table":
                # This variant of the struct implementation is only
                # available when in the web_common is enabled the frontend
                # feature. It provides several methods including the use
                # of GlueSQL. Fortunately, it does not force us like Diesel
                # to create yet again another duplicate of the struct.
                tables.write(f'#[cfg(feature = "frontend")]\n')
                tables.write(f"impl {struct_name} {{\n")
                columns = ", ".join(
                    [
                        attribute["field_name"]
                        for attribute in struct_metadata["attributes"]
                    ]
                )
                table_name = struct_metadata["table_name"]
                struct_name = struct_metadata["struct_name"]

                # As first thing, we implement the `into_row` method for the struct. This method
                # converts the struct into a vector of `gluesql::core::ast_builder::ExprList`
                # variants, which are used to insert the struct into the GlueSQL database.
                types_and_methods = {
                    "i8": "gluesql::core::ast_builder::num({})",
                    "i16": "gluesql::core::ast_builder::num({})",
                    "i32": "gluesql::core::ast_builder::num({})",
                    "i64": "gluesql::core::ast_builder::num({})",
                    "i128": "gluesql::core::ast_builder::num({})",
                    "u8": "gluesql::core::ast_builder::num({})",
                    "u16": "gluesql::core::ast_builder::num({})",
                    "u32": "gluesql::core::ast_builder::num({})",
                    "u64": "gluesql::core::ast_builder::num({})",
                    "u128": "gluesql::core::ast_builder::num({})",
                    "f32": "gluesql::core::ast_builder::num({})",
                    "f64": "gluesql::core::ast_builder::num({})",
                    "String": "gluesql::core::ast_builder::text({})",
                    "Uuid": "gluesql::core::ast_builder::expr({}.to_string())",
                    "bool": "({}.into())",
                    "NaiveDateTime": "gluesql::core::ast_builder::timestamp({}.to_string())",
                    "DateTime<Utc>": "gluesql::core::ast_builder::timestamp({}.to_string())",
                }

                update_types_and_methods = types_and_methods.copy()
                update_types_and_methods["bool"] = "{}"

                tables.write(
                    f"    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {{\n"
                )

                tables.write(f"        vec![\n")
                for attribute in struct_metadata["attributes"]:
                    attribute_type = attribute["field_type"]
                    attribute_name = attribute["field_name"]

                    if attribute_type.startswith("Option<"):
                        inner_attribute_name = attribute_type[7:-1]
                        if inner_attribute_name in types_and_methods:
                            tables.write(
                                f"            match self.{attribute_name} {{\n"
                            )
                            tables.write(
                                f"                Some({attribute_name}) => {types_and_methods[inner_attribute_name].format(attribute_name)},\n"
                            )
                            tables.write(
                                f"                None => gluesql::core::ast_builder::null(),\n"
                            )
                            tables.write("            },\n")
                        else:
                            raise NotImplementedError(
                                f"The type {inner_attribute_name} is not supported. "
                                f"The struct {struct_name} contains an {attribute_type}. "
                            )
                    elif attribute_type in types_and_methods:
                        tables.write(
                            f"            {types_and_methods[attribute_type].format('self.{}'.format(attribute_name))},\n"
                        )
                    else:
                        raise NotImplementedError(
                            f"The type {attribute_type} is not supported."
                        )

                tables.write(f"        ]\n")

                tables.write("    }\n\n")

                # We implement the `insert` method for the struct. This method
                # receives a connection to the GlueSQL database and inserts the
                # struct into the database.
                tables.write(f"    /// Insert the {struct_name} into the database.\n")
                tables.write(f"    ///\n")
                tables.write(f"    /// # Arguments\n")
                tables.write(
                    f"    /// * `connection` - The connection to the database.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Returns\n")
                tables.write(
                    f"    /// The number of rows inserted in table {table_name}\n"
                )
                tables.write(f"    pub async fn insert<C>(\n")
                tables.write(f"        self,\n")
                tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(f"    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write(f"    {{\n")
                tables.write(f"        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.
                tables.write(f'        table("{table_name}")\n')
                tables.write(f"            .insert()\n")
                tables.write(f'            .columns("{columns}")\n')
                tables.write(f"            .values(vec![self.into_row()])\n")
                tables.write(f"            .execute(connection)\n")
                tables.write(f"            .await\n")
                tables.write("             .map(|payload| match payload {\n")
                tables.write(
                    "                 gluesql::prelude::Payload::Insert ( number_of_inserted_rows ) => number_of_inserted_rows,\n"
                )
                tables.write(
                    '                 _ => unreachable!("Payload must be an Insert"),\n'
                )
                tables.write("             })\n")
                tables.write("    }\n\n")

                # We implement the `get` method for the struct. This method
                # receives the ID of the struct and a connection to the GlueSQL
                # database. The method returns the struct from the database.
                tables.write(
                    f"    /// Get {struct_name} from the database by its ID.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Arguments\n")
                tables.write(f"    /// * `id` - The ID of {struct_name} to get.\n")
                tables.write(
                    f"    /// * `connection` - The connection to the database.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    pub async fn get<C>(\n")
                tables.write(f"        id: Uuid,\n")
                tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(
                    f"    ) -> Result<Option<Self>, gluesql::prelude::Error> where\n"
                )
                tables.write(
                    f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write(f"    {{\n")
                tables.write(f"        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.
                tables.write(f'        let select_row = table("{table_name}")\n')
                tables.write(f"            .select()\n")
                tables.write(f'            .filter(col("id").eq(id.to_string()))\n')
                tables.write(f'            .project("{columns}")\n')
                tables.write(f"            .limit(1)\n")
                tables.write(f"            .execute(connection)\n")
                tables.write(f"            .await?;\n")
                tables.write(f"         Ok(select_row.select()\n")
                tables.write(f"            .unwrap()\n")
                tables.write(f"            .map(Self::from_row)\n")
                tables.write(f"            .collect::<Vec<_>>()\n")
                tables.write(f"            .pop())\n")
                tables.write("    }\n\n")

                # We implement the `delete` method for the struct. This method deletes
                # the struct from the GlueSQL database.
                tables.write(f"    /// Delete {struct_name} from the database.\n")
                tables.write(f"    ///\n")
                tables.write(f"    /// # Arguments\n")
                tables.write(f"    /// * `id` - The ID of the struct to delete.\n")
                tables.write(
                    f"    /// * `connection` - The connection to the database.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Returns\n")
                tables.write(f"    /// The number of rows deleted.\n")
                tables.write(f"    pub async fn delete_from_id<C>(\n")
                tables.write(f"        id: Uuid,\n")
                tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(f"    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write(f"    {{\n")
                tables.write(f"        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.
                tables.write(f'        table("{table_name}")\n')
                tables.write(f"            .delete()\n")
                tables.write(f'            .filter(col("id").eq(id.to_string()))\n')
                tables.write(f"            .execute(connection)\n")
                tables.write(f"            .await\n")
                tables.write("             .map(|payload| match payload {\n")
                tables.write(
                    "                 gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,\n"
                )
                tables.write(
                    '                 _ => unreachable!("Payload must be a Delete"),\n'
                )
                tables.write("             })\n")
                tables.write("    }\n\n")

                # We implement the `delete` method for the struct. This method deletes
                # the current instance of the struct from the GlueSQL database.
                tables.write(
                    f"    /// Delete the current instance of {struct_name} from the database.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Arguments\n")
                tables.write(
                    f"    /// * `connection` - The connection to the database.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Returns\n")
                tables.write(f"    /// The number of rows deleted.\n")
                tables.write(f"    pub async fn delete<C>(\n")
                tables.write(f"        self,\n")
                tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(f"    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write(f"    {{\n")
                tables.write(
                    f"        Self::delete_from_id(self.id, connection).await\n"
                )
                tables.write("    }\n")

                # We implement the `update` method for the struct. This method updates
                # the struct in the GlueSQL database.
                tables.write(f"    /// Update the struct in the database.\n")
                tables.write(f"    ///\n")
                tables.write(f"    /// # Arguments\n")
                tables.write(
                    f"    /// * `connection` - The connection to the database.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Returns\n")
                tables.write(f"    /// The number of rows updated.\n")
                tables.write(f"    pub async fn update<C>(\n")
                tables.write(f"        self,\n")
                tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(f"    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write(f"    {{\n")
                tables.write(f"        use gluesql::core::ast_builder::*;\n")
                # We use the AST builder as much as possible so to avoid SQL injection attacks.
                tables.write(f'        let mut update_row = table("{table_name}")\n')
                tables.write(f"            .update();\n")
                for attribute in struct_metadata["attributes"]:
                    attribute_name = attribute["field_name"]
                    attribute_type = attribute["field_type"]
                    if attribute_type.startswith("Option<"):
                        inner_attribute_name = attribute_type[7:-1]
                        conversion = update_types_and_methods[
                            inner_attribute_name
                        ].format("self.{}".format(attribute_name))
                        if inner_attribute_name in update_types_and_methods:
                            tables.write(
                                f"        if let Some({attribute_name}) = self.{attribute_name} {{\n"
                            )
                            tables.write(
                                f'            update_row = update_row.set("{attribute_name}", {update_types_and_methods[inner_attribute_name].format(attribute_name)});\n'
                            )
                            tables.write("        }\n")
                        else:
                            raise NotImplementedError(
                                f"The type {inner_attribute_name} is not supported. "
                                f"The struct {struct_name} contains an {attribute_type}. "
                            )
                    elif attribute_type in update_types_and_methods:
                        conversion = update_types_and_methods[attribute_type].format(
                            "self.{}".format(attribute_name)
                        )
                        tables.write(
                            f'        update_row = update_row.set("{attribute_name}", {conversion});\n'
                        )
                    else:
                        raise NotImplementedError(
                            f"The type {attribute_type} is not supported."
                            f"The struct {struct_name} contains an {attribute_type}."
                        )
                tables.write(f"            update_row.execute(connection)\n")
                tables.write(f"            .await\n")
                tables.write("             .map(|payload| match payload {\n")
                tables.write(
                    "                 gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,\n"
                )
                tables.write(
                    '                 _ => unreachable!("Expected Payload::Update")\n'
                )
                tables.write("})\n")
                tables.write("    }\n\n")

                # Next, we implement the `update_or_insert` method for the struct. This method
                # inserts the struct into the GlueSQL database if it does not exist, otherwise
                # it updates the struct in the database.
                tables.write(
                    f"    /// Update the struct in the database if it exists, otherwise insert it.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Arguments\n")
                tables.write(
                    f"    /// * `connection` - The connection to the database.\n"
                )
                tables.write(f"    ///\n")
                tables.write(f"    /// # Returns\n")
                tables.write(f"    /// The number of rows updated or inserted.\n")
                tables.write(f"    pub async fn update_or_insert<C>(\n")
                tables.write(f"        self,\n")
                tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(f"    ) -> Result<usize, gluesql::prelude::Error> where\n")
                tables.write(
                    f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n"
                )
                tables.write(f"    {{\n")
                tables.write(
                    f"        let number_of_rows = self.clone().update(connection).await?;\n"
                )
                tables.write(f"        if number_of_rows == 0 {{\n")
                tables.write(f"            self.insert(connection).await\n")
                tables.write(f"        }} else {{\n")
                tables.write(f"            Ok(number_of_rows)\n")
                tables.write(f"        }}\n")
                tables.write(f"    }}\n")

                # We implement the `all` method for the struct. This method returns all of the
                # structs in the GlueSQL database.
                tables.write(f"    /// Get all {struct_name} from the database.\n")
                tables.write(f"    ///\n")
                tables.write(f"    /// # Arguments\n")
                tables.write(f"    /// * `connection` - The connection to the database.\n")
                tables.write(f"    ///\n")
                tables.write(f"    pub async fn all<C>(\n")
                tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                tables.write(f"    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n")
                tables.write(f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n")
                tables.write(f"    {{\n")
                tables.write(f"        use gluesql::core::ast_builder::*;\n")
                tables.write(f'        let select_row = table("{table_name}")\n')
                tables.write(f"            .select()\n")
                tables.write(f"            .project(\"{columns}\")\n")
                tables.write(f"            .execute(connection)\n")
                tables.write(f"            .await?;\n")
                tables.write(f"        Ok(select_row.select()\n")
                tables.write(f"            .unwrap()\n")
                tables.write(f"            .map(Self::from_row)\n")
                tables.write(f"            .collect::<Vec<_>>())\n")
                tables.write(f"    }}\n")

                # We implement the `search` method for the struct. This method searches for the
                # struct in the GlueSQL database by a given query string. It is only implemented
                # for the structs that have the `pg_trgm` index. Of course, since we are in the
                # gluesql part of the implementation, we do not actually have access to the `pg_trgm`
                # index, but we can still implement the method for the structs that have the index.
                # In this case, we search using the Levenshtein distance. This method receives the
                # query string, the limit, the threshold, and the connection to the database. The method
                # returns a vector of instances of the struct found, sorted by the similarity to the query.
                # if similarity_indices.has_table(table_name):
                #     similarity_index = similarity_indices.get_table(table_name)
                #     human_readable_columns = similarity_index.get_human_readable_columns()
                #     columns_to_query = similarity_index.columns
                #     tables.write(f"    /// Search for {struct_name} by a given string.\n")
                #     tables.write(f"    ///\n")
                #     tables.write(f"    /// # Arguments\n")
                #     tables.write(f"    /// * `query` - The string to search for.\n")
                #     tables.write(f"    /// * `limit` - The maximum number of results, by default `10`.\n")
                #     tables.write(f"    /// * `threshold` - The similarity threshold, by default `0.6`.\n")
                #     tables.write(f"    /// * `connection` - The connection to the database.\n")
                #     tables.write(f"    ///\n")
                #     tables.write(f"    /// # Implementative details\n")
                #     tables.write(f"    /// The search is performed using the Levenshtein distance, and the\n")
                #     tables.write(f"    /// columns considered are {human_readable_columns}, as defined in the\n")
                #     tables.write(f"    /// `pg_trgm` index from the PostgreSQL database.\n")
                #     tables.write(f"    /// This is done so that in the offline mode, the search can still be performed,\n")
                #     tables.write(f"    /// altough the `pg_trgm` index is not available in the GlueSQL database and\n")
                #     tables.write(f"    /// therefore we fallback to the Levenshtein distance.\n")
                #     tables.write(f"    ///\n")
                #     tables.write(f"    /// # Returns\n")
                #     tables.write(f"    /// A vector of {struct_name} instances found, sorted by the similarity to the query.\n")
                #     tables.write(f"    pub async fn search<C>(\n")
                #     tables.write(f"        query: &str,\n")
                #     tables.write(f"        limit: Option<i32>,\n")
                #     tables.write(f"        threshold: Option<f64>,\n")
                #     tables.write(f"        connection: &mut gluesql::prelude::Glue<C>,\n")
                #     tables.write(f"    ) -> Result<Vec<Self>, gluesql::prelude::Error> where\n")
                #     tables.write(f"        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,\n")
                #     tables.write(f"    {{\n")
                #     tables.write(f"        use gluesql::core::ast_builder::*;\n")

                # We implement the `from_row` method for the struct. This method
                # receives a row from the GlueSQL database, which is a `HashMap<&str, &&Value>`.
                # The method returns the struct from the row.
                tables.write(
                    f"    pub fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {{\n"
                )
                tables.write("        Self {\n")

                clonables = {
                    "bool": "Bool",
                    "i8": "I8",
                    "i16": "I16",
                    "i32": "I32",
                    "i64": "I64",
                    "i128": "I128",
                    "u8": "U8",
                    "u16": "U16",
                    "u32": "U32",
                    "u64": "U64",
                    "u128": "U128",
                    "f32": "F32",
                    "f64": "F64",
                    "String": "Str",
                    "NaiveDateTime": "Timestamp",
                }

                for attribute in struct_metadata["attributes"]:
                    attribute_type = attribute["field_type"]
                    attribute_name = attribute["field_name"]
                    if attribute_type == "Uuid":
                        tables.write(
                            f'            {attribute_name}: match row.get("{attribute_name}").unwrap() {{\n'
                        )
                        tables.write(
                            f"                gluesql::prelude::Value::Uuid({attribute_name}) => Uuid::from_u128(*{attribute_name}),\n"
                        )
                        tables.write(
                            f'                _ => unreachable!("Expected Uuid"),\n'
                        )
                        tables.write("            },\n")
                    elif attribute_type == "Option<Uuid>":
                        tables.write(
                            f'            {attribute_name}: match row.get("{attribute_name}").unwrap() {{\n'
                        )
                        tables.write(
                            f"                gluesql::prelude::Value::Null => None,\n"
                        )
                        tables.write(
                            f"                gluesql::prelude::Value::Uuid({attribute_name}) => Some(Uuid::from_u128(*{attribute_name})),\n"
                        )
                        tables.write(
                            f'                _ => unreachable!("Expected Uuid"),\n'
                        )
                        tables.write("            },\n")
                    elif (
                        attribute_type in clonables
                        or attribute_type.startswith("Option<")
                        and attribute_type[7:-1] in clonables
                    ):
                        if attribute_type.startswith("Option<"):
                            attribute_type = attribute_type[7:-1]
                            tables.write(
                                f'            {attribute_name}: match row.get("{attribute_name}").unwrap() {{\n'
                            )
                            tables.write(
                                f"                gluesql::prelude::Value::Null => None,\n"
                            )
                            tables.write(
                                f"                gluesql::prelude::Value::{clonables[attribute_type]}({attribute_name}) => Some({attribute_name}.clone()),\n"
                            )
                            tables.write(
                                f'                _ => unreachable!("Expected {clonables[attribute_type]}")\n'
                            )
                            tables.write("            },\n")
                        else:
                            tables.write(
                                f'            {attribute_name}: match row.get("{attribute_name}").unwrap() {{\n'
                            )
                            tables.write(
                                f"                gluesql::prelude::Value::{clonables[attribute_type]}({attribute_name}) => {attribute_name}.clone(),\n"
                            )
                            tables.write(
                                f'                _ => unreachable!("Expected {clonables[attribute_type]}")\n'
                            )
                            tables.write("            },\n")
                    else:
                        raise NotImplementedError(
                            f"Found an unsupported attribute type for the struct {struct_name}: {attribute_type} "
                            f"for the attribute {attribute_name}."
                        )
                tables.write("        }\n")
                tables.write("    }\n")

                # And finally we close the struct implementation
                tables.write("}\n")

    # We create the Table enumeration, containing all
    # the table names. We also implement the `table_name`
    # method for the enumeration, returning the table name
    # for each of the structs.
    tables.write("\n")

    derives_for_enum = [
        "Deserialize",
        "Serialize",
        "Clone",
        "Debug",
        "PartialEq",
    ]

    lower_enumeration = enumeration.lower()

    tables.write(f"#[derive(")
    for derive in derives_for_enum + ["Copy", "Eq"]:
        tables.write(f"{derive}, ")
    tables.write(f")]\n")
    tables.write(f"pub enum {enumeration} {{\n")
    for struct_name in table_names.keys():
        tables.write(f"    {struct_name},\n")
    tables.write("}\n\n")

    tables.write(f"impl {enumeration} {{\n")
    tables.write("    pub fn name(&self) -> &'static str {\n")
    tables.write("        match self {\n")
    for struct_name, table_data in table_names.items():
        table_name = table_data["table_name"]
        tables.write(f'            {enumeration}::{struct_name} => "{table_name}",\n')
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("}\n")

    # We implement Display for the enumeration
    tables.write(f"impl std::fmt::Display for {enumeration} {{\n")
    tables.write(
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n"
    )
    tables.write('        write!(f, "{}", self.name())\n')
    tables.write("    }\n")
    tables.write("}\n")

    # Finally, we write an enum for the rows in the tables (or views)

    tables.write("\n")
    tables.write(f"#[derive(")
    for derive in derives:
        tables.write(f"{derive}, ")
    tables.write(f")]\n")
    tables.write(f"pub enum {enumeration}Row {{\n")
    for struct_name in table_names.keys():
        tables.write(f"    {struct_name}({struct_name}),\n")
    tables.write("}\n\n")

    # We also implement the From<&str> trait for the enumeration
    tables.write(f"impl From<&str> for {enumeration} {{\n")
    tables.write("    fn from(item: &str) -> Self {\n")
    tables.write("        match item {\n")
    for struct_name, table_data in table_names.items():
        table_name = table_data["table_name"]
        tables.write(f'            "{table_name}" => {enumeration}::{struct_name},\n')
    tables.write('            _ => panic!("Unknown table name"),\n')
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("}\n")

    tables.write(f"impl {enumeration}Row {{\n")
    tables.write(
        f"    pub fn {lower_enumeration}(&self) -> &'static {enumeration} {{\n"
    )
    tables.write("        match self {\n")
    for struct_name in table_names.keys():
        tables.write(
            f"            {enumeration}Row::{struct_name}(_) => &{enumeration}::{struct_name},\n"
        )
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("\n")
    tables.write(f"    pub fn {lower_enumeration}_name(&self) -> &'static str {{\n")
    tables.write(f"        self.{lower_enumeration}().name()\n")
    tables.write("    }\n")
    tables.write("}\n")

    # We implement the web_common version of the SearchableTable or SearchableView
    # enumeration. This enumeration contains all the tables or views that implement
    # the `pg_trgm` index. The bidirectional conversion between the SearchableTable
    # or SearchableView and their backend version that actually provides the Diesel
    # methods are solely available in the backend.

    has_any_searchables = any(
        similarity_indices.has_table(table_data["table_name"])
        for table_data in table_names.values()
    )

    if has_any_searchables:
        tables.write("\n")

        tables.write(f"#[derive(")
        for derive in derives + ["Copy", "Eq"]:
            tables.write(f"{derive}, ")
        tables.write(f")]\n")
        tables.write(f"pub enum Searcheable{enumeration} {{\n")
        for struct_name in table_names.keys():
            if similarity_indices.has_table(table_names[struct_name]["table_name"]):
                tables.write(f"    {struct_name},\n")
        tables.write("}\n\n")

    tables.close()

    return table_names


def get_view_names() -> List[str]:
    """Returns list of view names.

    Implementative details
    -------------------------
    The view names are extracted from the `migrations` directory. The view names are extracted
    from the `up.sql` file in each of the directories. The view names are extracted by searching
    for the `CREATE VIEW` statements in the SQL file.
    """
    view_names = []
    for directory in os.listdir("migrations"):
        if not os.path.isdir(f"migrations/{directory}"):
            continue
        with open(f"migrations/{directory}/up.sql", "r") as file:
            content = file.read()
        for line in content.split("\n"):
            if "CREATE VIEW" in line:
                view_name = line.split(" ")[2]
                view_names.append(view_name)
    return view_names


def get_views(cursor):
    cursor.execute(
        "SELECT table_name FROM information_schema.views WHERE table_schema = 'public';"
    )
    views = cursor.fetchall()
    return views


def get_view_columns(cursor, view_name):
    cursor.execute(
        f"SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '{view_name}' AND table_schema = 'public';"
    )
    columns = cursor.fetchall()
    return columns


def map_postgres_to_rust_type(pg_type):
    pg_to_rust_types = {
        "uuid": "diesel::sql_types::Uuid",
        "text": "diesel::sql_types::Text",
        "timestamp without time zone": "diesel::sql_types::Timestamp",
        "character varying": "diesel::sql_types::Text",
        "integer": "diesel::sql_types::Integer",
    }

    if pg_type in pg_to_rust_types:
        return pg_to_rust_types[pg_type]

    raise NotImplementedError(f'Postgres type "{pg_type}" is not supported.')


def generate_diesel_schema(view_name, columns):
    schema_code = "diesel::table! {\n"
    schema_code += f"    {view_name} (id) {{\n"
    for column in columns:
        schema_code += (
            f"        {column[0]} -> {map_postgres_to_rust_type(column[1])},\n"
        )
    schema_code += "    }\n"
    schema_code += "}\n"
    return schema_code


def generate_view_schema():
    """Generate the view schema.

    Implementative details
    -------------------------
    We generate the views by connecting to the database and querying the `information_schema`
    tables. We then write the views to the file `src/views/schema.rs`. The database is a postgres
    database, and the connection string is read from the environment variable `DATABASE_URL`.
    """
    # We load the data from the environment variables from the `.env` file
    # at `../.env`.
    conn, cursor = get_cursor()

    # Getting the list of views
    views = get_views(cursor)

    # We open the file to write the schema
    schema_file = open("src/views/schema.rs", "w")

    # Generating Diesel schema for each view
    for view in views:
        view_name = view[0]
        columns = get_view_columns(cursor, view_name)
        schema_code = generate_diesel_schema(view_name, columns)
        schema_file.write(schema_code + "\n")

    # Closing the cursor and connection
    cursor.close()
    conn.close()


def check_schema_completion():
    """Check the view schema completion.

    Implementative details
    -------------------------
    Diesel does not support the `CREATE VIEW` statements, and as such we need to manually
    create the views in the schema file `src/views/schema.rs`. This script check that all
    of the view names are present in the schema file.
    """
    view_names = get_view_names()
    with open("src/views/schema.rs", "r") as file:
        content = file.read()
    for view_name in view_names:
        if view_name not in content:
            raise NotImplementedError(
                f"View {view_name} is not present in the schema file."
            )


def generate_view_structs():
    """Generate the view structs.

    Implementative details
    -------------------------
    Since Diesel solely supports the generation of the table structs, we need to use
    this custom script to generate the view structs. The view structs are generated
    starting from the schema file `src/views/schema.rs` and are written to the file
    `src/views/views.rs`. The view structs are generated by copying the views structs
    defined in the views schema, with the data types appropriately changed to match the
    view schema.

    An example of a schema entry is:

    ```rust
    diesel::table! {
        users_view (id) {
            id -> Uuid,
            first_name -> Nullable<Text>,
            middle_name -> Nullable<Text>,
            last_name -> Nullable<Text>,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
    ```
    """
    with open("src/views/schema.rs", "r") as file:
        schema = file.read()

    data_types = {
        "diesel::sql_types::Uuid": "Uuid",
        "diesel::sql_types::Text": "String",
        "diesel::sql_types::Timestamp": "NaiveDateTime",
        "diesel::sql_types::Integer": "i32",
    }

    imports = [
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use uuid::Uuid;",
        "use chrono::NaiveDateTime;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::prelude::*;",
        "use crate::views::schema::*;",
    ]

    derives = [
        "Deserialize",
        "Serialize",
        "Clone",
        "Debug",
        "PartialEq",
        "Queryable",
        "QueryableByName",
    ]

    views = open("src/views/views.rs", "w")

    for import_statement in imports:
        views.write(f"{import_statement}\n")

    last_line_was_table = False
    view_name = None
    # A dictionary to store the view attributes and
    # their respective types.
    attributes = {}
    brackets_count = 0

    view_names = []

    for line in schema.split("\n"):
        if "{" in line:
            brackets_count += 1
        if "}" in line:
            brackets_count -= 1

        if last_line_was_table:
            view_name = line.split("(")[0].strip(" ")
            view_names.append(view_name)

        if "diesel::table! {" in line:
            last_line_was_table = True
            continue
        else:
            last_line_was_table = False

        if "->" in line:
            (attribute, data_type) = line.strip(" ,").split(" -> ")
            attributes[attribute] = data_type

        # If we have found the end of the struct, we write the struct
        # to the file.
        if brackets_count == 0 and view_name is not None:
            # First, we generate the derives.
            views.write("#[derive(")
            for derive in derives:
                views.write(f"{derive}, ")
            views.write(")]\n")

            # Then, we write the table_name attribute to link
            # the struct to the view.
            views.write(f"#[diesel(table_name = {view_name})]\n")

            # We convert the view name, which is in snake case, to
            # the struct name, which is in camel case.
            view_name_parts = view_name.split("_")
            view_name_parts = [part.capitalize() for part in view_name_parts]
            view_struct = "".join(view_name_parts)

            # We write the struct definition
            views.write(f"pub struct {view_struct} {{\n")
            for attribute, data_type in attributes.items():
                views.write(f"    pub {attribute}: {data_types[data_type]},\n")
            views.write("}\n\n")

        if brackets_count == 0:
            attributes = {}
            view_name = None

    view_names_from_sql = get_view_names()
    for view_name in view_names_from_sql:
        assert (
            view_name in view_names
        ), f'View "{view_name}" is not present in the "schema.rs" file.'

    views.close()


def main():
    # Read the replacements from the JSON file
    replacements = compress_json.local_load("replacements.json")

    # We make sure the migrations were fully executed
    status = os.system("diesel migration run")

    if status != 0:
        raise Exception("The migrations were not fully executed.")

    # We run the diesel extended CLI command
    status = os.system("diesel_ext --model --add-table-name > src/models.rs")

    if status != 0:
        raise Exception("The diesel_ext command failed.")

    # Read the generated file
    with open("src/models.rs", "r") as file:
        content = file.read()

    # Imports to always add to the file
    imports = [
        "use diesel::Queryable;",
        "use diesel::QueryableByName;",
        "use diesel::Identifiable;",
        "use diesel::Insertable;",
        "use crate::schema::*;",
        "use diesel::Selectable;",
        "use serde::Deserialize;",
        "use serde::Serialize;",
        "use diesel::r2d2::ConnectionManager;",
        "use diesel::r2d2::PooledConnection;",
        "use diesel::prelude::*;",
    ]

    for import_statement in imports:
        if import_statement not in content:
            content = content.replace(
                "#![allow(clippy::all)]", f"#![allow(clippy::all)]\n{import_statement}"
            )

    # We need to add some extra derives to the structs
    derives = ["Selectable", "Clone", "PartialEq"]

    for derive in derives:
        content = content.replace("#[derive(", f"#[derive({derive}, ")

    # Apply the replacements
    for replacement in replacements:
        if replacement["search"] in content:
            content = content.replace(replacement["search"], replacement["replace"])
            for import_statement in replacement["imports"]:
                if import_statement not in content:

                    # The import statements must be added on the top of the file,
                    # but below the eventual macros and attributes. We can refer
                    # to the following macros as an anchor point, which are characterized
                    # by the `#![...]` syntax.
                    #
                    # ```rust
                    # #![allow(unused)]
                    # #![allow(clippy::all)]
                    # ```
                    #
                    # We then insert the import statements right after the anchor point.
                    content = content.replace(
                        "#![allow(clippy::all)]",
                        f"#![allow(clippy::all)]\n{import_statement}",
                    )

    complex_derives = [
        "Serialize",
        "Deserialize",
        "Insertable",
        "PartialEq",
        "QueryableByName",
    ]

    deny_list = ["Interval", "Range<Numeric>", "Money"]

    # Some derives are more complex, and we only add them if within
    # the struct we are currently processing there is NOT in the deny list.
    # A struct is defined from where the `struct` keyword is found until the
    # next `}` is found.
    new_content = ""
    currently_in_struct = False

    for line_number, line in enumerate(content.split("\n")):
        if "#[derive(" in line:
            currently_in_struct = True
        if currently_in_struct:
            # We execute a look ahead to see if we find any of the
            # types in the deny list in the next lines up until
            # we find the closing bracket of the struct.
            found_deny = False

            for look_ahead_line in content.split("\n")[line_number:]:
                if "}" in look_ahead_line:
                    currently_in_struct = False
                    break
                for deny in deny_list:
                    if deny in look_ahead_line:
                        found_deny = True
                        break
            if not found_deny:
                for derive in complex_derives:
                    if derive not in line:
                        line = line.replace("#[derive(", f"#[derive({derive}, ")

        new_content += line + "\n"
    content = new_content

    # Write the file back
    with open("src/models.rs", "w") as file:
        file.write(content)


if __name__ == "__main__":
    # Load dotenv file
    load_dotenv()

    # We make sure that the ./db_data/taxons.tsv file is present
    # or otherwise we run the script to generate it.
    if not os.path.exists("./db_data/taxons.tsv"):
        retrieve_ncbi_taxon()

    # If there is a "__pycache__" directory, we remove it as Diesel
    # seems to be keen to try and run it as a migration, and it will
    # fail.
    if os.path.exists("__pycache__"):
        shutil.rmtree("__pycache__")
    
    if os.path.exists("migrations/__pycache__"):
        shutil.rmtree("migrations/__pycache__")

    main()
    generate_view_schema()
    check_schema_completion()
    generate_view_structs()
    table_structs: Dict[str, Dict[str, str]] = write_web_common_structs(
        "src/models.rs", "tables", "Table"
    )
    view_structs: Dict[str, Dict[str, str]] = write_web_common_structs(
        "src/views/views.rs", "views", "View"
    )
    write_from_impls("src/models.rs", "tables", table_structs)
    write_from_impls("src/views/views.rs", "views", view_structs)
