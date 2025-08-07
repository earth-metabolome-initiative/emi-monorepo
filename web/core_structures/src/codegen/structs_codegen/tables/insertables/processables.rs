#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcessableExtensionAttributes {
    Trackable(crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes),
}
impl core::fmt::Display for InsertableProcessableExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Trackable(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcessableAttributes {
    Extension(InsertableProcessableExtensionAttributes),
    Id,
    Kilograms,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes>
    for InsertableProcessableAttributes
{
    fn from(
        trackables: crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    ) -> Self {
        Self::Extension(InsertableProcessableExtensionAttributes::Trackable(trackables))
    }
}
impl core::fmt::Display for InsertableProcessableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::Kilograms => write!(f, "kilograms"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::processables::processables
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcessable {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) kilograms: f32,
}
impl InsertableProcessable {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcessableBuilder<
    Trackable = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
> {
    pub(crate) kilograms: Option<f32>,
    pub(crate) id: Trackable,
}
impl<Trackable> web_common_traits::database::ExtendableBuilder
for InsertableProcessableBuilder<Trackable>
where
    Trackable: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    >,
{
    type Attributes = InsertableProcessableAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableProcessableAttributes::Extension(
                    InsertableProcessableExtensionAttributes::Trackable(attribute),
                ))
            })?;
        if let Some(kilograms) = other.kilograms {
            self = self.kilograms(kilograms)?;
        }
        Ok(self)
    }
}
impl<Trackable> web_common_traits::prelude::SetPrimaryKey
    for InsertableProcessableBuilder<Trackable>
where
    Trackable: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<Trackable>
{
    /// Sets the value of the `processables.kilograms` column from table
    /// `processables`.
    pub fn kilograms<P>(
        mut self,
        kilograms: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kilograms = kilograms.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableProcessableAttributes::Kilograms)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| e.rename_field(InsertableProcessableAttributes::Kilograms))?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `processables`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
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
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `processables`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    {
        self.id = self.id.created_by(created_by).map_err(|e| e.into_field_name(From::from))?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `processables`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.id` column from table `processables`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.name` column from table
    /// `processables`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `processables`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    {
        self.id = self.id.parent(parent_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `processables`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    {
        self.id = self.id.photograph(photograph_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `processables`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `processables`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcessableAttributes>>
    {
        self.id = self.id.updated_by(updated_by).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl<Trackable, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableProcessableBuilder<Trackable>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::processables::Processable,
            Error = web_common_traits::database::InsertError<InsertableProcessableAttributes>,
        >,
    Trackable: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableProcessableAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.kilograms.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::processables::Processable =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
