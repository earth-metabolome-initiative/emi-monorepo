//! Submodule implementing the method `postgis_diesel` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `postgis_diesel` crate.

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate {
    /// Returns the cached `ExternalCrate` instance describing the
    /// `postgis_diesel` crate.
    pub fn postgis_diesel() -> ExternalCrate {
        ExternalCrate::new("postgis_diesel")
            .unwrap()
            .features(["serde", "diesel", "postgres", "sqlite"])
            .types([
                ExternalType::point(),
                ExternalType::linestring(),
                ExternalType::polygon(),
                ExternalType::multipoint(),
                ExternalType::multilinestring(),
                ExternalType::multipolygon(),
                ExternalType::geometrycollection(),
                ExternalType::geometry(),
                ExternalType::geography(),
            ])
            .unwrap()
            .git("https://github.com/LucaCappelletti94/postgis-diesel", "master")
            .into()
    }
}

impl ExternalType {
    fn point() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(postgis_diesel::types::Point),
        )
        .postgres_types(["point", "geography(point, 4326)", "geometry(point, 4326)"])
        .unwrap()
        .supports_copy()
        .supports_debug()
        .supports_partial_eq()
        .supports_partial_ord()
        .into()
    }

    fn linestring() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(postgis_diesel::types::LineString),
        )
        .postgres_type("linestring")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }

    fn polygon() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(postgis_diesel::types::Polygon),
        )
        .postgres_type("polygon")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }

    fn multipoint() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(postgis_diesel::types::MultiPoint),
        )
        .postgres_type("multipoint")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }

    fn multilinestring() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(postgis_diesel::types::MultiLineString),
        )
        .postgres_type("multilinestring")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }

    fn multipolygon() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(postgis_diesel::types::MultiPolygon),
        )
        .postgres_type("multipolygon")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }

    fn geometrycollection() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(postgis_diesel::types::GeometryCollection),
        )
        .postgres_type("geometrycollection")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }

    fn geometry() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geometry),
            syn::parse_quote!(
                postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>
            ),
        )
        .postgres_type("geometry")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }

    fn geography() -> Self {
        ExternalType::new(
            syn::parse_quote!(postgis_diesel::sql_types::Geography),
            syn::parse_quote!(
                postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>
            ),
        )
        .postgres_type("geography")
        .unwrap()
        .supports_clone()
        .supports_debug()
        .supports_partial_eq()
        .into()
    }
}
