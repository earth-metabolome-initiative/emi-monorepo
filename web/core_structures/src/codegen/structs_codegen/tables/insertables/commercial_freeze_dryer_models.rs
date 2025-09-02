#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialFreezeDryerModelExtensionAttributes {
    FreezeDryerModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes,
    ),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialFreezeDryerModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::FreezeDryerModel(e) => write!(f, "{e}"),
            Self::CommercialProduct(e) => write!(f, "{e}"),
        }
    }
}
impl
    From<crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes>
    for InsertableCommercialFreezeDryerModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes,
    ) -> Self {
        Self::FreezeDryerModel(attribute)
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    > for InsertableCommercialFreezeDryerModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialFreezeDryerModelAttributes {
    Extension(InsertableCommercialFreezeDryerModelExtensionAttributes),
    Id,
}
impl core::str::FromStr for InsertableCommercialFreezeDryerModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialFreezeDryerModelAttributes {
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
        table_name = crate::codegen::diesel_codegen::tables::commercial_freeze_dryer_models::commercial_freeze_dryer_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialFreezeDryerModel {
    pub(crate) id: i32,
}
impl InsertableCommercialFreezeDryerModel {
    pub fn commercial_freeze_dryer_models_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn commercial_freeze_dryer_models_id_fkey1<
        C: diesel::connection::LoadConnection,
    >(
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
pub struct InsertableCommercialFreezeDryerModelBuilder<
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    FreezeDryerModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) commercial_freeze_dryer_models_id_fkey: FreezeDryerModel,
    pub(crate) commercial_freeze_dryer_models_id_fkey1: CommercialProduct,
}
/// Trait defining setters for attributes of an instance of
/// `CommercialFreezeDryerModel` or descendant tables.
pub trait CommercialFreezeDryerModelBuildable:
    crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelBuildable
    + crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable
{
}
impl CommercialFreezeDryerModelBuildable for Option<i32> {}
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    FreezeDryerModel: crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes,
        >,
> CommercialFreezeDryerModelBuildable
for InsertableCommercialFreezeDryerModelBuilder<CommercialProduct, FreezeDryerModel> {}
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    FreezeDryerModel: crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialFreezeDryerModelBuilder<CommercialProduct, FreezeDryerModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerModelAttributes;
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::most_concrete_table(
                self.commercial_freeze_dryer_models_id_fkey1,
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.commercial_freeze_dryer_models_id_fkey1,
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.commercial_freeze_dryer_models_id_fkey1,
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.commercial_freeze_dryer_models_id_fkey1,
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.commercial_freeze_dryer_models_id_fkey1,
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.commercial_freeze_dryer_models_id_fkey1,
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.commercial_freeze_dryer_models_id_fkey1,
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
    FreezeDryerModel: crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable
for InsertableCommercialFreezeDryerModelBuilder<CommercialProduct, FreezeDryerModel> {
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable>::deprecation_date(
                self.commercial_freeze_dryer_models_id_fkey1,
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
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable>::brand(
                self.commercial_freeze_dryer_models_id_fkey1,
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
    FreezeDryerModel: crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelBuildable
for InsertableCommercialFreezeDryerModelBuilder<CommercialProduct, FreezeDryerModel> {}
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
    FreezeDryerModel: crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryerModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialFreezeDryerModelBuilder<CommercialProduct, FreezeDryerModel> {
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model_id` column.
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_freeze_dryer_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.commercial_freeze_dryer_models_id_fkey1,
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
impl<FreezeDryerModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialFreezeDryerModelBuilder<FreezeDryerModel, CommercialProduct>
where
    FreezeDryerModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_freeze_dryer_models_id_fkey =
            self.commercial_freeze_dryer_models_id_fkey.set_primary_key(primary_key);
        self.commercial_freeze_dryer_models_id_fkey1 =
            self.commercial_freeze_dryer_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProduct,
    FreezeDryerModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialFreezeDryerModelBuilder<CommercialProduct, FreezeDryerModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
        Error = web_common_traits::database::InsertError<
            InsertableCommercialFreezeDryerModelAttributes,
        >,
    >,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    FreezeDryerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = InsertableCommercialFreezeDryerModelAttributes;
    fn is_complete(&self) -> bool {
        self.commercial_freeze_dryer_models_id_fkey1.is_complete()
            && self.commercial_freeze_dryer_models_id_fkey.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
