#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialPipetteTipModelExtensionAttributes {
    PipetteTipModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes,
    ),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialPipetteTipModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PipetteTipModel(e) => write!(f, "{e}"),
            Self::CommercialProduct(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes>
    for InsertableCommercialPipetteTipModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes,
    ) -> Self {
        Self::PipetteTipModel(attribute)
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    > for InsertableCommercialPipetteTipModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialPipetteTipModelAttributes {
    Extension(InsertableCommercialPipetteTipModelExtensionAttributes),
    Id,
}
impl core::str::FromStr for InsertableCommercialPipetteTipModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialPipetteTipModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPipetteTipModel {
    pub(crate) id: i32,
}
impl InsertableCommercialPipetteTipModel {
    pub fn commercial_pipette_tip_models_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::table(
                ),
                self.id,
            ),
            conn,
        )
    }
    pub fn commercial_pipette_tip_models_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPipetteTipModelBuilder<
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    PipetteTipModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) commercial_pipette_tip_models_id_fkey: PipetteTipModel,
    pub(crate) commercial_pipette_tip_models_id_fkey1: CommercialProduct,
}
/// Trait defining setters for attributes of an instance of
/// `CommercialPipetteTipModel` or descendant tables.
pub trait CommercialPipetteTipModelBuildable:
    crate::codegen::structs_codegen::tables::insertables::PipetteTipModelBuildable
    + crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable
{
}
impl CommercialPipetteTipModelBuildable for Option<i32> {}
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    PipetteTipModel: crate::codegen::structs_codegen::tables::insertables::PipetteTipModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes,
        >,
> CommercialPipetteTipModelBuildable
for InsertableCommercialPipetteTipModelBuilder<CommercialProduct, PipetteTipModel> {}
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    PipetteTipModel: crate::codegen::structs_codegen::tables::insertables::PipetteTipModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialPipetteTipModelBuilder<CommercialProduct, PipetteTipModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelAttributes;
    #[inline]
    ///Sets the value of the `public.asset_models.most_concrete_table` column.
    fn most_concrete_table<MCT>(
        mut self,
        most_concrete_table: MCT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MCT: TryInto<String>,
        validation_errors::SingleFieldError: From<<MCT as TryInto<String>>::Error>,
    {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::most_concrete_table(
                self.commercial_pipette_tip_models_id_fkey1,
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
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.commercial_pipette_tip_models_id_fkey1,
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
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.commercial_pipette_tip_models_id_fkey1,
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
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
            self,
            parent_model_id,
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.commercial_pipette_tip_models_id_fkey1,
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
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.commercial_pipette_tip_models_id_fkey1,
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
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.commercial_pipette_tip_models_id_fkey1,
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
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.commercial_pipette_tip_models_id_fkey1,
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
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    PipetteTipModel: crate::codegen::structs_codegen::tables::insertables::PipetteTipModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable
for InsertableCommercialPipetteTipModelBuilder<CommercialProduct, PipetteTipModel> {
    #[inline]
    ///Sets the value of the `public.commercial_products.deprecation_date` column.
    fn deprecation_date<DD>(
        mut self,
        deprecation_date: DD,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        DD: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        validation_errors::SingleFieldError: From<
            <DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error,
        >,
    {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable>::deprecation_date(
                self.commercial_pipette_tip_models_id_fkey1,
                deprecation_date,
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
    ///Sets the value of the `public.commercial_products.brand_id` column.
    fn brand(
        mut self,
        brand_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable>::brand(
                self.commercial_pipette_tip_models_id_fkey1,
                brand_id,
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
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    PipetteTipModel: crate::codegen::structs_codegen::tables::insertables::PipetteTipModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialPipetteTipModelBuilder<CommercialProduct, PipetteTipModel> {
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model_id` column.
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.commercial_pipette_tip_models_id_fkey1,
                parent_model_id,
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
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    PipetteTipModel: crate::codegen::structs_codegen::tables::insertables::PipetteTipModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PipetteTipModelBuildable
for InsertableCommercialPipetteTipModelBuilder<CommercialProduct, PipetteTipModel> {}
impl<PipetteTipModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    PipetteTipModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_pipette_tip_models_id_fkey =
            self.commercial_pipette_tip_models_id_fkey.set_primary_key(primary_key);
        self.commercial_pipette_tip_models_id_fkey1 =
            self.commercial_pipette_tip_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProduct,
    PipetteTipModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialPipetteTipModelBuilder<CommercialProduct, PipetteTipModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
        Error = web_common_traits::database::InsertError<
            InsertableCommercialPipetteTipModelAttributes,
        >,
    >,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PipetteTipModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = InsertableCommercialPipetteTipModelAttributes;
    fn is_complete(&self) -> bool {
        self.commercial_pipette_tip_models_id_fkey1.is_complete()
            && self.commercial_pipette_tip_models_id_fkey.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
