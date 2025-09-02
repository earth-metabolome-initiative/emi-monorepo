#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDigitalAssetExtensionAttributes {
    Asset(crate::codegen::structs_codegen::tables::insertables::InsertableAssetAttributes),
}
impl core::fmt::Display for InsertableDigitalAssetExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Asset(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableAssetAttributes>
    for InsertableDigitalAssetExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableAssetAttributes,
    ) -> Self {
        Self::Asset(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDigitalAssetAttributes {
    Extension(InsertableDigitalAssetExtensionAttributes),
    Id,
    ModelId,
}
impl core::str::FromStr for InsertableDigitalAssetAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ModelId" => Ok(Self::ModelId),
            "model_id" => Ok(Self::ModelId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableDigitalAssetAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ModelId => write!(f, "model_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::digital_assets::digital_assets
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableDigitalAsset {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) model_id: i32,
}
impl InsertableDigitalAsset {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::assets::Asset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::assets::Asset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::assets::Asset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::assets::Asset::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel::table(),
                self.model_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableDigitalAssetBuilder<
    Asset = crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
> {
    pub(crate) model_id: Option<i32>,
    pub(crate) id: Asset,
}
/// Trait defining setters for attributes of an instance of `DigitalAsset` or
/// descendant tables.
pub trait DigitalAssetBuildable:
    crate::codegen::structs_codegen::tables::insertables::AssetBuildable
{
    /// Sets the value of the `public.digital_assets.model_id` column.
    ///
    /// # Arguments
    /// * `model_id`: The value to set for the `public.digital_assets.model_id`
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
    fn model(
        self,
        model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl DigitalAssetBuildable for Option<::rosetta_uuid::Uuid> {
    fn model(
        self,
        _model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    Asset: crate::codegen::structs_codegen::tables::insertables::AssetBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetAttributes,
        >,
> DigitalAssetBuildable for InsertableDigitalAssetBuilder<Asset> {
    ///Sets the value of the `public.digital_assets.model_id` column.
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
    ///subgraph v2 ["`assets`"]
    ///    v0@{shape: rounded, label: "model_id"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v3 ["`digital_assets`"]
    ///    v1@{shape: rounded, label: "model_id"}
    ///class v1 column-of-interest
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn model(
        mut self,
        model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let model_id = model_id
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableDigitalAssetAttributes::ModelId)
            })?;
        self.id = self
            .id
            .model(model_id)
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.model_id = Some(model_id);
        Ok(self)
    }
}
impl<
    Asset: crate::codegen::structs_codegen::tables::insertables::AssetBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetBuildable
for InsertableDigitalAssetBuilder<Asset> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetAttributes;
    #[inline]
    ///Sets the value of the `public.assets.id` column.
    fn id(
        mut self,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::id(
                self.id,
                id,
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
    ///Sets the value of the `public.assets.most_concrete_table` column.
    fn most_concrete_table<MCT>(
        mut self,
        most_concrete_table: MCT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MCT: TryInto<String>,
        validation_errors::SingleFieldError: From<<MCT as TryInto<String>>::Error>,
    {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::most_concrete_table(
                self.id,
                most_concrete_table,
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
    ///Sets the value of the `public.assets.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::name(
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
    ///Sets the value of the `public.assets.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::description(
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
    ///Sets the value of the `public.assets.model_id` column.
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
    ///subgraph v2 ["`assets`"]
    ///    v0@{shape: rounded, label: "model_id"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`digital_assets`"]
    ///    v1@{shape: rounded, label: "model_id"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn model(
        self,
        model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as DigitalAssetBuildable>::model(self, model_id)
    }
    #[inline]
    ///Sets the value of the `public.assets.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::created_by(
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
    ///Sets the value of the `public.assets.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::created_at(
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
    ///Sets the value of the `public.assets.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::updated_by(
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
    ///Sets the value of the `public.assets.updated_at` column.
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <Asset as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::updated_at(
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
impl<Asset> web_common_traits::prelude::SetPrimaryKey for InsertableDigitalAssetBuilder<Asset>
where
    Asset: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<Asset, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableDigitalAssetBuilder<Asset>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset,
            Error = web_common_traits::database::InsertError<InsertableDigitalAssetAttributes>,
        >,
    Asset: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableDigitalAssetAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.model_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
