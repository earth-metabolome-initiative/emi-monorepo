#![doc = include_str!("../README.md")]
extern crate proc_macro;

use std::path::Path;

use csqlv::{CSVSchemaBuilder, SQLGenerationOptions};
use pg2sqlite::prelude::Pg2Sqlite;
use proc_macro::TokenStream;
use quote::quote;
use sqlparser::ast::Statement;

#[proc_macro]
/// Generates a function to load a `SQLite` database from CSV files.
///
/// # Arguments
///
/// * `csv_directory` - A string literal representing the directory containing
///   the CSV files.
///
/// # Panics
///
/// * If the CSV directory cannot be loaded or if SQL generation fails, the
///   macro will panic.
pub fn load_sqlite_from_csvs(csv_directory: TokenStream) -> TokenStream {
    // Parse the input token stream
    let csv_directory: String = syn::parse_macro_input!(csv_directory as syn::LitStr).value();

    let cargo_toml_directory =
        std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");

    let csv_directory = Path::new(&cargo_toml_directory).join(csv_directory);

    // We check that the provided path is a valid directory
    assert!(csv_directory.is_dir(), "The provided path is not a valid directory.");

    // We convert the path to a string
    let csv_directory = csv_directory.to_str().expect("Failed to convert path to string");

    // Load the CSV directory using `csqlv`.
    let schema = CSVSchemaBuilder::default()
        .include_gz()
        .singularize()
        .from_dir(csv_directory)
        .expect("Failed to load CSV directory");

    let sql_generation_options: SQLGenerationOptions = SQLGenerationOptions::default();
    let sql = schema.to_sql(&sql_generation_options).expect("Failed to generate SQL");

    let translated_sql: Vec<Statement> = Pg2Sqlite::default()
        .remove_unsupported_check_constraints()
        .verbose()
        .sql(&sql)
        .expect("Failed to parse SQL")
        .translate()
        .expect("Failed to translate SQL");

    let translated_sql: String = translated_sql
        .into_iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join(";\n");

    // Minify the SQL content
    let minified_document: String = sql_minifier::minify_sql(&translated_sql);

    // Return the minified SQL content
    TokenStream::from(quote! {
        #minified_document
    })
}

#[proc_macro]
/// Generates a function to load a `SQLite` database from `PostgreSQL` SQL
/// migrations.
///
/// # Arguments
///
/// * `migrations_directory` - A string literal representing the directory
///   containing the SQL migrations.
///
/// # Panics
///
/// * If the CSV directory cannot be loaded or if SQL generation fails, the
///   macro will panic.
pub fn load_sqlite_from_migrations(migrations_directory: TokenStream) -> TokenStream {
    // Parse the input token stream
    let migrations_directory: String =
        syn::parse_macro_input!(migrations_directory as syn::LitStr).value();

    let cargo_toml_directory =
        std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");

    let migrations_directory = Path::new(&cargo_toml_directory).join(migrations_directory);

    // We check that the provided path is a valid directory
    assert!(migrations_directory.is_dir(), "The provided path is not a valid directory.");

    // We convert the path to a string
    let migrations_directory =
        migrations_directory.to_str().expect("Failed to convert path to string");

    let translated_sql: Vec<Statement> = Pg2Sqlite::default()
        .remove_unsupported_check_constraints()
        .verbose()
        .ups(migrations_directory)
        .expect("Failed to parse SQL")
        .translate()
        .expect("Failed to translate SQL");

    let translated_sql: String = translated_sql
        .into_iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join(";\n");

    // Minify the SQL content
    let minified_document: String = sql_minifier::minify_sql(&translated_sql);

    // Return the minified SQL content
    TokenStream::from(quote! {
        #minified_document
    })
}
