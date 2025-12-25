//! Test module to verify mandatory triangular same-as relationship decoration.

use sql_traits::prelude::ParserDB;
use synql::prelude::*;

#[test]
fn test_mandatory_triangular_same_as() -> Result<(), Box<dyn std::error::Error>> {
    let db = ParserDB::try_from(
        r#"
    CREATE TABLE grandparent (id INT PRIMARY KEY);
    CREATE TABLE parent (id INT PRIMARY KEY REFERENCES grandparent(id));
    CREATE TABLE sibling (id INT PRIMARY KEY, grandparent_id INT REFERENCES grandparent(id), UNIQUE(id, grandparent_id));
    CREATE TABLE child (
        id INT PRIMARY KEY REFERENCES parent(id), 
        sibling_id INT REFERENCES sibling(id),
        FOREIGN KEY (sibling_id, id) REFERENCES sibling(id, grandparent_id)
    );
	"#,
    )?;
    let temp_dir = tempfile::tempdir().expect("Unable to create temporary directory");
    let workspace_path = temp_dir.path().join("synql_mandatory_triangular");

    let synql: SynQL<ParserDB> = SynQL::new(&db, &workspace_path)
        .name("synql-mandatory-triangular")
        .generate_workspace_toml()
        .generate_rustfmt()
        .into();
    synql.generate().expect("Unable to generate workspace");

    // Construct the expected path to the generated file
    // The crate name is constructed from the workspace name and table name
    let child_crate_name = "synql-mandatory-triangular-child";
    let child_rs_path = workspace_path.join(child_crate_name).join("src").join("lib.rs");

    let content = std::fs::read_to_string(&child_rs_path)
        .unwrap_or_else(|e| panic!("Could not read file at {:?}: {}", child_rs_path, e));

    // Normalize content by removing all whitespace to avoid formatting issues
    let normalized_content: String = content.chars().filter(|c| !c.is_whitespace()).collect();
    let expected_decorator = "#[mandatory(synql_mandatory_triangular_sibling::sibling)]";

    assert!(
        normalized_content.contains(&expected_decorator),
        "Child struct should have {} decorator on sibling_id field. Content:\n{}",
        expected_decorator,
        content
    );

    Ok(())
}
