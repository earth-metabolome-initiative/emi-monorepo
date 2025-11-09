//! Submodule implementing the method `postgis_diesel` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `postgis_diesel` crate.

use std::sync::Arc;

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Initializes a `ExternalCrate` instance describing the `postgis_diesel`
    /// crate.
    pub fn postgis_diesel() -> Arc<Self> {
        Arc::from(
            ExternalCrate::new()
                .name("postgis_diesel")
                .unwrap()
                .features(["serde", "diesel", "postgres", "sqlite"])
                .add_types(vec![
                    Arc::new(ExternalType::point()),
                    Arc::new(ExternalType::linestring()),
                    Arc::new(ExternalType::polygon()),
                    Arc::new(ExternalType::multipoint()),
                    Arc::new(ExternalType::multilinestring()),
                    Arc::new(ExternalType::multipolygon()),
                    Arc::new(ExternalType::geometrycollection()),
                    Arc::new(ExternalType::geometry()),
                    Arc::new(ExternalType::geography()),
                ])
                .unwrap()
                // .version("3.0.2")
                .git("https://github.com/LucaCappelletti94/postgis-diesel", "master")
                .build()
                .expect("Failed to build ExternalCrate for postgis_diesel"),
        )
    }
}

impl ExternalType {
    fn point() -> Self {
        ExternalType::new()
            .postgres_types(["point", "geography(point, 4326)", "geometry(point, 4326)"])
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(postgis_diesel::types::Point))
            .supports_copy()
            .supports_debug()
            .supports_partial_eq()
            .supports_partial_ord()
            .supports_serde()
            .build()
            .expect("Failed to build ExternalType for Point")
    }

    fn linestring() -> Self {
        ExternalType::new()
            .postgres_type("linestring")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(postgis_diesel::types::LineString))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for LineString")
    }

    fn polygon() -> Self {
        ExternalType::new()
            .postgres_type("polygon")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(postgis_diesel::types::Polygon))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for Polygon")
    }

    fn multipoint() -> Self {
        ExternalType::new()
            .postgres_type("multipoint")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(postgis_diesel::types::MultiPoint))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for MultiPoint")
    }

    fn multilinestring() -> Self {
        ExternalType::new()
            .postgres_type("multilinestring")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(postgis_diesel::types::MultiLineString))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for MultiLineString")
    }

    fn multipolygon() -> Self {
        ExternalType::new()
            .postgres_type("multipolygon")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(postgis_diesel::types::MultiPolygon))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for MultiPolygon")
    }

    fn geometrycollection() -> Self {
        ExternalType::new()
            .postgres_type("geometrycollection")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(postgis_diesel::types::GeometryCollection))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for GeometryCollection")
    }

    fn geometry() -> Self {
        ExternalType::new()
            .postgres_type("geometry")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geometry))
            .rust_type(syn::parse_quote!(
                postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>
            ))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for Geometry")
    }

    fn geography() -> Self {
        ExternalType::new()
            .postgres_type("geography")
            .unwrap()
            .diesel_type(syn::parse_quote!(postgis_diesel::sql_types::Geography))
            .rust_type(syn::parse_quote!(
                postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>
            ))
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for Geography")
    }
}
