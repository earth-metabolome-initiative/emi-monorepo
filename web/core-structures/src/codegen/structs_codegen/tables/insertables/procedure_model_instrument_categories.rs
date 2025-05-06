#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcedureModelInstrumentCategoryAttributes {
    ProcedureModelId,
    InstrumentCategory,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableProcedureModelInstrumentCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcedureModelInstrumentCategoryAttributes::ProcedureModelId => {
                write!(f, "procedure_model_id")
            }
            InsertableProcedureModelInstrumentCategoryAttributes::InstrumentCategory => {
                write!(f, "instrument_category")
            }
            InsertableProcedureModelInstrumentCategoryAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableProcedureModelInstrumentCategoryAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableProcedureModelInstrumentCategoryAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableProcedureModelInstrumentCategoryAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_model_instrument_categories::procedure_model_instrument_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureModelInstrumentCategory {
    procedure_model_id: i32,
    instrument_category: instrument_categories::InstrumentCategory,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureModelInstrumentCategory {
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
pub struct InsertableProcedureModelInstrumentCategoryBuilder {
    procedure_model_id: Option<i32>,
    instrument_category: Option<instrument_categories::InstrumentCategory>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableProcedureModelInstrumentCategoryBuilder {
    fn default() -> Self {
        Self {
            procedure_model_id: None,
            instrument_category: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableProcedureModelInstrumentCategoryBuilder {
    pub fn procedure_model_id<P: Into<i32>>(
        mut self,
        procedure_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let procedure_model_id = procedure_model_id.into();
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
    pub fn instrument_category<P: Into<instrument_categories::InstrumentCategory>>(
        mut self,
        instrument_category: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let instrument_category = instrument_category.into();
        self.instrument_category = Some(instrument_category);
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
    pub fn created_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_at = created_at.into();
        self.created_at = Some(created_at);
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
    pub fn updated_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_at = updated_at.into();
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableProcedureModelInstrumentCategoryBuilder {
    type Error = web_common_traits::database::InsertError<
        InsertableProcedureModelInstrumentCategoryAttributes,
    >;
    type Object = InsertableProcedureModelInstrumentCategory;
    type Attribute = InsertableProcedureModelInstrumentCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            procedure_model_id: self.procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelInstrumentCategoryAttributes::ProcedureModelId,
                ),
            )?,
            instrument_category: self.instrument_category.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelInstrumentCategoryAttributes::InstrumentCategory,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelInstrumentCategoryAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelInstrumentCategoryAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelInstrumentCategoryAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelInstrumentCategoryAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableProcedureModelInstrumentCategory>
    for InsertableProcedureModelInstrumentCategoryBuilder
{
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableProcedureModelInstrumentCategory,
    ) -> Result<Self, Self::Error> {
        Self::default()
            .procedure_model_id(insertable_variant.procedure_model_id)?
            .instrument_category(insertable_variant.instrument_category)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
