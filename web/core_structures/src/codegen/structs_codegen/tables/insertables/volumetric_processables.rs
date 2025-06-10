#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricProcessableAttributes {
    Id(crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes),
    Liters,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes>
    for InsertableVolumetricProcessableAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl core::fmt::Display for InsertableVolumetricProcessableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableVolumetricProcessableAttributes::Id(id) => write!(f, "{}", id),
            InsertableVolumetricProcessableAttributes::Liters => write!(f, "liters"),
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
    id: ::rosetta_uuid::Uuid,
    liters: f32,
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::processables::Processable::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableVolumetricProcessableBuilder {
    pub(crate) id:
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder,
    pub(crate) liters: Option<f32>,
}
impl InsertableVolumetricProcessableBuilder {
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
        self.id = self.id.kilograms(kilograms).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
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
        self.id = self.id.id(id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
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
        self.id = self.id.name(name).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
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
        self.id = self.id.description(description).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.photograph_id(photograph_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn parent_id<P>(
        mut self,
        parent_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.parent_id(parent_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_by(created_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
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
        self.id = self.id.created_at(created_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_by(updated_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
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
        self.id = self.id.updated_at(updated_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
}
impl InsertableVolumetricProcessableBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableVolumetricProcessable,
        web_common_traits::database::InsertError<
            InsertableVolumetricProcessableAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcessableBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::processables::Processable,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableProcessableAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let liters = self.liters.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableVolumetricProcessableAttributes::Liters,
        ))?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        Ok(InsertableVolumetricProcessable { id, liters })
    }
}
