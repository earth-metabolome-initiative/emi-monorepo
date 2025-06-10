#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialProductAttributes {
    Id(crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes),
    DeprecationDate,
    BrandId,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes>
    for InsertableCommercialProductAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl core::fmt::Display for InsertableCommercialProductAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCommercialProductAttributes::Id(id) => write!(f, "{}", id),
            InsertableCommercialProductAttributes::DeprecationDate => {
                write!(f, "deprecation_date")
            }
            InsertableCommercialProductAttributes::BrandId => write!(f, "brand_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_products::commercial_products
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialProduct {
    id: ::rosetta_uuid::Uuid,
    deprecation_date: Option<::rosetta_timestamp::TimestampUTC>,
    brand_id: i32,
}
impl InsertableCommercialProduct {
    pub fn brand<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::brands::Brand,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::brands::Brand: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::brands::Brand,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::brands::Brand::table(),
                self.brand_id,
            ),
            conn,
        )
    }
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableCommercialProductBuilder {
    pub(crate) id: crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    pub(crate) deprecation_date: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) brand_id: Option<i32>,
}
impl InsertableCommercialProductBuilder {
    pub fn deprecation_date<P>(
        mut self,
        deprecation_date: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        <P as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let deprecation_date = deprecation_date.try_into().map_err(
            |err: <P as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error| {
                Into::into(err).rename_field(InsertableCommercialProductAttributes::DeprecationDate)
            },
        )?;
        self.deprecation_date = deprecation_date;
        Ok(self)
    }
    pub fn brand_id<P>(
        mut self,
        brand_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let brand_id = brand_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableCommercialProductAttributes::BrandId)
        })?;
        self.brand_id = Some(brand_id);
        Ok(self)
    }
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
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
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCommercialProductAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
}
impl InsertableCommercialProductBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableCommercialProduct,
        web_common_traits::database::InsertError<InsertableCommercialProductAttributes>,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::trackables::Trackable,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let brand_id =
            self.brand_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCommercialProductAttributes::BrandId,
            ))?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        Ok(InsertableCommercialProduct { id, deprecation_date: self.deprecation_date, brand_id })
    }
}
