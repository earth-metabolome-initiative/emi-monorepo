#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismExtensionAttributes {
    Trackable(crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes),
}
impl core::fmt::Display for InsertableOrganismExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Trackable(e) => write!(f, "Trackable.{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismAttributes {
    Extension(InsertableOrganismExtensionAttributes),
    Id,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes>
    for InsertableOrganismAttributes
{
    fn from(
        trackables: crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    ) -> Self {
        Self::Extension(InsertableOrganismExtensionAttributes::Trackable(trackables))
    }
}
impl core::fmt::Display for InsertableOrganismAttributes {
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
    diesel(table_name = crate::codegen::diesel_codegen::tables::organisms::organisms)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganism {
    pub(crate) id: ::rosetta_uuid::Uuid,
}
impl InsertableOrganism {
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
pub struct InsertableOrganismBuilder<
    Trackable = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
> {
    pub(crate) id: Trackable,
}
impl<Trackable> web_common_traits::database::ExtendableBuilder
for InsertableOrganismBuilder<Trackable>
where
    Trackable: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    >,
{
    type Attributes = InsertableOrganismAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableOrganismAttributes::Extension(
                    InsertableOrganismExtensionAttributes::Trackable(attribute),
                ))
            })?;
        Ok(self)
    }
}
impl<Trackable> web_common_traits::prelude::SetPrimaryKey for InsertableOrganismBuilder<Trackable>
where
    Trackable: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.id` column from table `organisms`.
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.name` column from table `organisms`.
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `organisms`.
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `organisms`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>> {
        self.id = self.id.photograph(photograph_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `organisms`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>> {
        self.id = self.id.parent(parent_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `organisms`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>> {
        self.id = self.id.created_by(created_by).map_err(|e| e.into_field_name(From::from))?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `organisms`.
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>>
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
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `organisms`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>> {
        self.id = self.id.updated_by(updated_by).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `organisms`.
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableOrganismAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl<Trackable, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableOrganismBuilder<Trackable>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::organisms::Organism,
            Error = web_common_traits::database::InsertError<InsertableOrganismAttributes>,
        >,
    Trackable: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableOrganismAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::organisms::Organism =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
