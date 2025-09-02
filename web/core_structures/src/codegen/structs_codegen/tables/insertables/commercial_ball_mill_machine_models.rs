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
    ParentModel,
}
impl core::str::FromStr for InsertableCommercialBallMillMachineModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ParentModel" => Ok(Self::ParentModel),
            "parent_model" => Ok(Self::ParentModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialBallMillMachineModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ParentModel => write!(f, "parent_model"),
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
    pub(crate) parent_model: i32,
}
impl InsertableCommercialBallMillMachineModel {
    pub fn parent_model<C: diesel::connection::LoadConnection>(
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
                self.parent_model,
            ),
            conn,
        )
    }
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
    pub(crate) parent_model: Option<i32>,
    pub(crate) commercial_ball_mill_machine_models_id_fkey: BallMillMachineModel,
    pub(crate) commercial_ball_mill_machine_models_id_fkey1: CommercialProduct,
}
/// Trait defining setters for attributes of an instance of
/// `CommercialBallMillMachineModel` or descendant tables.
pub trait CommercialBallMillMachineModelBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.commercial_ball_mill_machine_models.parent_model` column.
    ///
    /// # Arguments
    /// * `parent_model`: The value to set for the
    ///   `public.commercial_ball_mill_machine_models.parent_model` column.
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
        parent_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
        >,
    CommercialProduct,
> CommercialBallMillMachineModelBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes;
    ///Sets the value of the `public.commercial_ball_mill_machine_models.parent_model` column.
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_ball_mill_machine_models`"]
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v1 --->|"`ancestral same as`"| v2
    ///v5 --->|"`extends`"| v3
    ///v4 -.->|"`descendant of`"| v3
    ///v4 -.->|"`descendant of`"| v5
    ///```
    fn parent_model(
        mut self,
        parent_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let parent_model = parent_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableCommercialBallMillMachineModelAttributes::ParentModel,
                    )
            })?;
        self.commercial_ball_mill_machine_models_id_fkey = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.commercial_ball_mill_machine_models_id_fkey,
                Some(parent_model),
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.parent_model = Some(parent_model);
        Ok(self)
    }
}
impl<
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelAttributes,
        >,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes;
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
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
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
    ///Sets the value of the `public.asset_models.parent_model` column.
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
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
            self,
            parent_model,
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
impl<BallMillMachineModel, CommercialProduct>
    crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelBuildable
    for InsertableCommercialBallMillMachineModelBuilder<BallMillMachineModel, CommercialProduct>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes;
}
impl<
    BallMillMachineModel,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes;
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
    BallMillMachineModel,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelAttributes;
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model` column.
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_ball_mill_machine_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v1 --->|"`ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v5 --->|"`extends`"| v3
    ///v4 -.->|"`descendant of`"| v3
    ///v4 -.->|"`descendant of`"| v5
    ///```
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialBallMillMachineModelBuildable>::parent_model(
            self,
            parent_model
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::ParentModel,
                    ),
                )?,
        )
    }
}
impl<BallMillMachineModel, CommercialProduct> web_common_traits::database::MostConcreteTable
    for InsertableCommercialBallMillMachineModelBuilder<BallMillMachineModel, CommercialProduct>
where
    BallMillMachineModel: web_common_traits::database::MostConcreteTable,
    CommercialProduct: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_ball_mill_machine_models_id_fkey.set_most_concrete_table(table_name);
        self.commercial_ball_mill_machine_models_id_fkey1.set_most_concrete_table(table_name);
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
            && self.parent_model.is_some()
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
