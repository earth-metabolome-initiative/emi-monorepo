#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialPackagingLotExtensionAttributes {
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    ),
    PackagingModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialPackagingLotExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CommercialProductLot(e) => write!(f, "{e}"),
            Self::PackagingModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
> for InsertableCommercialPackagingLotExtensionAttributes {
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    ) -> Self {
        Self::CommercialProductLot(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes>
    for InsertableCommercialPackagingLotExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes,
    ) -> Self {
        Self::PackagingModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialPackagingLotAttributes {
    Extension(InsertableCommercialPackagingLotExtensionAttributes),
    Id,
    ProductModelId,
}
impl core::str::FromStr for InsertableCommercialPackagingLotAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProductModelId" => Ok(Self::ProductModelId),
            "product_model_id" => Ok(Self::ProductModelId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialPackagingLotAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ProductModelId => write!(f, "product_model_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_packaging_lots::commercial_packaging_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPackagingLot {
    pub(crate) id: i32,
    pub(crate) product_model_id: i32,
}
impl InsertableCommercialPackagingLot {
    pub fn commercial_packaging_lots_id_fkey<C: diesel::connection::LoadConnection>(
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
    pub fn commercial_packaging_lots_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::packaging_models::PackagingModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel::table(),
                self.product_model_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPackagingLotBuilder<
    CommercialProductLot
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    PackagingModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) product_model_id: Option<i32>,
    pub(crate) commercial_packaging_lots_id_fkey: CommercialProductLot,
    pub(crate) commercial_packaging_lots_id_fkey1: PackagingModel,
}
/// Trait defining setters for attributes of an instance of
/// `CommercialPackagingLot` or descendant tables.
pub trait CommercialPackagingLotBuildable:
    crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable
    + crate::codegen::structs_codegen::tables::insertables::PackagingModelBuildable
{
    /// Sets the value of the
    /// `public.commercial_packaging_lots.product_model_id` column.
    ///
    /// # Arguments
    /// * `product_model_id`: The value to set for the
    ///   `public.commercial_packaging_lots.product_model_id` column.
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
        product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl CommercialPackagingLotBuildable for Option<i32> {
    fn product_model(
        self,
        _product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    PackagingModel: crate::codegen::structs_codegen::tables::insertables::PackagingModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes,
        >,
> CommercialPackagingLotBuildable
for InsertableCommercialPackagingLotBuilder<CommercialProductLot, PackagingModel> {
    ///Sets the value of the `public.commercial_packaging_lots.product_model_id` column.
    fn product_model(
        mut self,
        product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let product_model_id = product_model_id
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableCommercialPackagingLotAttributes::ProductModelId,
                    )
            })?;
        self.product_model_id = Some(product_model_id);
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    PackagingModel: crate::codegen::structs_codegen::tables::insertables::PackagingModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialPackagingLotBuilder<CommercialProductLot, PackagingModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotAttributes;
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
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.commercial_packaging_lots_id_fkey,
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
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.commercial_packaging_lots_id_fkey,
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
    ///subgraph v3 ["`commercial_packaging_lots`"]
    ///    v1@{shape: rounded, label: "product_model_id"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 -.->|"`descendant of`"| v2
    ///```
    fn parent_model(
        self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialPackagingLotBuildable>::product_model(
            self,
            parent_model_id
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::ProductModelId,
                    ),
                )?,
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.commercial_packaging_lots_id_fkey,
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
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.commercial_packaging_lots_id_fkey,
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
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.commercial_packaging_lots_id_fkey,
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
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.commercial_packaging_lots_id_fkey,
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
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    PackagingModel: crate::codegen::structs_codegen::tables::insertables::PackagingModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable
for InsertableCommercialPackagingLotBuilder<CommercialProductLot, PackagingModel> {
    #[inline]
    ///Sets the value of the `public.commercial_product_lots.lot` column.
    fn lot<'L, L>(
        mut self,
        lot: &'L L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'L L: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'L L as TryInto<String>>::Error>,
    {
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable>::lot(
                self.commercial_packaging_lots_id_fkey,
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
    ///Sets the value of the `public.commercial_product_lots.product_model_id` column.
    fn product_model(
        mut self,
        product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable>::product_model(
                self.commercial_packaging_lots_id_fkey,
                product_model_id,
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
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    PackagingModel: crate::codegen::structs_codegen::tables::insertables::PackagingModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PackagingModelBuildable
for InsertableCommercialPackagingLotBuilder<CommercialProductLot, PackagingModel> {}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    PackagingModel: crate::codegen::structs_codegen::tables::insertables::PackagingModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialPackagingLotBuilder<CommercialProductLot, PackagingModel> {
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model_id` column.
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_packaging_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.commercial_packaging_lots_id_fkey,
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
impl<CommercialProductLot, PackagingModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialPackagingLotBuilder<CommercialProductLot, PackagingModel>
where
    CommercialProductLot: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    PackagingModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_packaging_lots_id_fkey =
            self.commercial_packaging_lots_id_fkey.set_primary_key(primary_key);
        self.commercial_packaging_lots_id_fkey1 =
            self.commercial_packaging_lots_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProductLot,
    PackagingModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialPackagingLotBuilder<CommercialProductLot, PackagingModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
        Error = web_common_traits::database::InsertError<
            InsertableCommercialPackagingLotAttributes,
        >,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PackagingModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attributes = InsertableCommercialPackagingLotAttributes;
    fn is_complete(&self) -> bool {
        self.commercial_packaging_lots_id_fkey.is_complete()
            && self.commercial_packaging_lots_id_fkey1.is_complete()
            && self.product_model_id.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
