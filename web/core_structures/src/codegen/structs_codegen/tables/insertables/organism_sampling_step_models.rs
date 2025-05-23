#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismSamplingStepModelAttributes {
    Id,
    OrganismId,
}
impl core::fmt::Display for InsertableOrganismSamplingStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganismSamplingStepModelAttributes::Id => write!(f, "id"),
            InsertableOrganismSamplingStepModelAttributes::OrganismId => {
                write!(f, "organism_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::organism_sampling_step_models::organism_sampling_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganismSamplingStepModel {
    id: i32,
    organism_id: ::rosetta_uuid::Uuid,
}
impl InsertableOrganismSamplingStepModel {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::sampling_step_models::sampling_step_models::dsl::id
                    .eq(&self.id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn organism(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::organisms::Organism, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::organisms::Organism::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organisms::organisms::dsl::id
                    .eq(&self.organism_id),
            )
            .first::<crate::codegen::structs_codegen::tables::organisms::Organism>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableOrganismSamplingStepModelBuilder {
    id: Option<i32>,
    organism_id: Option<::rosetta_uuid::Uuid>,
}
impl InsertableOrganismSamplingStepModelBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableOrganismSamplingStepModelAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn organism_id<P>(
        mut self,
        organism_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let organism_id = organism_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableOrganismSamplingStepModelAttributes::OrganismId)
            },
        )?;
        self.organism_id = Some(organism_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableOrganismSamplingStepModelBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableOrganismSamplingStepModelAttributes>;
    type Object = InsertableOrganismSamplingStepModel;
    type Attribute = InsertableOrganismSamplingStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganismSamplingStepModelAttributes::Id,
            ))?,
            organism_id: self.organism_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismSamplingStepModelAttributes::OrganismId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableOrganismSamplingStepModel> for InsertableOrganismSamplingStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableOrganismSamplingStepModel,
    ) -> Result<Self, Self::Error> {
        Self::default().id(insertable_variant.id)?.organism_id(insertable_variant.organism_id)
    }
}
