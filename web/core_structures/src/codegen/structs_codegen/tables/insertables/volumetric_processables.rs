#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricProcessableExtensionAttributes {
    Processable(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes,
    ),
}
impl core::fmt::Display for InsertableVolumetricProcessableExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Processable(e) => write!(f, "Processable.{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricProcessableAttributes {
    Extension(InsertableVolumetricProcessableExtensionAttributes),
    Id,
    Liters,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes>
    for InsertableVolumetricProcessableAttributes
{
    fn from(
        processables: crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes,
    ) -> Self {
        Self::Extension(InsertableVolumetricProcessableExtensionAttributes::Processable(
            processables,
        ))
    }
}
impl core::fmt::Display for InsertableVolumetricProcessableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::Liters => write!(f, "liters"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::volumetric_processables::volumetric_processables
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableVolumetricProcessable {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) liters: f32,
}
impl InsertableVolumetricProcessable {
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
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableVolumetricProcessableBuilder<
    Processable
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
> {
    pub(crate) liters: Option<f32>,
    pub(crate) id: Processable,
}
impl<Processable> web_common_traits::database::ExtendableBuilder
for InsertableVolumetricProcessableBuilder<Processable>
where
    Processable: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes,
    >,
{
    type Attributes = InsertableVolumetricProcessableAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableVolumetricProcessableAttributes::Extension(
                    InsertableVolumetricProcessableExtensionAttributes::Processable(
                        attribute,
                    ),
                ))
            })?;
        if let Some(liters) = other.liters {
            self = self.liters(liters)?;
        }
        Ok(self)
    }
}
impl<Processable> web_common_traits::prelude::SetPrimaryKey
    for InsertableVolumetricProcessableBuilder<Processable>
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
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        Processable,
    >
{
    /// Sets the value of the `volumetric_processables.liters` column from table
    /// `volumetric_processables`.
    pub fn liters<P>(
        mut self,
        liters: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let liters = liters.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableVolumetricProcessableAttributes::Liters)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| e.rename_field(InsertableVolumetricProcessableAttributes::Liters))?;
        self.liters = Some(liters);
        Ok(self)
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            Trackable,
        >,
    >
{
    /// Sets the value of the `processables.kilograms` column from table
    /// `volumetric_processables`.
    pub fn kilograms<P>(
        mut self,
        kilograms: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.kilograms(kilograms).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.id` column from table
    /// `volumetric_processables`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.name` column from table
    /// `volumetric_processables`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `volumetric_processables`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `volumetric_processables`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    > {
        self.id = self.id.photograph(photograph_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `volumetric_processables`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    > {
        self.id = self.id.parent(parent_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `volumetric_processables`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    > {
        self.id = self.id.created_by(created_by).map_err(|e| e.into_field_name(From::from))?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `volumetric_processables`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `volumetric_processables`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    > {
        self.id = self.id.updated_by(updated_by).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `volumetric_processables`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl<Processable, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableVolumetricProcessableBuilder<Processable>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
        Error = web_common_traits::database::InsertError<
            InsertableVolumetricProcessableAttributes,
        >,
    >,
    Processable: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Attributes = InsertableVolumetricProcessableAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.liters.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
