#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialReagentExtensionAttributes {
    Processable(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialReagentExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Processable(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialReagentAttributes {
    Extension(InsertableCommercialReagentExtensionAttributes),
    Id,
    CommercialProductLotId,
}
impl core::fmt::Display for InsertableCommercialReagentAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::CommercialProductLotId => write!(f, "commercial_product_lot_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialReagent {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) commercial_product_lot_id: ::rosetta_uuid::Uuid,
}
impl InsertableCommercialReagent {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::processables::Processable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::processables::Processable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::processables::Processable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::processables::Processable::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn commercial_product_lot<C: diesel::connection::LoadConnection>(
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
                self.commercial_product_lot_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialReagentBuilder<
    Processable
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
> {
    pub(crate) commercial_product_lot_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) id: Processable,
}
impl<Processable> web_common_traits::database::ExtendableBuilder
for InsertableCommercialReagentBuilder<Processable>
where
    Processable: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes,
    >,
{
    type Attributes = InsertableCommercialReagentAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(
                        attribute,
                    ),
                ))
            })?;
        if let Some(commercial_product_lot_id) = other.commercial_product_lot_id {
            self = self.commercial_product_lot(commercial_product_lot_id)?;
        }
        Ok(self)
    }
}
impl<Processable> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialReagentBuilder<Processable>
where
    Processable: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<Processable>
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        Processable,
    >
{
    /// Sets the value of the `commercial_reagents.commercial_product_lot_id`
    /// column from table `commercial_reagents`.
    pub fn commercial_product_lot(
        mut self,
        commercial_product_lot_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    {
        self.commercial_product_lot_id = Some(commercial_product_lot_id);
        Ok(self)
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            Trackable,
        >,
    >
{
    /// Sets the value of the `processables.kilograms` column from table
    /// `commercial_reagents`.
    pub fn kilograms<P>(
        mut self,
        kilograms: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.kilograms(kilograms).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `commercial_reagents`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `commercial_reagents`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    {
        self.id = self.id.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `commercial_reagents`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.id` column from table
    /// `commercial_reagents`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.name` column from table
    /// `commercial_reagents`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `commercial_reagents`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    {
        self.id = self.id.parent(parent_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `commercial_reagents`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    {
        self.id = self.id.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `commercial_reagents`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `commercial_reagents`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>>
    {
        self.id = self.id.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableCommercialReagentAttributes::Extension(
                    InsertableCommercialReagentExtensionAttributes::Processable(attribute),
                )
            })
        })?;
        Ok(self)
    }
}
impl<Processable, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableCommercialReagentBuilder<Processable>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent,
            Error = web_common_traits::database::InsertError<InsertableCommercialReagentAttributes>,
        >,
    Processable:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableCommercialReagentAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.commercial_product_lot_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
