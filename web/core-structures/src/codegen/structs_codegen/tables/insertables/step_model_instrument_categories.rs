#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepModelInstrumentCategoryAttributes {
    StepModelId,
    InstrumentCategoryId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableStepModelInstrumentCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepModelInstrumentCategoryAttributes::StepModelId => {
                write!(f, "step_model_id")
            }
            InsertableStepModelInstrumentCategoryAttributes::InstrumentCategoryId => {
                write!(f, "instrument_category_id")
            }
            InsertableStepModelInstrumentCategoryAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableStepModelInstrumentCategoryAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableStepModelInstrumentCategoryAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableStepModelInstrumentCategoryAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::step_model_instrument_categories::step_model_instrument_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepModelInstrumentCategory {
    step_model_id: i32,
    instrument_category_id: i16,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableStepModelInstrumentCategory {
    #[cfg(feature = "postgres")]
    pub async fn step_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_models::StepModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::step_models::StepModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_models::step_models::dsl::id
                    .eq(&self.step_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::step_models::StepModel>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn instrument_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instrument_categories::instrument_categories::dsl::id
                    .eq(&self.instrument_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
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
pub struct InsertableStepModelInstrumentCategoryBuilder {
    step_model_id: Option<i32>,
    instrument_category_id: Option<i16>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableStepModelInstrumentCategoryBuilder {
    pub fn step_model_id(
        mut self,
        step_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_model_id = Some(step_model_id);
        Ok(self)
    }
    pub fn instrument_category_id(
        mut self,
        instrument_category_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.instrument_category_id = Some(instrument_category_id);
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
impl common_traits::prelude::Builder for InsertableStepModelInstrumentCategoryBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableStepModelInstrumentCategoryAttributes>;
    type Object = InsertableStepModelInstrumentCategory;
    type Attribute = InsertableStepModelInstrumentCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            step_model_id: self.step_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelInstrumentCategoryAttributes::StepModelId,
                ),
            )?,
            instrument_category_id: self.instrument_category_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelInstrumentCategoryAttributes::InstrumentCategoryId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelInstrumentCategoryAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelInstrumentCategoryAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelInstrumentCategoryAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelInstrumentCategoryAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableStepModelInstrumentCategory>
    for InsertableStepModelInstrumentCategoryBuilder
{
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableStepModelInstrumentCategory,
    ) -> Result<Self, Self::Error> {
        Self::default()
            .step_model_id(insertable_variant.step_model_id)?
            .instrument_category_id(insertable_variant.instrument_category_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}
