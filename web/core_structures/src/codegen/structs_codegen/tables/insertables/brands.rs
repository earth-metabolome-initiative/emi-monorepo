#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BrandAttribute {
    Id,
    Name,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::str::FromStr for BrandAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "UpdatedBy" => Ok(Self::UpdatedBy),
            "UpdatedAt" => Ok(Self::UpdatedAt),
            "name" => Ok(Self::Name),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            "updated_by" => Ok(Self::UpdatedBy),
            "updated_at" => Ok(Self::UpdatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for BrandAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedBy => write!(f, "updated_by"),
            Self::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::brands::brands)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBrand {
    pub(crate) name: String,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableBrand {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.updated_by,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBrandBuilder {
    pub(crate) name: Option<String>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl From<InsertableBrandBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableBrandBuilder>
{
    fn from(builder: InsertableBrandBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableBrandBuilder {
    fn default() -> Self {
        Self {
            name: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
/// Trait defining setters for attributes of an instance of `Brand` or
/// descendant tables.
pub trait BrandSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.brands.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.brands.name` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.brands.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the `public.brands.created_by`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_by(
        self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.brands.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the `public.brands.created_at`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_at<CA>(
        self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.brands.updated_by` column.
    ///
    /// # Arguments
    /// * `updated_by`: The value to set for the `public.brands.updated_by`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn updated_by(
        self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.brands.updated_at` column.
    ///
    /// # Arguments
    /// * `updated_at`: The value to set for the `public.brands.updated_at`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn updated_at<UA>(
        self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
}
impl BrandSettable for InsertableBrandBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::BrandAttribute;
    /// Sets the value of the `public.brands.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(BrandAttribute::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::BrandAttribute::Name,
            )
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.brands.created_by` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// v0@{shape: rounded, label: "created_by"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "updated_by"}
    /// class v1 directly-involved-column
    /// ```
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self = self.updated_by(created_by)?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.brands.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(BrandAttribute::CreatedAt)
        })?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::CreatedAt,
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    /// Sets the value of the `public.brands.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    /// Sets the value of the `public.brands.updated_at` column.
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let updated_at = updated_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(BrandAttribute::UpdatedAt)
        })?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::CreatedAt,
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableBrandBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableBrandBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::brands::Brand,
            Error = web_common_traits::database::InsertError<BrandAttribute>,
        >,
{
    type Attributes = BrandAttribute;
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.updated_by.is_some()
            && self.updated_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::brands::Brand =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
