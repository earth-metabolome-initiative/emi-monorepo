#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableVolumetricProcessableAttributes {
    Id,
    Liters,
}
impl core::fmt::Display for InsertableVolumetricProcessableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableVolumetricProcessableAttributes::Id => write!(f, "id"),
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
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::processables::Processable,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::processables::Processable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::processables::processables::dsl::id
                    .eq(&self.id),
            )
            .first::<crate::codegen::structs_codegen::tables::processables::Processable>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableVolumetricProcessableBuilder {
    id: Option<::rosetta_uuid::Uuid>,
    liters: Option<f32>,
}
impl InsertableVolumetricProcessableBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableVolumetricProcessableAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn liters<P>(
        mut self,
        liters: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
impl common_traits::prelude::Builder for InsertableVolumetricProcessableBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableVolumetricProcessableAttributes>;
    type Object = InsertableVolumetricProcessable;
    type Attribute = InsertableVolumetricProcessableAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableVolumetricProcessableAttributes::Id,
            ))?,
            liters: self.liters.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableVolumetricProcessableAttributes::Liters,
            ))?,
        })
    }
}
impl TryFrom<InsertableVolumetricProcessable> for InsertableVolumetricProcessableBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableVolumetricProcessable) -> Result<Self, Self::Error> {
        Self::default().id(insertable_variant.id)?.liters(insertable_variant.liters)
    }
}
