#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepNameplateModelAttributes {
    Id,
    StepId,
    NameplateModelId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableStepNameplateModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepNameplateModelAttributes::Id => write!(f, "id"),
            InsertableStepNameplateModelAttributes::StepId => write!(f, "step_id"),
            InsertableStepNameplateModelAttributes::NameplateModelId => {
                write!(f, "nameplate_model_id")
            }
            InsertableStepNameplateModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableStepNameplateModelAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::step_nameplate_models::step_nameplate_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepNameplateModel {
    id: rosetta_uuid::Uuid,
    step_id: rosetta_uuid::Uuid,
    nameplate_model_id: i32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableStepNameplateModel {
    #[cfg(feature = "postgres")]
    pub async fn step(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::steps::Step, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::steps::Step::table()
            .filter(crate::codegen::diesel_codegen::tables::steps::steps::dsl::id.eq(&self.step_id))
            .first::<crate::codegen::structs_codegen::tables::steps::Step>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn nameplate_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::nameplate_models::nameplate_models::dsl::id
                    .eq(&self.nameplate_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>(
                conn,
            )
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
}
pub struct InsertableStepNameplateModelBuilder {
    id: Option<rosetta_uuid::Uuid>,
    step_id: Option<rosetta_uuid::Uuid>,
    nameplate_model_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableStepNameplateModelBuilder {
    fn default() -> Self {
        Self {
            id: None,
            step_id: None,
            nameplate_model_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableStepNameplateModelBuilder {
    pub fn id<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let id = id.into();
        self.id = Some(id);
        Ok(self)
    }
    pub fn step_id<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        step_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let step_id = step_id.into();
        self.step_id = Some(step_id);
        Ok(self)
    }
    pub fn nameplate_model_id<P: Into<i32>>(
        mut self,
        nameplate_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let nameplate_model_id = nameplate_model_id.into();
        self.nameplate_model_id = Some(nameplate_model_id);
        Ok(self)
    }
    pub fn created_by<P: Into<i32>>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_by = created_by.into();
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_at = created_at.into();
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableStepNameplateModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStepNameplateModelAttributes>;
    type Object = InsertableStepNameplateModel;
    type Attribute = InsertableStepNameplateModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepNameplateModelAttributes::Id,
            ))?,
            step_id: self.step_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepNameplateModelAttributes::StepId,
            ))?,
            nameplate_model_id: self.nameplate_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepNameplateModelAttributes::NameplateModelId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepNameplateModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepNameplateModelAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableStepNameplateModel> for InsertableStepNameplateModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStepNameplateModel) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .step_id(insertable_variant.step_id)?
            .nameplate_model_id(insertable_variant.nameplate_model_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)
    }
}
