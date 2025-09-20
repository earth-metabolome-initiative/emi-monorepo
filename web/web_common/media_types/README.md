# Mime Types

[![PGRX Build](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-media_types.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-build-media_types.yml)
[![Clippy](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-media_types.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-clippy-media_types.yml)
[![Test](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-media_types.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/cargo-test-media_types.yml)

[A media type](https://en.wikipedia.org/wiki/Media_type) (formerly known as MIME type) is a standardized way to indicate the nature and format of a file or data. It is used in protocols like HTTP, email (MIME), and others to tell clients (e.g., browsers) how to handle the content.

The [media_types](https://github.com/earth-metabolome-initiative/emi-monorepo/tree/main/web/web_common/media_types) crate provides strongly typed structs and methods to operate on mime types, with integration with [`serde`](https://docs.rs/serde/latest/serde/index.html), [`diesel`](https://github.com/diesel-rs/diesel) with both `SQLite` and `PostgreSQL` backends, and additionally support for [`pgrx`](https://github.com/pgcentralfoundation/pgrx).

## Usage examples

Here are some introductory examples of how to use the `media_types` crate.

### Parsing a mime type

Suppose you want to parse a mime type from a string. You can use the `MediaType` struct to do this:

```rust
use media_types::{MediaType, RootMediaType, SubMediaType, errors::Error};

let mime_type = MediaType::try_from("image/gif").unwrap();
assert_eq!(mime_type.root(), RootMediaType::Image);
assert_eq!(mime_type.subtype(), media_types::image::GIF);

let wrong_mime_type1 = MediaType::try_from("application/vnd.qcelp+json+xml").unwrap_err();
assert_eq!(wrong_mime_type1, Error::UnknownSubMediaType(RootMediaType::Application, "vnd.qcelp+json+xml".to_owned()));

let wrong_mime_type2 = MediaType::try_from("audio/jpeg").unwrap_err();
assert_eq!(wrong_mime_type2, Error::UnknownSubMediaType(RootMediaType::Audio, "jpeg".to_owned()));

let wrong_mime_type2 = MediaType::try_from("customized/jpeg").unwrap_err();
assert_eq!(wrong_mime_type2, Error::UnknownRootMediaType("customized".to_owned()));
```

### Using in a match

One of the most common use cases for media types is to match them against a set of known types.
Here's an example of how to do this with `media_types`:

```rust
use media_types::{MediaType, RootMediaType};

let mime_type = MediaType::try_from("image/gif").unwrap();

match (mime_type.root(), mime_type.subtype().as_ref()) {
    (RootMediaType::Image, media_types::image::GIF) => {
        println!("This is a GIF image!");
    }
    (RootMediaType::Image, media_types::image::PNG) => {
        println!("This is a PNG image!");
    }
    (RootMediaType::Audio, media_types::audio::MP4) => {
        println!("This is an MP4 audio file!");
    }
    _ => {
        println!("I am not sure what to do with this media type.");
    }
}
```

### How the validation is implemented

For all the root media types, there is an extensive list of known subtypes, as made available by the [IANA](https://www.iana.org/assignments/media-types/media-types.xhtml) registry. While it may be desirable to have a complete enumeration for each of these, it is not practical to do so as it would involve large match statements, [which lead to odd compiler errors when you have as target WASM](https://github.com/drager/wasm-pack/issues/981). For this reason, we decided instead to use the [`phf::Set`](https://docs.rs/phf/latest/phf/struct.Set.html) type and the [`phf::phf_set`](https://docs.rs/phf/latest/phf/macro.phf_set.html) compile time macro to create a set of known subtypes for each root media type. This allows us to have a complete list of known subtypes without the need for large match statements.

If you desire to check whether a subtype is valid or not for a given root media type, you can proceed as follows:

```rust
use media_types::image::SUBTYPES;

assert!(SUBTYPES.contains("gif"));
assert!(!SUBTYPES.contains("gifv"));
```

### Deprecation messages

Over time, some of the media types have been deprecated or replaced by new ones. At this time, we include the deprecated media types in the `media_types` crate, but we also provide a deprecation message to inform users that they should use the new media type instead. The deprecated media types are listed by [`IANA`](https://www.iana.org/assignments/media-types/media-types.xhtml), and appear in their CSVs with labels such as: `vnd.qcelp - DEPRECATED in favor of audio/qcelp`, `javascript (OBSOLETED in favor of text/javascript)`, or `remote-printing (OBSOLETE)`.

This means that when you try to use a media type constant which has been deprecated in favour of some other constant, like: `media_types::application::JAVASCRIPT`, you will get a deprecation message like this: ```use of deprecated constant `media_types::application::JAVASCRIPT`: Use `media_types::text::JAVASCRIPT` instead```. Alternatively, when you try to use something that has been deprecated and removed, like: `media_types::application::REMOTE_PRINTING`, you will get a compilation error like this: ```use of deprecated constant `application::REMOTE_PRINTING`: This subtype is deprecated and should not be used.```.

## Using with `PostgreSQL`

The `media_types` crate provides a custom `PostgreSQL` extension that allows you to use the `media_types` crate in your `PostgreSQL` database. This extension provides a custom data type for media types, which seamlessly integrates with the `diesel` ORM. Do note that the binary serialization used for `pgrx` extension is [CBOR](https://en.wikipedia.org/wiki/CBOR) based, which is the default serialization format for `pgrx` extensions.

### Compiling the PGRX extension

After having cloned the repository, you can compile the PGRX extension in the `./extension` directory by running in this directory:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.

### Installing the PGRX extension

As with any other postgres extension, you can install the `media_types` extension by first copying the `*.so`, `*.sql`, and `*.control` files to the `postgres` extension directory. This is usually located at `/usr/share/postgresql/extension/` or `/usr/local/share/postgresql/extension/`. You can then run the following command in your `PostgreSQL` database:

```sql
CREATE EXTENSION "media_types";
```

### Using in a table

Now that you have installed the `media_types` extension, you can use it in your `PostgreSQL` database. You can create a table with a column of type `media_type` as follows:

```sql
CREATE TABLE my_table (
    id SERIAL PRIMARY KEY,
    mime_type MediaType NOT NULL
);
```

### Using with Diesel

The `media_types` crate provides a custom `Diesel` type that allows you to use the `media_types` crate in your `PostgreSQL` database. This type is called `MediaType`, and it is used to represent media types in your database. Remember to enable the `diesel_pgrx` feature in your `Cargo.toml` file for the `media_types` crate, alongside your desired database backend, which can either be `sqlite` or `postgres`.

Next, to use the `MediaType` type in your `Diesel` schema, you need to add the following line to your `schema.rs` file:

```rust
# #[cfg(all(feature = "diesel_pgrx", any(feature = "postgres", feature = "sqlite")))]
# pub fn main() {
    diesel::table! {
        my_table (id) {
            id -> Integer,
            mime_type -> ::media_types::diesel_impls::MediaType,
        }
    }

    #[derive(diesel::Queryable, diesel::Insertable, Debug)]
    #[diesel(table_name = my_table)]
    struct MyTable {
        id: i32,
        mime_type: ::media_types::MediaType,
    }
# }
# #[cfg(not(all(feature = "diesel_pgrx", any(feature = "postgres", feature = "sqlite"))))]
# pub fn main() {}
```

## Related work

The following crates also tackle the problem of representing media types in Rust:

* [mime](https://crates.io/crates/mime): A crate for parsing media types.

## Contributing

We welcome contributions to the `media_types` crate! Do check out our [contributing guidelines](https://github.com/earth-metabolome-initiative/emi-monorepo/blob/main/CONTRIBUTING.md) to get started.
