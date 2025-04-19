#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepModelAttributes {
    Name,
    Description,
    Snoozable,
    Copiable,
    PhotographId,
    StepModelCategoryId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepModelAttributes::Name => write!(f, "name"),
            InsertableStepModelAttributes::Description => write!(f, "description"),
            InsertableStepModelAttributes::Snoozable => write!(f, "snoozable"),
            InsertableStepModelAttributes::Copiable => write!(f, "copiable"),
            InsertableStepModelAttributes::PhotographId => write!(f, "photograph_id"),
            InsertableStepModelAttributes::StepModelCategoryId => {
                write!(f, "step_model_category_id")
            }
            InsertableStepModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableStepModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertableStepModelAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableStepModelAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::step_models::step_models)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepModel {
    name: String,
    description: String,
    snoozable: bool,
    copiable: Option<bool>,
    photograph_id: rosetta_uuid::Uuid,
    step_model_category_id: i16,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableStepModel {
    #[cfg(feature = "postgres")]
    pub async fn photograph(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photographs::Photograph,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::photographs::Photograph::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photographs::photographs::dsl::id
                    .eq(&self.photograph_id),
            )
            .first::<crate::codegen::structs_codegen::tables::photographs::Photograph>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn step_model_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_categories::step_model_categories::dsl::id
                    .eq(&self.step_model_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
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
pub struct InsertableStepModelBuilder {
    name: Option<String>,
    description: Option<String>,
    snoozable: Option<bool>,
    copiable: Option<bool>,
    photograph_id: Option<rosetta_uuid::Uuid>,
    step_model_category_id: Option<i16>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableStepModelBuilder {
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_not_be_empty(name.as_ref())
            .map_err(|e| e.rename_field(InsertableStepModelAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description(
        mut self,
        description: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_not_be_empty(description.as_ref())
            .map_err(|e| e.rename_field(InsertableStepModelAttributes::Description))?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn snoozable(
        mut self,
        snoozable: bool,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.snoozable = Some(snoozable);
        Ok(self)
    }
    pub fn copiable(
        mut self,
        copiable: Option<bool>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.copiable = copiable;
        Ok(self)
    }
    pub fn photograph_id(
        mut self,
        photograph_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.photograph_id = Some(photograph_id);
        Ok(self)
    }
    pub fn step_model_category_id(
        mut self,
        step_model_category_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_model_category_id = Some(step_model_category_id);
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
impl common_traits::prelude::Builder for InsertableStepModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStepModelAttributes>;
    type Object = InsertableStepModel;
    type Attribute = InsertableStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::Name,
                )
            })?,
            description: self.description.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::Description,
                )
            })?,
            snoozable: self.snoozable.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::Snoozable,
                )
            })?,
            copiable: self.copiable,
            photograph_id: self.photograph_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::PhotographId,
                )
            })?,
            step_model_category_id: self.step_model_category_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::StepModelCategoryId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::CreatedAt,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::UpdatedBy,
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::UpdatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableStepModel> for InsertableStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStepModel) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .snoozable(insertable_variant.snoozable)?
            .copiable(insertable_variant.copiable)?
            .photograph_id(insertable_variant.photograph_id)?
            .step_model_category_id(insertable_variant.step_model_category_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?)
    }
}
