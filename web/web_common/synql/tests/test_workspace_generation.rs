//! Test module to verify workspace generation in a `tmp` directory.
//!
//! The test includes generating a SQL-based workspace and checking
//! that the expected files pass a workspace-level `cargo check` and
//! `cargo test`, as some of the generated code may tests, and finally
//! cleaning up the temporary directory after the tests complete.

use std::process::Command;

use ::graph::prelude::DirectoryTree;
use synql::prelude::*;

#[test]
fn test_workspace_generation() -> Result<(), Box<dyn std::error::Error>> {
    let db = ParserDB::try_from(
        r#"
		CREATE TABLE users (
		    id SERIAL PRIMARY KEY,
		    name TEXT NOT NULL,
		    email TEXT UNIQUE NOT NULL
		);
        CREATE TABLE comments (
		    id SERIAL PRIMARY KEY,
		    name TEXT NOT NULL,
            user_id INT REFERENCES users(id)
		);
	"#,
    )?;
    let temp_dir = tempfile::tempdir().expect("Unable to create temporary directory");
    // let workspace_path = temp_dir.path().join("synql_workspace");
    let workspace_path = std::path::PathBuf::from("../../../../local");

    let synql = SynQL::new()
        .database(&db)
        .path(workspace_path.as_path())
        .generate_workspace_toml()
        .generate_rustfmt()
        .build()
        .expect("Unable to build SynQL instance");
    synql.generate().expect("Unable to generate workspace");

    // Load the path tree to see if it matches expectation using `insta`
    let directory_tree = DirectoryTree::from(workspace_path.clone());

    // Convert to string and replace the temp directory path with a placeholder
    let tree_output = directory_tree.to_string();
    let normalized_output = tree_output
        .replace(&workspace_path.to_string_lossy().to_string(), "[TEMP_DIR]/synql_workspace");

    insta::assert_snapshot!("synql_workspace", normalized_output);

    // Verify that the workspace directory was created
    assert!(workspace_path.exists(), "Workspace directory should be created");

    // Verify that Cargo.toml exists
    let cargo_toml = workspace_path.join("Cargo.toml");
    assert!(cargo_toml.exists(), "Cargo.toml should be created");

    // Verify that the generated workspace can be checked
    let output = Command::new("cargo").arg("check").current_dir(&workspace_path).output()?;

    if !output.status.success() {
        eprintln!("cargo check stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo check stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo check failed for generated workspace");
    }

    // Verify that the generated workspace can be tested
    let output = Command::new("cargo").arg("test").current_dir(&workspace_path).output()?;

    if !output.status.success() {
        eprintln!("cargo test stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo test stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo test failed for generated workspace");
    }

    // Runs the `cargo fmt` command in the specified directory.
    let output = Command::new("cargo").arg("fmt").current_dir(&workspace_path).output()?;

    if !output.status.success() {
        eprintln!("cargo fmt stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo fmt stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo fmt failed for generated workspace");
    }

    Ok(())
}
