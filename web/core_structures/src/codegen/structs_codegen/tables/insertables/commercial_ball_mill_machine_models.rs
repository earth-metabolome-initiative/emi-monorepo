#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialBallMillMachineModelExtensionAttributes {
    BallMillMachineModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
    ),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialBallMillMachineModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::BallMillMachineModel(e) => write!(f, "{e}"),
            Self::CommercialProduct(e) => write!(f, "{e}"),
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
> for InsertableCommercialBallMillMachineModelExtensionAttributes {
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
    ) -> Self {
        Self::BallMillMachineModel(attribute)
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    > for InsertableCommercialBallMillMachineModelExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialBallMillMachineModelAttributes {
    Extension(InsertableCommercialBallMillMachineModelExtensionAttributes),
    Id,
}
impl core::str::FromStr for InsertableCommercialBallMillMachineModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialBallMillMachineModelAttributes {
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
        table_name = crate::codegen::diesel_codegen::tables::commercial_ball_mill_machine_models::commercial_ball_mill_machine_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialBallMillMachineModel {
    pub(crate) id: i32,
}
impl InsertableCommercialBallMillMachineModel {
    pub fn commercial_ball_mill_machine_models_id_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn commercial_ball_mill_machine_models_id_fkey1<
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
pub struct InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            Option<i32>,
        >,
> {
    pub(crate) commercial_ball_mill_machine_models_id_fkey: BallMillMachineModel,
    pub(crate) commercial_ball_mill_machine_models_id_fkey1: CommercialProduct,
}
/// Trait defining setters for attributes of an instance of
/// `CommercialBallMillMachineModel` or descendant tables.
pub trait CommercialBallMillMachineModelBuildable:
    crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable
    + crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable
{
}
impl CommercialBallMillMachineModelBuildable for Option<i32> {}
impl<
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
        >,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
> CommercialBallMillMachineModelBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {}
impl<
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
        >,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes;
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
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.commercial_ball_mill_machine_models_id_fkey,
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
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.commercial_ball_mill_machine_models_id_fkey,
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
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.commercial_ball_mill_machine_models_id_fkey,
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
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.commercial_ball_mill_machine_models_id_fkey,
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
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.commercial_ball_mill_machine_models_id_fkey,
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
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.commercial_ball_mill_machine_models_id_fkey,
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
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
        >,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {}
impl<
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
        >,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {
    #[inline]
    ///Sets the value of the `public.commercial_products.deprecation_date` column.
    fn deprecation_date<'DD, DD>(
        mut self,
        deprecation_date: &'DD DD,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'DD DD: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        validation_errors::SingleFieldError: From<
            <&'DD DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error,
        >,
    {
        self.commercial_ball_mill_machine_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable>::deprecation_date(
                self.commercial_ball_mill_machine_models_id_fkey1,
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
        self.commercial_ball_mill_machine_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable>::brand(
                self.commercial_ball_mill_machine_models_id_fkey1,
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
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
        >,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model_id` column.
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.commercial_ball_mill_machine_models_id_fkey,
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
impl<BallMillMachineModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialBallMillMachineModelBuilder<BallMillMachineModel, CommercialProduct>
where
    BallMillMachineModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_ball_mill_machine_models_id_fkey =
            self.commercial_ball_mill_machine_models_id_fkey.set_primary_key(primary_key);
        self.commercial_ball_mill_machine_models_id_fkey1 =
            self.commercial_ball_mill_machine_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    BallMillMachineModel,
    CommercialProduct,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
        Error = web_common_traits::database::InsertError<
            InsertableCommercialBallMillMachineModelAttributes,
        >,
    >,
    BallMillMachineModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Attributes = InsertableCommercialBallMillMachineModelAttributes;
    fn is_complete(&self) -> bool {
        self.commercial_ball_mill_machine_models_id_fkey.is_complete()
            && self.commercial_ball_mill_machine_models_id_fkey1.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
