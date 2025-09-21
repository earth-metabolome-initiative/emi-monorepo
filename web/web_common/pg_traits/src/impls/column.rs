
    /// Returns the diesel type of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the diesel `Type` of the column if the operation
    /// was successful, or a `crate::error::Error` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn diesel_type(&self, conn: &mut PgConnection) -> Result<Type, crate::error::Error> {
        let tentative_type = postgres_type_to_diesel(self.data_type_str(conn)?, self.is_nullable());
        match tentative_type {
            Ok(t) => Ok(t),
            Err(e) => {
                if self.has_custom_type() {
                    PgType::from_name(self.data_type_str(conn)?, conn)?
                        .diesel_type(self.is_nullable(), conn)
                } else {
                    Err(e)
                }
            }
        }
    }


    /// Returns whether the column type supports the `PartialOrd` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_partial_ord(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        if self.supports_ord(conn)? {
            return Ok(true);
        }
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(PARTIAL_ORD_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?
                        .supports_partial_ord(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns whether the column type supports the `Ord` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_ord(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(ORD_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_ord(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }


    /// Returns whether the column type implements copy.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        if let Ok(Some(geometry)) = self.geometry(conn) {
            return Ok(geometry.supports_copy());
        }
        if let Ok(Some(geography)) = self.geography(conn) {
            return Ok(geography.supports_copy());
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(COPY_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_copy(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns whether the column type supports the `Hash` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(HASH_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_hash(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns whether the column type supports the `Eq` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(EQ_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_eq(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }


    /// Returns the rust type of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the rust `Type` of the column if the operation
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn rust_data_type(&self, conn: &mut PgConnection) -> Result<Type, crate::error::Error> {
        if let Ok(Some(geometry)) = self.geometry(conn) {
            return geometry.rust_type(self.is_nullable());
        }
        if let Ok(Some(geography)) = self.geography(conn) {
            return geography.rust_type(self.is_nullable());
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => {
                if self.is_nullable() {
                    Ok(syn::parse_str(&format!("Option<{s}>"))?)
                } else {
                    Ok(syn::parse_str(s)?)
                }
            }
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?
                        .rust_type(self.is_nullable(), conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns the rust reference type of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the rust `Type` of the column if the operation
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn rust_ref_data_type(&self, conn: &mut PgConnection) -> Result<Type, crate::error::Error> {
        let rust_type = match self.str_rust_data_type(conn)?.as_str() {
            "String" => "&str".to_owned(),
            "Vec<u8>" => "&[u8]".to_owned(),
            other => format!("&{other}"),
        };

        let rust_type = if self.is_nullable() { format!("Option<{rust_type}>") } else { rust_type };

        Ok(syn::parse_str(&rust_type)?)
    }