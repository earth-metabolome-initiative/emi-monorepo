#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProjectWorkflowModelAttributes {
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
impl core::fmt::Display for InsertableProjectWorkflowModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProjectWorkflowModelAttributes::Name => write!(f, "name"),
            InsertableProjectWorkflowModelAttributes::Description => {
                write!(f, "description")
            }
            InsertableProjectWorkflowModelAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableProjectWorkflowModelAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
            InsertableProjectWorkflowModelAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableProjectWorkflowModelAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::project_workflow_models::project_workflow_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProjectWorkflowModel {
    name: String,
    description: String,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_at: rosetta_timestamp::TimestampUTC,
    created_by: i32,
    updated_by: i32,
}
impl InsertableProjectWorkflowModel {
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
pub struct InsertableProjectWorkflowModelBuilder {
    name: Option<String>,
    description: Option<String>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
}
impl Default for InsertableProjectWorkflowModelBuilder {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
            created_by: None,
            updated_by: None,
        }
    }
}
impl InsertableProjectWorkflowModelBuilder {
    pub fn name<P: Into<String>>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let name = name.into();
        pgrx_validation::must_not_be_empty(name.as_ref())
            .map_err(|e| e.rename_field(InsertableProjectWorkflowModelAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P: Into<String>>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let description = description.into();
        pgrx_validation::must_not_be_empty(description.as_ref())
            .map_err(|e| e.rename_field(InsertableProjectWorkflowModelAttributes::Description))?;
        self.description = Some(description);
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
    pub fn updated_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_at = updated_at.into();
        self.updated_at = Some(updated_at);
        Ok(self)
    }
    pub fn created_by<P: Into<i32>>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_by = created_by.into();
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn updated_by<P: Into<i32>>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_by = updated_by.into();
        self.updated_by = Some(updated_by);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableProjectWorkflowModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableProjectWorkflowModelAttributes>;
    type Object = InsertableProjectWorkflowModel;
    type Attribute = InsertableProjectWorkflowModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableProjectWorkflowModelAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectWorkflowModelAttributes::Description,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectWorkflowModelAttributes::CreatedAt,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectWorkflowModelAttributes::UpdatedAt,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectWorkflowModelAttributes::CreatedBy,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProjectWorkflowModelAttributes::UpdatedBy,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableProjectWorkflowModel> for InsertableProjectWorkflowModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableProjectWorkflowModel) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .created_at(insertable_variant.created_at)?
            .updated_at(insertable_variant.updated_at)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)
    }
}
