#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepModelAttributes {
    ProcedureModelId,
    Name,
    Description,
    Snoozable,
    Copiable,
    PhotographId,
    Icon,
    StepModelCategory,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepModelAttributes::ProcedureModelId => {
                write!(f, "procedure_model_id")
            }
            InsertableStepModelAttributes::Name => write!(f, "name"),
            InsertableStepModelAttributes::Description => write!(f, "description"),
            InsertableStepModelAttributes::Snoozable => write!(f, "snoozable"),
            InsertableStepModelAttributes::Copiable => write!(f, "copiable"),
            InsertableStepModelAttributes::PhotographId => write!(f, "photograph_id"),
            InsertableStepModelAttributes::Icon => write!(f, "icon"),
            InsertableStepModelAttributes::StepModelCategory => {
                write!(f, "step_model_category")
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
    procedure_model_id: i32,
    name: String,
    description: String,
    snoozable: bool,
    copiable: bool,
    photograph_id: ::rosetta_uuid::Uuid,
    icon: String,
    step_model_category: ::step_model_categories::StepModelCategory,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableStepModel {
    #[cfg(feature = "postgres")]
    pub async fn procedure_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_models::procedure_models::dsl::id
                    .eq(&self.procedure_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>(
                conn,
            )
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn photograph(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::documents::Document, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::documents::Document::table()
            .filter(
                crate::codegen::diesel_codegen::tables::documents::documents::dsl::id
                    .eq(&self.photograph_id),
            )
            .first::<crate::codegen::structs_codegen::tables::documents::Document>(conn)
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
pub struct InsertableStepModelBuilder {
    procedure_model_id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    snoozable: Option<bool>,
    copiable: Option<bool>,
    photograph_id: Option<::rosetta_uuid::Uuid>,
    icon: Option<String>,
    step_model_category: Option<::step_model_categories::StepModelCategory>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableStepModelBuilder {
    fn default() -> Self {
        Self {
            procedure_model_id: None,
            name: None,
            description: None,
            snoozable: Some(false),
            copiable: Some(false),
            photograph_id: None,
            icon: None,
            step_model_category: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableStepModelBuilder {
    pub fn procedure_model_id<P>(
        mut self,
        procedure_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let procedure_model_id =
            procedure_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(InsertableStepModelAttributes::ProcedureModelId)
            })?;
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableStepModelAttributes::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| e.rename_field(InsertableStepModelAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableStepModelAttributes::Description)
            })?;
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| e.rename_field(InsertableStepModelAttributes::Description))?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn snoozable<P>(
        mut self,
        snoozable: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let snoozable = snoozable.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableStepModelAttributes::Snoozable)
        })?;
        self.snoozable = Some(snoozable);
        Ok(self)
    }
    pub fn copiable<P>(
        mut self,
        copiable: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let copiable = copiable.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableStepModelAttributes::Copiable)
        })?;
        self.copiable = Some(copiable);
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let photograph_id = photograph_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableStepModelAttributes::PhotographId)
            },
        )?;
        self.photograph_id = Some(photograph_id);
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableStepModelAttributes::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| e.rename_field(InsertableStepModelAttributes::Icon))?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn step_model_category<P>(
        mut self,
        step_model_category: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::step_model_categories::StepModelCategory>,
        <P as TryInto<::step_model_categories::StepModelCategory>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let step_model_category = step_model_category.try_into().map_err(
            |err: <P as TryInto<::step_model_categories::StepModelCategory>>::Error| {
                Into::into(err).rename_field(InsertableStepModelAttributes::StepModelCategory)
            },
        )?;
        self.step_model_category = Some(step_model_category);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableStepModelAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableStepModelAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableStepModelAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableStepModelAttributes::UpdatedAt)
            },
        )?;
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
            procedure_model_id: self.procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::ProcedureModelId,
                ),
            )?,
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepModelAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::Description,
                ),
            )?,
            snoozable: self.snoozable.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::Snoozable,
                ),
            )?,
            copiable: self.copiable.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::Copiable,
                ),
            )?,
            photograph_id: self.photograph_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::PhotographId,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepModelAttributes::Icon,
            ))?,
            step_model_category: self.step_model_category.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::StepModelCategory,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableStepModel> for InsertableStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStepModel) -> Result<Self, Self::Error> {
        Self::default()
            .procedure_model_id(insertable_variant.procedure_model_id)?
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .snoozable(insertable_variant.snoozable)?
            .copiable(insertable_variant.copiable)?
            .photograph_id(insertable_variant.photograph_id)?
            .icon(insertable_variant.icon)?
            .step_model_category(insertable_variant.step_model_category)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
