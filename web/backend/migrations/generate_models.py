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
import compress_json
import os

def write_from_impls():
    """Write the `From` implementations for the structs in the `src/models.rs` file."""
    with open('src/models.rs', 'r') as file:
        content = file.read()
    
    # After each struct ends, as defined by the `}` character, after
    # we have found a `struct` keyword, we write the `From` implementation
    # for the struct where we implement the conversion to the struct in the
    # `web_common` crate.

    impl_from_line = "impl From<{struct_name}> for web_common::database::tables::{struct_name} {{\n"
    reverse_from = "impl From<web_common::database::tables::{struct_name}> for {struct_name} {{\n"

    struct_name = None
    struct_field_names = []
    new_content = ""

    for line in content.split('\n'):
        new_content += line + '\n'

        if 'struct' in line:
            # We have found a line such as "pub struct Archivable {"
            # and we need to extract the struct name.
            struct_name = line.split(' ')[2]
        
        # We are currently inside a struct
        if struct_name is not None:
            # And we have found the end of the struct
            if '}' in line:
                # We have found the end of the struct, and we write the
                # `From` implementation.
                new_content += "\n"
                new_content += impl_from_line.format(struct_name=struct_name)
                new_content += f"    fn from(item: {struct_name}) -> Self {{\n"
                new_content += "        Self {\n"
                for field_name in struct_field_names:
                    new_content += f"            {field_name}: item.{field_name},\n"
                new_content += "        }\n"
                new_content += "    }\n"
                new_content += "}\n\n"

                new_content += reverse_from.format(struct_name=struct_name)
                new_content += f"    fn from(item: web_common::database::tables::{struct_name}) -> Self {{\n"
                new_content += "        Self {\n"
                for field_name in struct_field_names:
                    new_content += f"            {field_name}: item.{field_name},\n"
                new_content += "        }\n"
                new_content += "    }\n"
                new_content += "}\n\n"                

                struct_name = None
                struct_field_names = []

        # We are currently inside a struct, and we are looking for
        # the field names.
        if struct_name is not None and 'pub' in line and ':' in line:
            # We have found a line such as `pub name: String,`
            # and we need to extract the field name.
            field_name = line.strip().split(' ')[1].strip(':')
            struct_field_names.append(field_name)
        
    with open('src/models.rs', 'w') as file:
        file.write(new_content)

def write_web_common_structs(
    path: str,
    target: str,
    enumeration: str
):
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
        "use chrono::DateTime;",
        "use chrono::Utc;"
    ]

    # The derives to apply to the structs in the `src/database/tables.rs` document
    derives = [
        "Deserialize",
        "Serialize",
        "Clone",
        "Debug",
        "PartialEq"
    ]

    # We check that we are currently executing in the `backend` crate
    # so to make sure that the relative path to the `web_common` crate
    # is correct.
    if not os.getcwd().endswith('backend'):
        raise Exception("This script must be executed in the `backend` crate.")

    tables = open(f"../web_common/src/database/{target}.rs", 'w')

    with open(path, 'r') as file:
        models = file.read()

    for import_statement in imports:
        struct_imported = import_statement.split(':')[-1].strip(';')
        if struct_imported not in models:
            continue
        tables.write(f"{import_statement}\n")

    inside_struct = False

    # A dictionary to store the table names and their
    # respective structs.
    table_names = {}
    last_table_name = None

    for line in models.split('\n'):
        # We skip all lines beginning with `//!` as they are comments
        if line.startswith('//!'):
            continue

        # We find the table name by searching lines like
        # #[diesel(table_name = item_continuous_quantities)]
        if 'table_name =' in line:
            last_table_name = line.split('=')[1].strip(" )]")

        # We determine whether a new struct has started
        # by checking if the `struct` keyword is present
        # in the line.
        if 'struct' in line:
            struct_name = line.split(' ')[2]
            table_names[struct_name] = last_table_name

            inside_struct = True
            # If we have just started a new struct, we need to
            # write the `#[derive(...)]` decorator.
            tables.write(f"#[derive(")
            for derive in derives:
                tables.write(f"{derive}, ")
            tables.write(f")]\n")

        if inside_struct:
            # We determine whether the struct has ended
            # by checking if the `}` keyword is present
            # in the line.
            if '}' in line:
                inside_struct = False

            # We write the line to the file
            tables.write(f"{line}\n")
        
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
        "Copy",
        "Eq"
    ]

    lower_enumeration = enumeration.lower()

    tables.write(f"#[derive(")
    for derive in derives_for_enum:
        tables.write(f"{derive}, ")
    tables.write(f")]\n")
    tables.write(f"pub enum {enumeration} {{\n")
    for struct_name in table_names.keys():
        tables.write(f"    {struct_name},\n")
    tables.write("}\n\n")

    tables.write(f"impl {enumeration} {{\n")
    tables.write("    pub fn name(&self) -> &'static str {\n")
    tables.write("        match self {\n")
    for struct_name, table_name in table_names.items():
        tables.write(f"            {enumeration}::{struct_name} => \"{table_name}\",\n")
    tables.write("        }\n")
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
    for struct_name, table_name in table_names.items():
        tables.write(f"            \"{table_name}\" => {enumeration}::{struct_name},\n")
    tables.write("            _ => panic!(\"Unknown table name\"),\n")
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("}\n")

    tables.write(f"impl {enumeration}Row {{\n")
    tables.write(f"    pub fn {lower_enumeration}(&self) -> &'static {enumeration} {{\n")
    tables.write("        match self {\n")
    for struct_name in table_names.keys():
        tables.write(f"            {enumeration}Row::{struct_name}(_) => &{enumeration}::{struct_name},\n")
    tables.write("        }\n")
    tables.write("    }\n")
    tables.write("\n")
    tables.write(f"    pub fn {lower_enumeration}_name(&self) -> &'static str {{\n")
    tables.write(f"        self.{lower_enumeration}().name()\n")
    tables.write("    }\n")
    tables.write("}\n")

    tables.close()

def main():
    # Read the replacements from the JSON file
    replacements = compress_json.local_load('replacements.json')

    # We make sure the migrations were fully executed
    os.system('diesel migration run')

    # We run the diesel extended CLI command
    os.system('diesel_ext --model --add-table-name > src/models.rs')

    # Read the generated file
    with open('src/models.rs', 'r') as file:
        content = file.read()

    # Imports to always add to the file
    imports = [
        "use diesel::Queryable;",
        "use diesel::Identifiable;",
        "use diesel::Insertable;",
        "use crate::schema::*;",
        "use diesel::Selectable;",
        "use serde::Deserialize;",
        "use serde::Serialize;",
    ]

    for import_statement in imports:
        if import_statement not in content:
            content = content.replace(
                "#![allow(clippy::all)]",
                f"#![allow(clippy::all)]\n{import_statement}"
            )

    # We need to add some extra derives to the structs
    derives = [
        "Selectable",
        "Clone",
    ]

    for derive in derives:
        content = content.replace(
            "#[derive(",
            f"#[derive({derive}, "
        )

    # Apply the replacements
    for replacement in replacements:
        if replacement['search'] in content:
            content = content.replace(
                replacement['search'],
                replacement['replace']
            )
            for import_statement in replacement['imports']:
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
                        f"#![allow(clippy::all)]\n{import_statement}"
                    )

    complex_derives = [
        "Serialize",
        "Deserialize",
        "Insertable"
    ]

    deny_list = [
        "Interval",
        "Range<Numeric>",
        "Money"
    ]

    # Some derives are more complex, and we only add them if within
    # the struct we are currently processing there is NOT in the deny list.
    # A struct is defined from where the `struct` keyword is found until the
    # next `}` is found.
    new_content = ""
    currently_in_struct = False

    for line_number, line in enumerate(content.split('\n')):
        if '#[derive(' in line:
            currently_in_struct = True
        if currently_in_struct:
            # We execute a look ahead to see if we find any of the
            # types in the deny list in the next lines up until
            # we find the closing bracket of the struct.
            found_deny = False

            for look_ahead_line in content.split('\n')[line_number:]:
                if '}' in look_ahead_line:
                    currently_in_struct = False
                    break
                for deny in deny_list:
                    if deny in look_ahead_line:
                        found_deny = True
                        break
            if not found_deny:
                for derive in complex_derives:
                    if derive not in line:
                        line = line.replace(
                            "#[derive(",
                            f"#[derive({derive}, "
                        )

        new_content += line + '\n'
    content = new_content

    # Write the file back
    with open('src/models.rs', 'w') as file:
        file.write(content)

    # Finally, we format the file
    os.system('rustfmt src/models.rs')

if __name__ == '__main__':
    main()
    write_web_common_structs('src/models.rs', 'tables', 'Table')
    write_web_common_structs('src/views/views.rs', 'views', 'View')
    write_from_impls()
    # Finally, we format the file
    os.system('rustfmt src/models.rs')
    os.system('rustfmt ../web_common/src/database/tables.rs')
    os.system('rustfmt ../web_common/src/database/views.rs')
