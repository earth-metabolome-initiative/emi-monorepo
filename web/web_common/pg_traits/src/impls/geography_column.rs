//! Submodule implementing traits for the [`GeographyColumn`](pg_diesel::models::GeographyColumn) model.

#[must_use]
    /// Returns the rust type of the geometry column.
    ///
    /// # Panics
    ///
    /// * If the geometry type is unknown.
    pub fn str_rust_type(&self) -> &'static str {
        match self.r#type.as_str() {
            "Point" => "postgis_diesel::types::Point",
            "LINESTRING" => "postgis_diesel::types::LineString",
            "POLYGON" => "postgis_diesel::types::Polygon",
            "MULTIPOINT" => "postgis_diesel::types::MultiPoint",
            "MULTILINESTRING" => "postgis_diesel::types::MultiLineString",
            "MULTIPOLYGON" => "postgis_diesel::types::MultiPolygon",
            "GEOMETRYCOLLECTION" => "postgis_diesel::types::GeometryCollection",
            unknown => panic!("Unknown geography type: {unknown}"),
        }
    }

    /// Returns the rust type of the geometry column.
    ///
    /// # Arguments
    ///
    /// * `optional` - If `true`, the type will be wrapped in an `Option`.
    ///
    /// # Errors
    ///
    /// * If the rust type cannot be parsed.
    pub fn rust_type(&self, optional: bool) -> Result<syn::Type, WebCodeGenError> {
        let mut rust_type_str = self.str_rust_type().to_owned();

        if optional {
            rust_type_str = format!("Option<{rust_type_str}>");
        }

        Ok(syn::parse_str(&rust_type_str)?)
    }

    #[must_use]
    /// Returns whether the underlying rust type supports the `Copy` trait.
    ///
    /// # Panics
    ///
    /// * If the geography type is unknown.
    pub fn supports_copy(&self) -> bool {
        match self.r#type.as_str() {
            "POINT" | "Point" => true,
            "LINESTRING" | "POLYGON" | "MULTIPOINT" | "MULTILINESTRING" | "MULTIPOLYGON"
            | "GEOMETRYCOLLECTION" | "GEOMETRY" => false,
            _ => panic!("Unknown geography type: {}", self.r#type.as_str()),
        }
    }