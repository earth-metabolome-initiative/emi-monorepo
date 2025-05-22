//! Build script for the `elements` crate.

use std::{io::Write, path::PathBuf};

use downloader::Downloader;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Array with the media type root type and the corresponding CSV URL
const CSV_URLS: [(&str, Option<&str>); 11] = [
    ("application", Some("https://www.iana.org/assignments/media-types/application.csv")),
    ("audio", Some("https://www.iana.org/assignments/media-types/audio.csv")),
    ("example", None),
    ("font", Some("https://www.iana.org/assignments/media-types/font.csv")),
    ("haptics", Some("https://www.iana.org/assignments/media-types/haptics.csv")),
    ("image", Some("https://www.iana.org/assignments/media-types/image.csv")),
    ("message", Some("https://www.iana.org/assignments/media-types/message.csv")),
    ("model", Some("https://www.iana.org/assignments/media-types/model.csv")),
    ("multipart", Some("https://www.iana.org/assignments/media-types/multipart.csv")),
    ("text", Some("https://www.iana.org/assignments/media-types/text.csv")),
    ("video", Some("https://www.iana.org/assignments/media-types/video.csv")),
];

#[derive(serde::Deserialize, Debug)]
/// Struct representing a row in any of the CSV files
struct Row {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Template")]
    template: String,
    #[serde(rename = "Reference")]
    _reference: String,
}

#[derive(Debug)]
#[allow(dead_code)]
/// Enum representing the possible errors that can occur while generating the
/// submodules
pub enum Error {
    /// Error while downloading the file
    Download(downloader::DownloaderError),
    /// Error while reading the CSV file
    CSV(csv::Error),
    /// When a subtype template does not start with the root type
    InvalidSubtypeTemplate {
        /// The root type
        root: String,
        /// The subtype template
        template: String,
    },
    /// When parsing a deprecation message fails
    InvalidDeprecationMessage {
        /// The root type
        root: String,
        /// The name of the media type
        name: String,
    },
}

impl From<downloader::DownloaderError> for Error {
    fn from(err: downloader::DownloaderError) -> Self {
        Self::Download(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Self::CSV(err)
    }
}

fn to_const_str_name(name: &str) -> String {
    let mut name = name.replace('-', "_");
    name = name.replace('.', "_");

    // If the name ends with a plus sign, we replace it with "_PLUS"
    if let Some(prefix) = name.strip_suffix('+') {
        name = format!("{prefix}_PLUS");
    }

    name = name.replace('+', "_");
    name = name.to_uppercase();

    // If the name starts with a digit, we need to add an underscore
    if name.chars().next().unwrap_or('0').is_ascii_digit() { format!("MT_{name}") } else { name }
}

fn to_const_name(name: &str) -> Ident {
    Ident::new(&to_const_str_name(name), proc_macro2::Span::call_site())
}

enum Deprecation<'a> {
    /// When the deprecation message is present and it does not
    /// provide any alternative
    CompletelyDeprecated,
    /// When the deprecation message is present and it provides
    /// an alternative
    DeprecatedInFavorOf(&'a str, &'a str),
}

/// Returns the deprecation messages for the given name
///
/// # Implementation details
///
/// Occasionally, the IANA registry contains deprecation messages for certain
/// media types. For instance, at the time of writing, the audio subtype
/// `vnd.qcelp` is marked as deprecated, and therefore the `name` entry in the
/// CSV file contains the following message:
///
/// ```text
/// vnd.qcelp - DEPRECATED in favor of audio/qcelp
/// javascript (OBSOLETED in favor of text/javascript)
/// remote-printing (OBSOLETE)
/// vnd.nokia.n-gage.symbian.install (OBSOLETE; no replacement given)
/// ```
///
/// This function parses the deprecation message and returns the new name.
///
/// # Arguments
///
/// * `root` - The root type of the media type
/// * `name` - The name of the media type
fn parse_deprecation_messages<'a>(
    root: &'a str,
    name: &'a str,
) -> Result<Option<Deprecation<'a>>, Error> {
    if !name.contains("DEPRECATED") && !name.contains("OBSOLETED") && !name.contains("OBSOLETE") {
        return Ok(None);
    }

    // If the name ends with `- DEPRECATED`, there is no new name
    if (name.ends_with("DEPRECATED")
        || name.contains("DEPRECATED by")
        || name.contains("OBSOLETED by")
        || name.contains("(OBSOLETE"))
        && !name.contains("in favor of")
    {
        return Ok(Some(Deprecation::CompletelyDeprecated));
    }

    let trimmed_name = name.trim_end_matches(")");

    let parts: Vec<&str> = trimmed_name.split(" in favor of ").collect();

    if parts.len() != 2 {
        return Err(Error::InvalidDeprecationMessage {
            root: root.to_string(),
            name: name.to_string(),
        });
    }

    let new_name = parts[1].trim();

    // Check if the new name starts with the root type
    for (prefix, _url) in CSV_URLS {
        if let Some(new_name) = new_name.strip_prefix(&format!("{prefix}/")) {
            return Ok(Some(Deprecation::DeprecatedInFavorOf(prefix, new_name)));
        }
    }

    Ok(Some(Deprecation::DeprecatedInFavorOf(root, new_name)))
}

/// Creates the submodule for the given root type and URL.
async fn create_submodule(root: &str, url: &str) -> Result<TokenStream, Error> {
    // First, we download the CSV file
    let _download_report = Downloader::default()
        .user_agent("EarthMetabolomeDownloader/0.1.0 (https://earthmetabolome.org; contact@earthmetabolome.org)")
        .task(url)?
        .delete_on_cancel()
        .cache()
        .extract()
        .execute()
        .await?;

    // Next, we load it up.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(format!("{root}.csv"))?;

    let rows = reader.deserialize::<Row>().collect::<Result<Vec<Row>, csv::Error>>()?;

    // We check that all subtype templates start with the root
    let mut names = Vec::new();
    let mut sub_types = Vec::new();
    let expected_prefix = format!("{root}/");
    for row in &rows {
        if let Some(subtype) = row.template.strip_prefix(&expected_prefix) {
            names.push(row.name.as_ref());
            sub_types.push(subtype);
        } else {
            return Err(Error::InvalidSubtypeTemplate {
                root: root.to_string(),
                template: row.template.clone(),
            });
        }
    }

    let consts = names
        .iter()
        .zip(sub_types.iter())
        .map(|(name, subtype)| {
            let const_name = to_const_name(subtype);
            let docstring = format!("The `{subtype}` subtype of the `{root}` media type.",);
            let deprecation_message = parse_deprecation_messages(root, name)?.map(|deprecation| {
                let deprecation_note = match deprecation {
                    Deprecation::CompletelyDeprecated => {
                        "This subtype is deprecated and should not be used.".to_owned()
                    }
                    Deprecation::DeprecatedInFavorOf(new_root, new_sub_type) => {
                        let crate_name =
                            std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME not set");
                        let new_const_name = to_const_str_name(new_sub_type);
                        format!("Use `{crate_name}::{new_root}::{new_const_name}` instead")
                    }
                };
                quote! {
                    #[deprecated(since = "0.1.0", note = #deprecation_note)]
                }
            });
            Ok(quote! {
                #[doc = #docstring]
                #deprecation_message
                pub const #const_name: &str = #subtype;
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let phf_set_documentation = format!(
        "Set of all subtypes of the `{root}` media type. This set is generated from the IANA registry at `{url}`.",
    );
    let phf_set_ident = Ident::new("SUBTYPES", proc_macro2::Span::call_site());
    let phf_set = quote! {
        #[doc = #phf_set_documentation]
        pub static #phf_set_ident: ::phf::Set<&'static str> = ::phf::phf_set! {
            #(#sub_types),*
        };
    };

    let module_docstring = format!(
        "Submodule providing the `{root}` media type and its subtypes. This module is generated from the IANA registry at `{url}`.",
    );

    Ok(quote! {
        #![doc = #module_docstring]

        #(#consts)*

        #phf_set
    })
}

#[tokio::main]
/// Build script for the `elements` crate
pub async fn main() -> Result<(), Error> {
    for (root, url) in CSV_URLS {
        let Some(url) = url else {
            continue;
        };

        let module = create_submodule(root, url).await?;

        // We write out the module to `src/<root>.rs`
        let file_name = format!("{root}.rs");
        let file_path = PathBuf::from("src").join(&file_name);
        let mut file = std::fs::File::create(&file_path).expect("Failed to create file");
        writeln!(file, "{module}").expect("Failed to write to file");

        // We run rustfmt on the generated file
        let status = std::process::Command::new("rustfmt")
            .arg(file_path.to_str().unwrap())
            .status()
            .expect("Failed to run rustfmt");
        assert!(status.success(), "rustfmt failed with status: {status}");
    }

    Ok(())
}
