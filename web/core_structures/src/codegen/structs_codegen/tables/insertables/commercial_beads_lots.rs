#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialBeadsLotExtensionAttributes {
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    ),
    BeadsModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialBeadsLotExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CommercialProductLot(e) => write!(f, "{e}"),
            Self::BeadsModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
> for InsertableCommercialBeadsLotExtensionAttributes {
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    ) -> Self {
        Self::CommercialProductLot(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes>
    for InsertableCommercialBeadsLotExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes,
    ) -> Self {
        Self::BeadsModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialBeadsLotAttributes {
    Extension(InsertableCommercialBeadsLotExtensionAttributes),
    Id,
    ProductModel,
}
impl core::str::FromStr for InsertableCommercialBeadsLotAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProductModel" => Ok(Self::ProductModel),
            "product_model" => Ok(Self::ProductModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialBeadsLotAttributes {
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
        table_name = crate::codegen::diesel_codegen::tables::commercial_beads_lots::commercial_beads_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialBeadsLot {
    pub(crate) id: i32,
    pub(crate) product_model: i32,
}
impl InsertableCommercialBeadsLot {
    pub fn commercial_beads_lots_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn commercial_beads_lots_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::beads_models::BeadsModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::beads_models::BeadsModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::beads_models::BeadsModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::beads_models::BeadsModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel::table(),
                self.product_model,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialBeadsLotBuilder<
    BeadsModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelBuilder<
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
    pub(crate) commercial_beads_lots_id_fkey: CommercialProductLot,
    pub(crate) commercial_beads_lots_id_fkey1: BeadsModel,
}
/// Trait defining setters for attributes of an instance of `CommercialBeadsLot`
/// or descendant tables.
pub trait CommercialBeadsLotBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.commercial_beads_lots.product_model`
    /// column.
    ///
    /// # Arguments
    /// * `product_model`: The value to set for the
    ///   `public.commercial_beads_lots.product_model` column.
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
    BeadsModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes,
        >,
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
> CommercialBeadsLotBuildable
for InsertableCommercialBeadsLotBuilder<BeadsModel, CommercialProductLot> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes;
    ///Sets the value of the `public.commercial_beads_lots.product_model` column.
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
    ///subgraph v4 ["`asset_models`"]
    ///    v3@{shape: rounded, label: "parent_model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`commercial_beads_lots`"]
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
    ///v5 --->|"`extends`"| v6
    ///v5 -.->|"`descendant of`"| v4
    ///v5 -.->|"`descendant of`"| v7
    ///v7 --->|"`extends`"| v4
    ///v6 --->|"`extends`"| v7
    ///v6 -.->|"`descendant of`"| v4
    ///```
    fn product_model(
        mut self,
        product_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_beads_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable>::product_model(
                self.commercial_beads_lots_id_fkey,
                product_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.commercial_beads_lots_id_fkey1,
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
    BeadsModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes,
        >,
    CommercialProductLot,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialBeadsLotBuilder<BeadsModel, CommercialProductLot>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes;
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
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.commercial_beads_lots_id_fkey1,
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
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.commercial_beads_lots_id_fkey1,
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
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.commercial_beads_lots_id_fkey1,
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
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.commercial_beads_lots_id_fkey1,
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
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.commercial_beads_lots_id_fkey1,
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
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.commercial_beads_lots_id_fkey1,
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
    BeadsModel: crate::codegen::structs_codegen::tables::insertables::BeadsModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes,
        >,
    CommercialProductLot,
> crate::codegen::structs_codegen::tables::insertables::BeadsModelBuildable
for InsertableCommercialBeadsLotBuilder<BeadsModel, CommercialProductLot> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes;
    #[inline]
    ///Sets the value of the `public.beads_models.diameter_millimeters` column.
    fn diameter_millimeters<DM>(
        mut self,
        diameter_millimeters: DM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        DM: TryInto<f32>,
        validation_errors::SingleFieldError: From<<DM as TryInto<f32>>::Error>,
    {
        self.commercial_beads_lots_id_fkey1 = <BeadsModel as crate::codegen::structs_codegen::tables::insertables::BeadsModelBuildable>::diameter_millimeters(
                self.commercial_beads_lots_id_fkey1,
                diameter_millimeters,
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
    BeadsModel,
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable
for InsertableCommercialBeadsLotBuilder<BeadsModel, CommercialProductLot>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialBeadsLotBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes;
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
        self.commercial_beads_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable>::lot(
                self.commercial_beads_lots_id_fkey,
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
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`commercial_beads_lots`"]
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
    ///v3 --->|"`extends`"| v4
    ///v3 -.->|"`descendant of`"| v5
    ///v4 --->|"`extends`"| v5
    ///```
    fn product_model(
        self,
        product_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialBeadsLotBuildable>::product_model(self, product_model)
    }
}
impl<
    BeadsModel,
    CommercialProductLot,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialBeadsLotBuilder<BeadsModel, CommercialProductLot>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialBeadsLotBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsLotAttributes;
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
    ///subgraph v4 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v5 ["`commercial_beads_lots`"]
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
    ///v7 --->|"`extends`"| v4
    ///v5 --->|"`extends`"| v6
    ///v5 -.->|"`descendant of`"| v4
    ///v5 -.->|"`descendant of`"| v7
    ///v6 --->|"`extends`"| v7
    ///v6 -.->|"`descendant of`"| v4
    ///```
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialBeadsLotBuildable>::product_model(
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
impl<CommercialProductLot, BeadsModel> web_common_traits::database::MostConcreteTable
    for InsertableCommercialBeadsLotBuilder<CommercialProductLot, BeadsModel>
where
    CommercialProductLot: web_common_traits::database::MostConcreteTable,
    BeadsModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_beads_lots_id_fkey.set_most_concrete_table(table_name);
        self.commercial_beads_lots_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<CommercialProductLot, BeadsModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialBeadsLotBuilder<CommercialProductLot, BeadsModel>
where
    CommercialProductLot: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    BeadsModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_beads_lots_id_fkey =
            self.commercial_beads_lots_id_fkey.set_primary_key(primary_key);
        self.commercial_beads_lots_id_fkey1 =
            self.commercial_beads_lots_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    BeadsModel,
    CommercialProductLot,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialBeadsLotBuilder<BeadsModel, CommercialProductLot>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_beads_lots::CommercialBeadsLot,
        Error = web_common_traits::database::InsertError<
            InsertableCommercialBeadsLotAttributes,
        >,
    >,
    BeadsModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Attributes = InsertableCommercialBeadsLotAttributes;
    fn is_complete(&self) -> bool {
        self.commercial_beads_lots_id_fkey1.is_complete()
            && self.commercial_beads_lots_id_fkey.is_complete()
            && self.product_model.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_beads_lots::CommercialBeadsLot = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
