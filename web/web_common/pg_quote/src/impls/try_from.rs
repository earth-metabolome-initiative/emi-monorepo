//! Submodule providing implementations of the `TryFrom` trait to attempt conversion of various structs from
//! the [`pg_diesel`](crate) into the corresponding `syn` AST nodes.

use pg_diesel::models::PgOperator;

impl TryFrom<PgOperator> for syn::BinOp {
    type Error = syn::Error;

    fn try_from(value: PgOperator) -> Result<Self, Self::Error> {
        syn::parse_str(&value.oprname)
    }
}

#[allow(clippy::too_many_lines)]
/// Returns the syn of the traits necessary for diesel to support the
/// conversion between the Postgres type and the Rust type.
///
/// # Arguments
///
/// * `conn` - The Postgres connection.
///
/// # Returns
///
/// A Result containing the syn of the struct or enum associated to the
/// `PgType`, or an error if the type is not supported.
///
/// # Errors
///
/// * Returns an error if the provided database connection fails.
///
/// # Panics
///
/// * If it is unknown what type implementations are needed.
pub fn to_diesel_impls(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
	let diesel_struct_path = self.diesel_type(false, conn)?;
	let rust_struct_path = self.rust_type(false, conn)?;
	if self.is_composite() {
		let mut diesel_types = Vec::new();
		let mut rust_types = Vec::new();
		let mut struct_attributes = Vec::new();
		let mut field_names = Vec::new();
		let attributes = self.attributes(conn)?;
		for attribute in &attributes {
			let field_name = Ident::new(&attribute.attname, proc_macro2::Span::call_site());
			let field_pg_type = attribute.pg_type(conn)?;
			let field_type = field_pg_type.rust_type(false, conn)?;
			field_names.push(quote! {
				#field_name
			});
			rust_types.push(quote! {
				#field_type
			});
			let diesel_type = field_pg_type.diesel_type(attribute.attnotnull, conn)?;
			if field_pg_type.supports_copy(conn)? || attributes.len() == 1 {
				struct_attributes.push(quote! {
					self.#field_name
				});
			} else {
				struct_attributes.push(quote! {
					self.#field_name.clone()
				});
			}

			diesel_types.push(quote! {
				#diesel_type
			});
		}

		let to_sql_operation = if diesel_types.len() > 1 {
			quote! {
				diesel::serialize::WriteTuple::<(#(#diesel_types),*)>::write_tuple(
					&(#(#struct_attributes),*),
					&mut out.reborrow(),
				)
			}
		} else {
			quote! {
				diesel::serialize::ToSql::<#(#diesel_types)*, diesel::pg::Pg>::to_sql(
					&#(#struct_attributes)*,
					out,
				)
			}
		};

		let from_sql_ops = if diesel_types.len() > 1 {
			quote! {
				let (#(#field_names),*): (#(#rust_types),*) = diesel::deserialize::FromSql::<diesel::sql_types::Record<(#(#diesel_types),*)>, diesel::pg::Pg>::from_sql(bytes)?;
				Ok(Self {
					#(#field_names),*
				})
			}
		} else {
			quote! {
				let #(#field_names)*: #(#rust_types),* = diesel::deserialize::FromSql::<#(#diesel_types)*, diesel::pg::Pg>::from_sql(bytes)?;
				Ok(Self {
					#(#field_names),*
				})
			}
		};

		Ok(quote! {
			#[cfg(feature = "postgres")]
			impl diesel::serialize::ToSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
				fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
					#to_sql_operation
				}
			}

			#[cfg(feature = "postgres")]
			impl diesel::deserialize::FromSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
				fn from_sql(
					bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
				) -> diesel::deserialize::Result<Self> {
					#from_sql_ops
				}
			}
		})
	} else if self.is_enum() {
		let variants = self.variants(conn)?;
		let mut in_variants = Vec::new();
		let mut out_variants = Vec::new();
		for variant in &variants {
			let variant_name = Ident::new(&variant.enumlabel, proc_macro2::Span::call_site());
			let variant = variant.enumlabel.clone();
			in_variants.push(quote! {
				#variant => Ok(Self::#variant_name),
			});
			out_variants.push(quote! {
				Self::#variant_name => std::io::Write::write_all(out, #variant.as_bytes())?,
			});
		}

		Ok(quote! {
			#[cfg(feature = "postgres")]
			impl diesel::serialize::ToSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
				fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
					match *self {
						#(#out_variants)*
					}
					Ok(diesel::serialize::IsNull::No)
				}
			}

			#[cfg(feature = "postgres")]
			impl diesel::deserialize::FromSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
				fn from_sql(
					bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
				) -> diesel::deserialize::Result<Self> {
					let s: String = diesel::deserialize::FromSql::<diesel::sql_types::Text, diesel::pg::Pg>::from_sql(bytes)?;
					match s.as_str() {
						#(#in_variants)*
						unknown => Err(format!("Unknown variant: {}", unknown).into()),
					}
				}
			}
		})
	} else {
		panic!("Unsupported type: {self:?}");
	}
}

#[must_use]
    /// Returns the syn of the struct or enum associated to the `PgType`.
    ///
    /// # Returns
    ///
    /// A Result containing the syn of the struct or enum associated to the
    /// `PgType`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    ///
    /// # Panics
    ///
    /// * If it is unknown what type macros are needed.
    pub fn to_diesel_macro(&self) -> TokenStream {
        let postgres_struct_name = self.pg_binding_ident();
        let this_typname: &str = &self.typname;
        if self.is_composite() || self.is_enum() {
            quote! {
                #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
                #[diesel(postgres_type(name = #this_typname))]
                pub struct #postgres_struct_name;
            }
        } else {
            panic!("Unsupported type: {self:?}");
        }
    }

	#[must_use]
    /// Returns the `CamelCased` name of the `PgType`.
    pub fn camelcased_name(&self) -> String {
        self.typname
            .split('_')
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }

    #[must_use]
    /// Returns the `CamelCased` name of the [`PgType`] for the Postgres
    /// binding.
    pub fn pg_binding_name(&self) -> String {
        format!("Pg{}", self.camelcased_name())
    }

    #[must_use]
    /// Returns the `CamelCased` Ident of the [`PgType`] for the Diesel binding.
    pub fn pg_binding_ident(&self) -> Ident {
        Ident::new(&self.pg_binding_name(), proc_macro2::Span::call_site())
    }

    /// Returns the syn-based struct or enum associated to the `PgType`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the syn of the struct or enum associated to the
    /// `PgType`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    ///
    /// # Panics
    ///
    /// * If it is unknown how to implement the associated struct or enum.
    pub fn to_struct_or_enum(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
        let postgres_struct_name = self.diesel_type(false, conn)?;
        if self.is_composite() {
            let mut fields = Vec::new();
            let attributes = self.attributes(conn)?;
            for attribute in &attributes {
                let field_name = Ident::new(&attribute.attname, proc_macro2::Span::call_site());
                let field_pg_type = attribute.pg_type(conn)?;
                let field_type = field_pg_type.rust_type(false, conn)?;

                fields.push(quote! {
                    pub #field_name: #field_type
                });
            }

            let mut derives = vec![
                Ident::new("Debug", proc_macro2::Span::call_site()),
                Ident::new("Clone", proc_macro2::Span::call_site()),
                Ident::new("PartialEq", proc_macro2::Span::call_site()),
            ];

            if self.supports_eq(conn)? {
                derives.push(Ident::new("Eq", proc_macro2::Span::call_site()));
            }

            if self.supports_hash(conn)? {
                derives.push(Ident::new("Hash", proc_macro2::Span::call_site()));
            }

            if self.supports_copy(conn)? {
                derives.push(Ident::new("Copy", proc_macro2::Span::call_site()));
            }

            Ok(quote! {
                #[derive(#(#derives),*)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
                #[diesel(sql_type = #postgres_struct_name)]
                pub struct #struct_name {
                    #(#fields),*
                }
            })
        } else if self.is_enum() {
            let variants = self.variants(conn)?;
            let mut variant_names = Vec::new();
            for variant in &variants {
                let variant_name = Ident::new(&variant.enumlabel, proc_macro2::Span::call_site());
                variant_names.push(quote! {
                    #variant_name
                });
            }

            Ok(quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
                #[diesel(sql_type = #postgres_struct_name)]
                pub enum #struct_name {
                    #(#variant_names),*
                }
            })
        } else {
            panic!("Unsupported type: {self:?}");
        }
    }

    /// Returns the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A string representing the sanitized snake case name of the table.
    pub fn snake_case_name(&self) -> Result<String, WebCodeGenError> {
        crate::utils::snake_case_name(&self.typname)
    }

    /// Returns the sanitized snake case identifier of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case identifier cannot be generated.
    pub fn snake_case_identifier(&self) -> Result<Ident, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(&snake_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
        }
    }