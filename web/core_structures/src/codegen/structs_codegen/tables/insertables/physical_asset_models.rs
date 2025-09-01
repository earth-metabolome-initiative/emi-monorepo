#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhysicalAssetModelExtensionAttributes {
    AssetModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
    ),
}
impl core::fmt::Display for InsertablePhysicalAssetModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AssetModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes>
    for InsertablePhysicalAssetModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
    ) -> Self {
        Self::AssetModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhysicalAssetModelAttributes {
    Extension(InsertablePhysicalAssetModelExtensionAttributes),
    Id,
    ParentModelId,
}
impl core::str::FromStr for InsertablePhysicalAssetModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ParentModelId" => Ok(Self::ParentModelId),
            "parent_model_id" => Ok(Self::ParentModelId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertablePhysicalAssetModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ParentModelId => write!(f, "parent_model_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::physical_asset_models::physical_asset_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhysicalAssetModel {
    pub(crate) id: i32,
    pub(crate) parent_model_id: Option<i32>,
}
impl InsertablePhysicalAssetModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn parent_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(parent_model_id) = self.parent_model_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
                QueryDsl::find(
                    crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::table(),
                    parent_model_id,
                ),
                conn,
            )
            .map(Some)
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhysicalAssetModelBuilder<
    AssetModel = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
> {
    pub(crate) parent_model_id: Option<i32>,
    pub(crate) id: AssetModel,
}
/// Trait defining setters for attributes of an instance of `PhysicalAssetModel`
/// or descendant tables.
pub trait PhysicalAssetModelBuildable:
    crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
{
    /// Sets the value of the `public.physical_asset_models.parent_model_id`
    /// column.
    ///
    /// # Arguments
    /// * `parent_model_id`: The value to set for the
    ///   `public.physical_asset_models.parent_model_id` column.
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
    fn parent_model(
        self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl PhysicalAssetModelBuildable for Option<i32> {
    fn parent_model(
        self,
        _parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
        >,
> PhysicalAssetModelBuildable for InsertablePhysicalAssetModelBuilder<AssetModel> {
    ///Sets the value of the `public.physical_asset_models.parent_model_id` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model_id"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model_id"}
    ///class v1 column-of-interest
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let parent_model_id = parent_model_id
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertablePhysicalAssetModelAttributes::ParentModelId)
            })?;
        self.id = self
            .id
            .parent_model(parent_model_id)
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.parent_model_id = parent_model_id;
        Ok(self)
    }
}
impl<
    AssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertablePhysicalAssetModelBuilder<AssetModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelAttributes;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<'N, N>(
        mut self,
        name: &'N N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'N N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<
            <&'N N as TryInto<Option<String>>>::Error,
        >,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.id,
                name,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.description` column.
    fn description<'D, D>(
        mut self,
        description: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<
            <&'D D as TryInto<Option<String>>>::Error,
        >,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.id,
                description,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.parent_model_id` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model_id"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model_id"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn parent_model(
        self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as PhysicalAssetModelBuildable>::parent_model(self, parent_model_id)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.id,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_at` column.
    fn created_at<'CA, CA>(
        mut self,
        created_at: &'CA CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'CA CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'CA CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.id,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.id,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<'UA, UA>(
        mut self,
        updated_at: &'UA UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'UA UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'UA UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <AssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.id,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<AssetModel> web_common_traits::prelude::SetPrimaryKey
    for InsertablePhysicalAssetModelBuilder<AssetModel>
where
    AssetModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<AssetModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertablePhysicalAssetModelBuilder<AssetModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        Error = web_common_traits::database::InsertError<
            InsertablePhysicalAssetModelAttributes,
        >,
    >,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = InsertablePhysicalAssetModelAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
