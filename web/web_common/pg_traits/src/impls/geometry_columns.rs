
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
    /// Returns the rust type of the geometry column.
    ///
    /// # Panics
    ///
    /// * If the geometry type is unknown.
    pub fn str_rust_type(&self) -> &'static str {
        match self.r#type.as_str() {
            "POINT" => "postgis_diesel::types::Point",
            "LINESTRING" => "postgis_diesel::types::LineString",
            "POLYGON" => "postgis_diesel::types::Polygon",
            "MULTIPOINT" => "postgis_diesel::types::MultiPoint",
            "MULTILINESTRING" => "postgis_diesel::types::MultiLineString",
            "MULTIPOLYGON" => "postgis_diesel::types::MultiPolygon",
            "GEOMETRYCOLLECTION" => "postgis_diesel::types::GeometryCollection",
            "GEOMETRY" => "postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>",
            unknown => panic!("Unknown geometry type: {unknown}"),
        }
    }

    #[must_use]
    /// Returns whether the underlying rust type supports the `Copy` trait.
    ///
    /// # Panics
    ///
    /// * If the geometry type is unknown.
    pub fn supports_copy(&self) -> bool {
        match self.r#type.as_str() {
            "POINT" | "Point" => true,
            "LINESTRING" | "POLYGON" | "MULTIPOINT" | "MULTILINESTRING" | "MULTIPOLYGON"
            | "GEOMETRYCOLLECTION" | "GEOMETRY" => false,
            _ => panic!("Unknown geometry type: {}", self.r#type.as_str()),
        }
    }