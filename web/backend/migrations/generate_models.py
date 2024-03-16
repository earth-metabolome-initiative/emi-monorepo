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

"""
import compress_json
import os

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