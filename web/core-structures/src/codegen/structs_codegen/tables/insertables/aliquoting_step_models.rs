#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingStepModelAttributes {
    Id,
    StepModelInstrumentCategoryId,
    Liters,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableAliquotingStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableAliquotingStepModelAttributes::Id => write!(f, "id"),
            InsertableAliquotingStepModelAttributes::StepModelInstrumentCategoryId => {
                write!(f, "step_model_instrument_category_id")
            }
            InsertableAliquotingStepModelAttributes::Liters => write!(f, "liters"),
            InsertableAliquotingStepModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableAliquotingStepModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertableAliquotingStepModelAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableAliquotingStepModelAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::aliquoting_step_models::aliquoting_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingStepModel {
    id: i32,
    step_model_instrument_category_id: i32,
    liters: f32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableAliquotingStepModel {
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
    pub async fn step_model_instrument_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        diesel::result::Error,
    >{
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_instrument_categories::step_model_instrument_categories::dsl::id
                    .eq(&self.step_model_instrument_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableAliquotingStepModelBuilder {
    id: Option<i32>,
    step_model_instrument_category_id: Option<i32>,
    liters: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableAliquotingStepModelBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn step_model_instrument_category_id(
        mut self,
        step_model_instrument_category_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_model_instrument_category_id = Some(step_model_instrument_category_id);
        Ok(self)
    }
    pub fn liters(
        mut self,
        liters: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| e.rename_field(InsertableAliquotingStepModelAttributes::Liters))?;
        self.liters = Some(liters);
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at(
        mut self,
        created_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at(
        mut self,
        updated_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableAliquotingStepModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableAliquotingStepModelAttributes>;
    type Object = InsertableAliquotingStepModel;
    type Attribute = InsertableAliquotingStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableAliquotingStepModelAttributes::Id,
            ))?,
            step_model_instrument_category_id: self.step_model_instrument_category_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepModelAttributes::StepModelInstrumentCategoryId,
                ),
            )?,
            liters: self.liters.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableAliquotingStepModelAttributes::Liters,
            ))?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableAliquotingStepModel> for InsertableAliquotingStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableAliquotingStepModel) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .step_model_instrument_category_id(
                insertable_variant.step_model_instrument_category_id,
            )?
            .liters(insertable_variant.liters)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}
