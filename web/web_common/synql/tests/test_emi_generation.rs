//! Test module to verify workspace generation in a `tmp` directory.
//!
//! The test simply verifies that `synql` is able to process successfully
//! the EMI database schema and generate the workspace without errors,
//! cleaning up the temporary directory after the tests complete.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use sql_constraints::prelude::*;
use synql::{
    prelude::*,
    structs::{ExternalCrate, ExternalType},
};
use time_requirements::{prelude::TimeTracker, report::Report, task::Task};

fn report(time_tracker: &TimeTracker) {
    Report::new(time_tracker.clone())
        .write(Path::new("emi_codegen.md"), Path::new("emi_codegen.png"))
        .unwrap();
}

#[test]
fn test_emi_generation() -> Result<(), Box<dyn std::error::Error>> {
    // We get the cargo toml.
    // And we adequately move to the emi-monorepo root.
    let workspace_root = "../../../";
    let mut tracking_test = TimeTracker::new("EMI Workspace Generation Test");

    let task = Task::new("Database Parsing");
    let db = ParserDB::try_from(
        [
            PathBuf::from(format!("{workspace_root}/web/web_common")).as_path(),
            PathBuf::from(format!("{workspace_root}/data_migrations/init_db/csvs")).as_path(),
            PathBuf::from(format!("{workspace_root}/data_migrations/init_db/extension_migrations"))
                .as_path(),
            PathBuf::from(format!("{workspace_root}/data_migrations/init_db/migrations")).as_path(),
        ]
        .as_slice(),
    )?;
    assert!(db.has_tables(), "Database should have tables");
    tracking_test.add_completed_task(task);

    // Validate the database schema with all available constraints
    let validation_task = Task::new("Schema Validation");
    let constrainer = DefaultConstrainer::<ParserDB>::default();
    constrainer.validate_schema(&db).expect("Database schema should pass all constraints");
    tracking_test.add_completed_task(validation_task);

    // let workspace_path = tempfile::tempdir().expect("Unable to create temporary
    // directory");
    let workspace_path = std::path::PathBuf::from("../../../../emi_local");

    let iso_codes = ExternalCrate::new("iso_codes")
        .unwrap()
        .features(["diesel", "diesel_pgrx"])
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(
            ExternalType::new(
                syn::parse_quote!(iso_codes::CountryCode),
                syn::parse_quote!(iso_codes::country_codes::diesel_impls::CountryCode),
            )
            .postgres_type("countrycode")?
            .into(),
        )
        .unwrap()
        .into();

    let media_types = ExternalCrate::new("media_types")
        .unwrap()
        .features(["diesel", "diesel_pgrx"])
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(
            ExternalType::new(
                syn::parse_quote!(media_types::MediaType),
                syn::parse_quote!(media_types::diesel_impls::MediaType),
            )
            .postgres_type("mediatype")?
            .into(),
        )
        .unwrap()
        .into();

    let cas_codes = ExternalCrate::new("cas_codes")
        .unwrap()
        .features(["diesel", "diesel_pgrx"])
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(
            ExternalType::new(
                syn::parse_quote!(cas_codes::CAS),
                syn::parse_quote!(cas_codes::diesel_impls::CAS),
            )
            .postgres_type("cas")?
            .into(),
        )
        .unwrap()
        .into();

    let molecular_formulas = ExternalCrate::new("molecular_formulas")
        .unwrap()
        .features(["diesel", "diesel_pgrx"])
        .version("0.1.0")
        .git("https://github.com/earth-metabolome-initiative/emi-monorepo", "postgres-crate")
        .add_type(
            ExternalType::new(
                syn::parse_quote!(molecular_formulas::MolecularFormula),
                syn::parse_quote!(
                    molecular_formulas::molecular_formula::diesel_impls::MolecularFormula
                ),
            )
            .postgres_type("molecularformula")?
            .into(),
        )
        .unwrap()
        .into();

    let synql: SynQL<ParserDB> = SynQL::new(&db, &workspace_path)
        .name("synql")
        .external_crates([iso_codes, media_types, cas_codes, molecular_formulas])
        .generate_workspace_toml()
        .generate_rustfmt()
        .into();
    tracking_test.extend(synql.generate().expect("Unable to generate workspace"));

    // We print the report
    report(&tracking_test);

    // Verify that the workspace directory was created
    assert!(workspace_path.exists(), "Workspace directory should be created");

    // Verify that Cargo.toml exists
    let cargo_toml = workspace_path.join("Cargo.toml");
    assert!(cargo_toml.exists(), "Cargo.toml should be created");

    let fmt_task = Task::new("Formatting Workspace");
    // Runs the `cargo fmt` command in the specified directory.
    let output = Command::new("cargo").arg("fmt").current_dir(&workspace_path).output()?;
    tracking_test.add_completed_task(fmt_task);
    report(&tracking_test);

    if !output.status.success() {
        eprintln!("cargo fmt stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo fmt stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo fmt failed for generated workspace");
    }

    let check_task = Task::new("Checking Generated Workspace");
    // Verify that the generated workspace can be checked
    let output = Command::new("cargo").arg("check").current_dir(&workspace_path).output()?;
    tracking_test.add_completed_task(check_task);
    report(&tracking_test);

    if !output.status.success() {
        eprintln!("cargo check stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("cargo check stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("cargo check failed for generated workspace");
    }

    Ok(())
}
