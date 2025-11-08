//! Test module to verify workspace generation in a `tmp` directory.
//!
//! The test simply verifies that `synql` is able to process successfully
//! the EMI database schema and generate the workspace without errors,
//! cleaning up the temporary directory after the tests complete.

use std::{path::PathBuf, process::Command, sync::Arc};

use ::graph::prelude::DirectoryTree;
use synql::prelude::*;
use synql_core::structs::{ExternalCrate, ExternalType};

#[test]
fn test_emi_generation() -> Result<(), Box<dyn std::error::Error>> {
    // We get the cargo toml.
    let mut workspace_root = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable not set");

    // And we adequately move to the emi-monorepo root.
    workspace_root.push_str("/../../..");

    let db = ParserDB::try_from(
        [
            PathBuf::from(format!("{workspace_root}/data_migrations/init_db/csvs")).as_path(),
            PathBuf::from(format!("{workspace_root}/data_migrations/init_db/extension_migrations"))
                .as_path(),
            PathBuf::from(format!("{workspace_root}/data_migrations/init_db/migrations")).as_path(),
        ]
        .as_slice(),
    )?;

    assert!(db.has_tables(), "Database should have tables");

    let temp_dir = tempfile::tempdir().expect("Unable to create temporary directory");
    // let workspace_path = temp_dir.path().join("synql_workspace");
    let workspace_path = std::path::PathBuf::from("../../../../emi_local");

    let iso_codes = ExternalCrate::new()
        .name("iso_codes")?
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(Arc::new(
            ExternalType::new()
                .rust_type(syn::parse_quote!(iso_codes::CountryCode))
                .diesel_type(syn::parse_quote!(iso_codes::country_codes::diesel_impls::CountryCode))
                .postgres_type("countrycode")?
                .build()?,
        ))
        .unwrap()
        .build()?;

    let media_types = ExternalCrate::new()
        .name("media_types")?
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(Arc::new(
            ExternalType::new()
                .rust_type(syn::parse_quote!(media_types::MediaType))
                .diesel_type(syn::parse_quote!(media_types::media_types::diesel_impls::MediaType))
                .postgres_type("mediatype")?
                .build()?,
        ))
        .unwrap()
        .build()?;

    let cas_codes = ExternalCrate::new()
        .name("cas_codes")?
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(Arc::new(
            ExternalType::new()
                .rust_type(syn::parse_quote!(cas_codes::CAS))
                .diesel_type(syn::parse_quote!(cas_codes::cas_codes::diesel_impls::CAS))
                .postgres_type("cas")?
                .build()?,
        ))
        .unwrap()
        .build()?;

    let molecular_formulas = ExternalCrate::new()
        .name("molecular_formulas")?
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(Arc::new(
            ExternalType::new()
                .rust_type(syn::parse_quote!(molecular_formulas::MolecularFormula))
                .diesel_type(syn::parse_quote!(
                    molecular_formulas::molecular_formulas::diesel_impls::MolecularFormula
                ))
                .postgres_type("molecularformula")?
                .build()?,
        ))
        .unwrap()
        .build()?;

    let synql = SynQL::new()
        .database(&db)
        .external_crates([
            Arc::new(iso_codes),
            Arc::new(media_types),
            Arc::new(cas_codes),
            Arc::new(molecular_formulas),
        ])
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

    // Runs the `cargo fmt` command in the specified directory.
    let output = Command::new("cargo").arg("fmt").current_dir(&workspace_path).output()?;

    if !output.status.success() {
        eprintln!("cargo fmt stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo fmt stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo fmt failed for generated workspace");
    }

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

    // Verify that the generated documentation can be built without errors or
    // warnings
    let output = Command::new("cargo")
        .args(&["doc", "--no-deps", "--document-private-items"])
        .current_dir(&workspace_path)
        .output()?;

    if !output.status.success() {
        eprintln!("cargo doc stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo doc stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo doc failed for generated workspace");
    }

    Ok(())
}
