#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProcedureTemplateAttribute {
    ProcedureTemplate,
    MostConcreteTable,
    Name,
    Description,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
    Deprecated,
}
impl core::str::FromStr for ProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MostConcreteTable" => Ok(Self::MostConcreteTable),
            "Name" => Ok(Self::Name),
            "Description" => Ok(Self::Description),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "UpdatedBy" => Ok(Self::UpdatedBy),
            "UpdatedAt" => Ok(Self::UpdatedAt),
            "Deprecated" => Ok(Self::Deprecated),
            "most_concrete_table" => Ok(Self::MostConcreteTable),
            "name" => Ok(Self::Name),
            "description" => Ok(Self::Description),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            "updated_by" => Ok(Self::UpdatedBy),
            "updated_at" => Ok(Self::UpdatedAt),
            "deprecated" => Ok(Self::Deprecated),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder
{
    type Attribute = ProcedureTemplateAttribute;
}
impl web_common_traits::database::TableField for ProcedureTemplateAttribute {}
impl web_common_traits::database::HasTableType for ProcedureTemplateAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl core::fmt::Display for ProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate => {
                write!(f, "procedure_templates.procedure_template")
            }
            Self::MostConcreteTable => {
                write!(f, "procedure_templates.most_concrete_table")
            }
            Self::Name => write!(f, "procedure_templates.name"),
            Self::Description => write!(f, "procedure_templates.description"),
            Self::CreatedBy => write!(f, "procedure_templates.created_by"),
            Self::CreatedAt => write!(f, "procedure_templates.created_at"),
            Self::UpdatedBy => write!(f, "procedure_templates.updated_by"),
            Self::UpdatedAt => write!(f, "procedure_templates.updated_at"),
            Self::Deprecated => write!(f, "procedure_templates.deprecated"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureTemplate {
    pub(crate) most_concrete_table: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) deprecated: bool,
}
impl InsertableProcedureTemplate {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.updated_by, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`ProcedureTemplate`](crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::ProcedureTemplate;
/// use core_structures::tables::insertables::ProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let procedure_template = ProcedureTemplate::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .deprecated(deprecated)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableProcedureTemplateBuilder {
    pub(crate) most_concrete_table: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) deprecated: Option<bool>,
}
impl diesel::associations::HasTable for InsertableProcedureTemplateBuilder {
    type Table =
        crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::procedure_templates::procedure_templates::table
    }
}
impl From<InsertableProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableProcedureTemplateBuilder>
{
    fn from(builder: InsertableProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableProcedureTemplateBuilder {
    fn default() -> Self {
        Self {
            most_concrete_table: Default::default(),
            name: Default::default(),
            description: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
            deprecated: Some(false),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder
{
    fn is_complete(&self) -> bool {
        self.name.is_some()
            && self.description.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.updated_by.is_some()
            && self.updated_at.is_some()
            && self.deprecated.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `ProcedureTemplate`
/// or descendant tables.
pub trait ProcedureTemplateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.procedure_templates.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.procedure_templates.name`
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
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.procedure_templates.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the
    ///   `public.procedure_templates.description` column.
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
    fn description<D>(self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<String>>::Error>;
    /// Sets the value of the `public.procedure_templates.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.procedure_templates.created_by` column.
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
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedure_templates.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.procedure_templates.created_at` column.
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
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.procedure_templates.updated_by` column.
    ///
    /// # Arguments
    /// * `updated_by`: The value to set for the
    ///   `public.procedure_templates.updated_by` column.
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
    fn updated_by<UB>(self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedure_templates.updated_at` column.
    ///
    /// # Arguments
    /// * `updated_at`: The value to set for the
    ///   `public.procedure_templates.updated_at` column.
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
    fn updated_at<UA>(self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.procedure_templates.deprecated` column.
    ///
    /// # Arguments
    /// * `deprecated`: The value to set for the
    ///   `public.procedure_templates.deprecated` column.
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
    /// * If the provided value cannot be converted to the required type `bool`.
    /// * If the provided value does not pass schema-defined validation.
    fn deprecated<D>(self, deprecated: D) -> Result<Self, Self::Error>
    where
        D: TryInto<bool>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<bool>>::Error>;
}
impl ProcedureTemplateSettable for InsertableProcedureTemplateBuilder
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(ProcedureTemplateAttribute::Name)
            })?;
        if let Some(description) = self.description.as_ref() {
            pgrx_validation::must_be_distinct(name.as_ref(), description)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::Name,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::Name,
                    )
            })?;
        self.name = Some(name);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        let description = description
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(ProcedureTemplateAttribute::Description)
            })?;
        if let Some(name) = self.name.as_ref() {
            pgrx_validation::must_be_distinct(name, description.as_ref())
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::Name,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::Description,
                    )
            })?;
        self.description = Some(description);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_templates.created_by` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///v0@{shape: rounded, label: "created_by"}
    ///class v0 column-of-interest
    ///v1@{shape: rounded, label: "updated_by"}
    ///class v1 directly-involved-column
    ///```
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by = <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &created_by,
        );
        self = self.updated_by(created_by)?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_templates.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        let created_at = created_at
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(ProcedureTemplateAttribute::CreatedAt)
            })?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::created_at <= updated_at
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::UpdatedAt,
                        )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_templates.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let updated_by = <UB as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &updated_by,
        );
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_templates.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        let updated_at = updated_at
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(ProcedureTemplateAttribute::UpdatedAt)
            })?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::created_at <= updated_at
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::UpdatedAt,
                        )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(mut self, deprecated: D) -> Result<Self, Self::Error>
    where
        D: TryInto<bool>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        let deprecated = deprecated
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(ProcedureTemplateAttribute::Deprecated)
            })?;
        self.deprecated = Some(deprecated);
        Ok(self)
    }
}
impl web_common_traits::database::MostConcreteTable for InsertableProcedureTemplateBuilder {
    fn set_most_concrete_table(&mut self, table_name: &str) {
        if self.most_concrete_table.is_none() {
            self.most_concrete_table = Some(table_name.to_owned());
        }
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableProcedureTemplateBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableProcedureTemplateBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
            Error = web_common_traits::database::InsertError<ProcedureTemplateAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<ProcedureTemplateAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
