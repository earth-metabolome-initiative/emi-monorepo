#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetModelAttribute {
    Id,
    MostConcreteTable,
    Name,
    Description,
    ParentModel,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::str::FromStr for AssetModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MostConcreteTable" => Ok(Self::MostConcreteTable),
            "Name" => Ok(Self::Name),
            "Description" => Ok(Self::Description),
            "ParentModel" => Ok(Self::ParentModel),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "UpdatedBy" => Ok(Self::UpdatedBy),
            "UpdatedAt" => Ok(Self::UpdatedAt),
            "most_concrete_table" => Ok(Self::MostConcreteTable),
            "name" => Ok(Self::Name),
            "description" => Ok(Self::Description),
            "parent_model" => Ok(Self::ParentModel),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            "updated_by" => Ok(Self::UpdatedBy),
            "updated_at" => Ok(Self::UpdatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for AssetModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "asset_models.id"),
            Self::MostConcreteTable => write!(f, "asset_models.most_concrete_table"),
            Self::Name => write!(f, "asset_models.name"),
            Self::Description => write!(f, "asset_models.description"),
            Self::ParentModel => write!(f, "asset_models.parent_model"),
            Self::CreatedBy => write!(f, "asset_models.created_by"),
            Self::CreatedAt => write!(f, "asset_models.created_at"),
            Self::UpdatedBy => write!(f, "asset_models.updated_by"),
            Self::UpdatedAt => write!(f, "asset_models.updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::asset_models::asset_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAssetModel {
    pub(crate) most_concrete_table: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parent_model: Option<i32>,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableAssetModel {
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
    pub fn parent_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::asset_models::AssetModel>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_model) = self.parent_model else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::read(parent_model, conn)
            .optional()
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
/// [`AssetModel`](crate::codegen::structs_codegen::tables::asset_models::AssetModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::AssetModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let asset_model = AssetModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableAssetModelBuilder {
    pub(crate) most_concrete_table: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) parent_model: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl From<InsertableAssetModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableAssetModelBuilder>
{
    fn from(builder: InsertableAssetModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableAssetModelBuilder {
    fn default() -> Self {
        Self {
            most_concrete_table: Default::default(),
            name: Default::default(),
            description: Default::default(),
            parent_model: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder
{
    fn is_complete(&self) -> bool {
        self.most_concrete_table.is_some()
            && self.name.is_some()
            && self.description.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.updated_by.is_some()
            && self.updated_at.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `AssetModel` or
/// descendant tables.
pub trait AssetModelSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.asset_models.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.asset_models.name` column.
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
    /// Sets the value of the `public.asset_models.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the
    ///   `public.asset_models.description` column.
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
    fn description<D>(
        self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>;
    /// Sets the value of the `public.asset_models.parent_model` column.
    ///
    /// # Arguments
    /// * `parent_model`: The value to set for the
    ///   `public.asset_models.parent_model` column.
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
    fn parent_model<PM>(
        self,
        parent_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.asset_models.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.asset_models.created_by` column.
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
    fn created_by<CB>(
        self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.asset_models.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.asset_models.created_at` column.
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
    /// Sets the value of the `public.asset_models.updated_by` column.
    ///
    /// # Arguments
    /// * `updated_by`: The value to set for the
    ///   `public.asset_models.updated_by` column.
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
    fn updated_by<UB>(
        self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.asset_models.updated_at` column.
    ///
    /// # Arguments
    /// * `updated_at`: The value to set for the
    ///   `public.asset_models.updated_at` column.
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
impl AssetModelSettable for InsertableAssetModelBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute;
    /// Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(AssetModelAttribute::Name)
        })?;
        if let Some(description) = self.description.as_ref() {
            pgrx_validation::must_be_distinct(name.as_ref(), description)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Name,
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(name.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Name,
            )
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.asset_models.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        let description = description.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(AssetModelAttribute::Description)
        })?;
        if let Some(name) = self.name.as_ref() {
            pgrx_validation::must_be_distinct(name, description.as_ref())
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Name,
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Description,
                        )
                })?;
        }
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Description,
                    )
            })?;
        self.description = Some(description);
        Ok(self)
    }
    /// Sets the value of the `public.asset_models.parent_model` column.
    fn parent_model<PM>(
        mut self,
        parent_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        let parent_model =
            <PM as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &parent_model,
            );
        self.parent_model = parent_model;
        Ok(self)
    }
    /// Sets the value of the `public.asset_models.created_by` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// v0@{shape: rounded, label: "created_by"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "updated_by"}
    /// class v1 directly-involved-column
    /// ```
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by =
            <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(&created_by);
        self = self.updated_by(created_by)?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.asset_models.created_at` column.
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
            validation_errors::SingleFieldError::from(err)
                .rename_field(AssetModelAttribute::CreatedAt)
        })?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::UpdatedAt,
                        )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    /// Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let updated_by =
            <UB as web_common_traits::database::PrimaryKeyLike>::primary_key(&updated_by);
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    /// Sets the value of the `public.asset_models.updated_at` column.
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
            validation_errors::SingleFieldError::from(err)
                .rename_field(AssetModelAttribute::UpdatedAt)
        })?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::UpdatedAt,
                        )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl web_common_traits::database::MostConcreteTable for InsertableAssetModelBuilder {
    fn set_most_concrete_table(&mut self, table_name: &str) {
        if self.most_concrete_table.is_none() {
            self.most_concrete_table = Some(table_name.to_owned());
        }
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableAssetModelBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableAssetModelBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::asset_models::AssetModel,
            Error = web_common_traits::database::InsertError<AssetModelAttribute>,
        >,
{
    type Attribute = AssetModelAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::asset_models::AssetModel =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
