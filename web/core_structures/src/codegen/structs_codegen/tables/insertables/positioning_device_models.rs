#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePositioningDeviceModelExtensionAttributes {
    InstrumentModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes,
    ),
}
impl core::fmt::Display for InsertablePositioningDeviceModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::InstrumentModel(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePositioningDeviceModelAttributes {
    Extension(InsertablePositioningDeviceModelExtensionAttributes),
    Id,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes>
    for InsertablePositioningDeviceModelAttributes
{
    fn from(
        instrument_models: crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes,
    ) -> Self {
        Self::Extension(InsertablePositioningDeviceModelExtensionAttributes::InstrumentModel(
            instrument_models,
        ))
    }
}
impl core::fmt::Display for InsertablePositioningDeviceModelAttributes {
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
        table_name = crate::codegen::diesel_codegen::tables::positioning_device_models::positioning_device_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePositioningDeviceModel {
    pub(crate) id: ::rosetta_uuid::Uuid,
}
impl InsertablePositioningDeviceModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::table(
                ),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePositioningDeviceModelBuilder<
    InstrumentModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
> {
    pub(crate) id: InstrumentModel,
}
impl<InstrumentModel> web_common_traits::database::ExtendableBuilder
for InsertablePositioningDeviceModelBuilder<InstrumentModel>
where
    InstrumentModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes,
    >,
{
    type Attributes = InsertablePositioningDeviceModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertablePositioningDeviceModelAttributes::Extension(
                    InsertablePositioningDeviceModelExtensionAttributes::InstrumentModel(
                        attribute,
                    ),
                ))
            })?;
        Ok(self)
    }
}
impl<InstrumentModel> web_common_traits::prelude::SetPrimaryKey
    for InsertablePositioningDeviceModelBuilder<InstrumentModel>
where
    InstrumentModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `positioning_device_models`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.created_at`, `trackables.updated_at`
    /// columns from table `positioning_device_models`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .created_at_and_updated_at(created_at, updated_at)
            .map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `positioning_device_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    > {
        self.id = self.id.created_by(created_by).map_err(|e| e.into_field_name(From::from))?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `positioning_device_models`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        Description: TryInto<Option<String>>,
        <Description as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.id` column from table
    /// `positioning_device_models`.
    pub fn id<Id>(
        mut self,
        id: Id,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.name` column from table
    /// `positioning_device_models`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        Name: TryInto<Option<String>>,
        <Name as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.name`, `trackables.description`
    /// columns from table `positioning_device_models`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .name_and_description(name, description)
            .map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `positioning_device_models`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    > {
        self.id = self.id.parent(parent_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.parent_id`, `trackables.id` columns
    /// from table `positioning_device_models`.
    pub fn parent_and_id<Id>(
        mut self,
        parent_id: ::rosetta_uuid::Uuid,
        id: Id,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id =
            self.id.parent_and_id(parent_id, id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `positioning_device_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    > {
        self.id = self.id.photograph(photograph_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `positioning_device_models`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    >
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
        >,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `positioning_device_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePositioningDeviceModelAttributes>,
    > {
        self.id = self.id.updated_by(updated_by).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl<InstrumentModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertablePositioningDeviceModelBuilder<InstrumentModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        Error = web_common_traits::database::InsertError<
            InsertablePositioningDeviceModelAttributes,
        >,
    >,
    InstrumentModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Attributes = InsertablePositioningDeviceModelAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
