//! Submodule implementing the method `postgis_diesel` for the [`ExternalCrate`]
//! struct which initializes a `ExternalCrate` instance describing the
//! `postgis_diesel` crate.

use common_traits::builder::Builder;

use crate::structs::{ExternalCrate, ExternalType};

impl ExternalCrate<'static> {
    /// Initializes a `ExternalCrate` instance describing the `postgis_diesel`
    /// crate.
    pub fn postgis_diesel() -> Self {
        ExternalCrate::new()
            .name("postgis_diesel")
            .unwrap()
            .add_types(vec![
                ExternalType::point(),
                ExternalType::linestring(),
                ExternalType::polygon(),
                ExternalType::multipoint(),
                ExternalType::multilinestring(),
                ExternalType::multipolygon(),
                ExternalType::geometrycollection(),
                ExternalType::geometry(),
            ])
            .unwrap()
            .version("3.0.1")
            .build()
            .expect("Failed to build ExternalCrate for postgis_diesel")
    }
}

impl<'data> ExternalType<'data> {
    fn point() -> Self {
        ExternalType::new()
            .postgres_type("point")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::Point").unwrap())
            .supports_copy()
            .supports_debug()
            .supports_partial_eq()
            .supports_partial_ord()
            .build()
            .expect("Failed to build ExternalType for Point")
    }

    fn linestring() -> Self {
        ExternalType::new()
            .postgres_type("linestring")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::LineString").unwrap())
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
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::Polygon").unwrap())
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
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::MultiPoint").unwrap())
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
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::MultiLineString").unwrap())
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
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::MultiPolygon").unwrap())
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
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::GeometryCollection").unwrap())
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
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(
                syn::parse_str(
                    "postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>",
                )
                .unwrap(),
            )
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build ExternalType for Geometry")
    }
}
