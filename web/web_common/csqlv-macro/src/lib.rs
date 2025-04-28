#![doc = include_str!("../README.md")]
extern crate proc_macro;

use std::path::Path;

use csqlv::{CSVSchemaBuilder, SQLGenerationOptions};
use macro_utils::{cached_file_path, file_last_modified_time, most_recent_file};
use proc_macro::TokenStream;
use quote::quote;
use syn::{ExprTuple, parse_macro_input};

#[proc_macro]
/// Generates a function to generate `SQL` from CSV files.
///
/// # Arguments
///
/// * `input` - Either a string literal containing the directory containing the
///   CSV files or a tuple containing the directory and the container directory.
///
/// # Panics
///
/// * If the CSV directory cannot be loaded or if SQL generation fails, the
///   macro will panic.
pub fn load_populating_sql_from_csvs(input: TokenStream) -> TokenStream {
    // Parse the input token stream
    // Parse the input into a tuple expression
    let input = parse_macro_input!(input as ExprTuple);

    // Check the number of elements in the tuple
    let (csv_directory, container_directory): (String, Option<String>) = if input.elems.len() == 1 {
        // Handle the case where there is only one argument
        let first = &input.elems[0];
        let csv_directory: String = match first {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => lit_str.value(),
            _ => {
                return syn::Error::new_spanned(input, "Expected a string literal")
                    .to_compile_error()
                    .into();
            }
        };
        let container_directory = None;
        (csv_directory, container_directory)
    } else if input.elems.len() == 2 {
        // Handle the case where there are two arguments
        let first = &input.elems[0];
        let second = &input.elems[1];
        let csv_directory = match first {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => lit_str.value(),
            _ => {
                return syn::Error::new_spanned(input, "Expected a string literal")
                    .to_compile_error()
                    .into();
            }
        };
        let container_directory = match second {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => lit_str.value(),
            _ => {
                return syn::Error::new_spanned(input, "Expected a string literal")
                    .to_compile_error()
                    .into();
            }
        };
        (csv_directory, Some(container_directory))
    } else {
        // If there are not 1 or 2 arguments, return an error
        return syn::Error::new_spanned(input, "Expected one or two arguments")
            .to_compile_error()
            .into();
    };

    let cargo_toml_directory =
        std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");

    let csv_directory = Path::new(&cargo_toml_directory).join(csv_directory);

    // We check that the provided path is a valid directory
    assert!(csv_directory.is_dir(), "The provided path is not a valid directory.");

    // We get the most recent file in the directory
    let most_recent_file =
        most_recent_file(&csv_directory, &[".csv", ".csv.gz", ".tsv", ".tsv.gz"])
            .expect("The provided directory does not contain any CSV or TSV files.");

    let cached_file = cached_file_path(&csv_directory, "load_sqlite_from_csvs");
    let cached_file_last_modified = file_last_modified_time(&cached_file).unwrap_or(0);
    if cached_file_last_modified >= most_recent_file {
        // If the cached file is up to date, we return it
        let cached_file_content =
            std::fs::read_to_string(&cached_file).expect("Failed to read the cached SQL file");
        return TokenStream::from(quote! {
            #cached_file_content
        });
    }

    let csv_directory_str = csv_directory.to_str().expect("Failed to convert path to string");

    // Load the CSV directory using `csqlv`.
    let mut schema_builder = CSVSchemaBuilder::default().include_gz().singularize();

    if let Some(container_directory) = container_directory {
        // If a container directory is provided, we set it
        schema_builder = schema_builder.container_directory(&container_directory);
    }

    let schema = schema_builder.from_dir(csv_directory_str).expect("Failed to load CSV directory");

    let sql_generation_options: SQLGenerationOptions =
        SQLGenerationOptions::default().include_population();
    let sql = schema.to_sql(&sql_generation_options).expect("Failed to generate SQL");

    // Minify the SQL content
    let minified_document: String = sql_minifier::minify_sql(&sql);

    // Write the minified SQL content to the cached file
    std::fs::write(&cached_file, &minified_document).expect("Failed to write the cached SQL file");

    // Return the minified SQL content
    TokenStream::from(quote! {
        #minified_document
    })
}
