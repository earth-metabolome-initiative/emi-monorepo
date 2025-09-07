#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialBallMillMachineLotExtensionAttribute {
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ),
    BallMillMachineModel(
        crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute,
    ),
}
impl core::fmt::Display for CommercialBallMillMachineLotExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CommercialProductLot(e) => write!(f, "{e}"),
            Self::BallMillMachineModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute>
    for CommercialBallMillMachineLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ) -> Self {
        Self::CommercialProductLot(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute>
    for CommercialBallMillMachineLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute,
    ) -> Self {
        Self::BallMillMachineModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialBallMillMachineLotAttribute {
    Extension(CommercialBallMillMachineLotExtensionAttribute),
    Id,
    ProductModel,
}
impl core::str::FromStr for CommercialBallMillMachineLotAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProductModel" => Ok(Self::ProductModel),
            "product_model" => Ok(Self::ProductModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for CommercialBallMillMachineLotAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ProductModel => write!(f, "product_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_ball_mill_machine_lots::commercial_ball_mill_machine_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialBallMillMachineLot {
    pub(crate) id: i32,
    pub(crate) product_model: i32,
}
impl InsertableCommercialBallMillMachineLot {
    pub fn commercial_ball_mill_machine_lots_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CommercialProductLot, diesel::result::Error>
    where
        crate::CommercialProductLot: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CommercialProductLot::read(self.id, conn)
    }
    pub fn commercial_ball_mill_machine_lots_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BallMillMachineModel, diesel::result::Error>
    where
        crate::BallMillMachineModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BallMillMachineModel::read(self.id, conn)
    }
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CommercialBallMillMachineModel, diesel::result::Error>
    where
        crate::CommercialBallMillMachineModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CommercialBallMillMachineModel::read(self.product_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn commercial_ball_mill_machine_lots_id_product_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::AssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::AssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::id
                    .eq(&self.id)
                    .and(
                        crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::parent_model
                            .eq(&self.product_model),
                    ),
            )
            .first::<crate::AssetModel>(conn)
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialBallMillMachineLotBuilder<
    BallMillMachineModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillMachineModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    CommercialProductLot
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
            Option<i32>,
        >,
> {
    pub(crate) product_model: Option<i32>,
    pub(crate) commercial_ball_mill_machine_lots_id_fkey: CommercialProductLot,
    pub(crate) commercial_ball_mill_machine_lots_id_fkey1: BallMillMachineModel,
}
impl From<InsertableCommercialBallMillMachineLotBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialBallMillMachineLotBuilder>
{
    fn from(builder: InsertableCommercialBallMillMachineLotBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialBallMillMachineLot` or descendant tables.
pub trait CommercialBallMillMachineLotSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.commercial_ball_mill_machine_lots.product_model` column.
    ///
    /// # Arguments
    /// * `product_model`: The value to set for the
    ///   `public.commercial_ball_mill_machine_lots.product_model` column.
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
    fn product_model(
        self,
        product_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute,
        >,
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
> CommercialBallMillMachineLotSettable
for InsertableCommercialBallMillMachineLotBuilder<
    BallMillMachineModel,
    CommercialProductLot,
> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute;
    ///Sets the value of the `public.commercial_ball_mill_machine_lots.product_model` column.
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v4 ["`asset_models`"]
    ///    v3@{shape: rounded, label: "parent_model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`commercial_ball_mill_machine_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v6 ["`commercial_product_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v7 ["`physical_asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v3
    ///v1 -.->|"`inferred ancestral same as`"| v2
    ///v0 --->|"`ancestral same as`"| v3
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v0 -.->|"`inferred ancestral same as`"| v2
    ///v2 --->|"`ancestral same as`"| v3
    ///v7 --->|"`extends`"| v4
    ///v6 --->|"`extends`"| v7
    ///v5 --->|"`extends`"| v6
    ///```
    fn product_model(
        mut self,
        product_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_ball_mill_machine_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::product_model(
                self.commercial_ball_mill_machine_lots_id_fkey,
                product_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.commercial_ball_mill_machine_lots_id_fkey1 = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_ball_mill_machine_lots_id_fkey1,
                Some(product_model),
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.product_model = Some(product_model);
        Ok(self)
    }
}
impl<
    BallMillMachineModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelAttribute,
        >,
    CommercialProductLot,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialBallMillMachineLotBuilder<
    BallMillMachineModel,
    CommercialProductLot,
>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.commercial_ball_mill_machine_lots_id_fkey1 = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_ball_mill_machine_lots_id_fkey1,
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
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.commercial_ball_mill_machine_lots_id_fkey1 = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_ball_mill_machine_lots_id_fkey1,
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
    ///flowchart BT
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
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
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
        self.commercial_ball_mill_machine_lots_id_fkey1 = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_ball_mill_machine_lots_id_fkey1,
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
        self.commercial_ball_mill_machine_lots_id_fkey1 = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_ball_mill_machine_lots_id_fkey1,
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
        self.commercial_ball_mill_machine_lots_id_fkey1 = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_ball_mill_machine_lots_id_fkey1,
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
        self.commercial_ball_mill_machine_lots_id_fkey1 = <BallMillMachineModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_ball_mill_machine_lots_id_fkey1,
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
impl<BallMillMachineModel, CommercialProductLot>
    crate::codegen::structs_codegen::tables::insertables::BallMillMachineModelSettable
    for InsertableCommercialBallMillMachineLotBuilder<BallMillMachineModel, CommercialProductLot>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute;
}
impl<
    BallMillMachineModel,
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable
for InsertableCommercialBallMillMachineLotBuilder<
    BallMillMachineModel,
    CommercialProductLot,
>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute;
    #[inline]
    ///Sets the value of the `public.commercial_product_lots.lot` column.
    fn lot<L>(
        mut self,
        lot: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<String>,
        validation_errors::SingleFieldError: From<<L as TryInto<String>>::Error>,
    {
        self.commercial_ball_mill_machine_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::lot(
                self.commercial_ball_mill_machine_lots_id_fkey,
                lot,
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
    ///Sets the value of the `public.commercial_product_lots.product_model` column.
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`commercial_ball_mill_machine_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v4 ["`commercial_product_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v1 -.->|"`inferred ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v2
    ///v4 --->|"`extends`"| v5
    ///v3 --->|"`extends`"| v4
    ///```
    fn product_model(
        self,
        product_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialBallMillMachineLotSettable>::product_model(
            self,
            product_model,
        )
    }
}
impl<
    BallMillMachineModel,
    CommercialProductLot,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialBallMillMachineLotBuilder<
    BallMillMachineModel,
    CommercialProductLot,
>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineLotAttribute;
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
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v4 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v5 ["`commercial_ball_mill_machine_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v6 ["`commercial_product_lots`"]
    ///    v3@{shape: rounded, label: "product_model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v7 ["`physical_asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v1 --->|"`ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v3
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v3 --->|"`ancestral same as`"| v2
    ///v3 -.->|"`inferred ancestral same as`"| v0
    ///v5 --->|"`extends`"| v6
    ///v6 --->|"`extends`"| v7
    ///v7 --->|"`extends`"| v4
    ///```
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialBallMillMachineLotSettable>::product_model(
            self,
            parent_model
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::ProductModel,
                    ),
                )?,
        )
    }
}
impl<CommercialProductLot, BallMillMachineModel> web_common_traits::database::MostConcreteTable
    for InsertableCommercialBallMillMachineLotBuilder<CommercialProductLot, BallMillMachineModel>
where
    CommercialProductLot: web_common_traits::database::MostConcreteTable,
    BallMillMachineModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_ball_mill_machine_lots_id_fkey.set_most_concrete_table(table_name);
        self.commercial_ball_mill_machine_lots_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<CommercialProductLot, BallMillMachineModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialBallMillMachineLotBuilder<CommercialProductLot, BallMillMachineModel>
where
    CommercialProductLot: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    BallMillMachineModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_ball_mill_machine_lots_id_fkey =
            self.commercial_ball_mill_machine_lots_id_fkey.set_primary_key(primary_key);
        self.commercial_ball_mill_machine_lots_id_fkey1 =
            self.commercial_ball_mill_machine_lots_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<BallMillMachineModel, CommercialProductLot, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableCommercialBallMillMachineLotBuilder<BallMillMachineModel, CommercialProductLot>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::CommercialBallMillMachineLot,
            Error = web_common_traits::database::InsertError<CommercialBallMillMachineLotAttribute>,
        >,
    BallMillMachineModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = CommercialBallMillMachineLotAttribute;
    fn is_complete(&self) -> bool {
        self.commercial_ball_mill_machine_lots_id_fkey1.is_complete()
            && self.commercial_ball_mill_machine_lots_id_fkey.is_complete()
            && self.product_model.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::CommercialBallMillMachineLot = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
