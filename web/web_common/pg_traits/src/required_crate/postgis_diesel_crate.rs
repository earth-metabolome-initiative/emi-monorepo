//! Submodule implementing the method `postgis_diesel` for the [`RequiredCrate`]
//! struct which initializes a `RequiredCrate` instance describing the
//! `postgis_diesel` crate.

use common_traits::builder::Builder;

use crate::{RequiredCrate, RequiredType};

impl RequiredCrate {
    /// Initializes a `RequiredCrate` instance describing the `postgis_diesel`
    /// crate.
    pub fn postgis_diesel() -> Self {
        RequiredCrate::new()
            .name("postgis_diesel")
            .unwrap()
            .add_types(vec![
                RequiredType::point(),
                RequiredType::linestring(),
                RequiredType::polygon(),
                RequiredType::multipoint(),
                RequiredType::multilinestring(),
                RequiredType::multipolygon(),
                RequiredType::geometrycollection(),
                RequiredType::geometry(),
            ])
            .unwrap()
            .build()
            .expect("Failed to build RequiredCrate for postgis_diesel")
    }
}

impl RequiredType {
    fn point() -> Self {
        RequiredType::new()
            .postgres_type("point")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::Point").unwrap())
            .supports_copy()
            .supports_debug()
            .supports_partial_eq()
            .supports_partial_ord()
            .build()
            .expect("Failed to build RequiredType for Point")
    }

    fn linestring() -> Self {
        RequiredType::new()
            .postgres_type("linestring")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::LineString").unwrap())
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build RequiredType for LineString")
    }

    fn polygon() -> Self {
        RequiredType::new()
            .postgres_type("polygon")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::Polygon").unwrap())
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build RequiredType for Polygon")
    }

    fn multipoint() -> Self {
        RequiredType::new()
            .postgres_type("multipoint")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::MultiPoint").unwrap())
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build RequiredType for MultiPoint")
    }

    fn multilinestring() -> Self {
        RequiredType::new()
            .postgres_type("multilinestring")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::MultiLineString").unwrap())
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build RequiredType for MultiLineString")
    }

    fn multipolygon() -> Self {
        RequiredType::new()
            .postgres_type("multipolygon")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::MultiPolygon").unwrap())
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build RequiredType for MultiPolygon")
    }

    fn geometrycollection() -> Self {
        RequiredType::new()
            .postgres_type("geometrycollection")
            .unwrap()
            .diesel_type(syn::parse_str("postgis_diesel::sql_types::Geometry").unwrap())
            .rust_type(syn::parse_str("postgis_diesel::types::GeometryCollection").unwrap())
            .supports_clone()
            .supports_debug()
            .supports_partial_eq()
            .build()
            .expect("Failed to build RequiredType for GeometryCollection")
    }

    fn geometry() -> Self {
        RequiredType::new()
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
            .expect("Failed to build RequiredType for Geometry")
    }
}
