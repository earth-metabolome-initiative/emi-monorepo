#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcedureModelToolCategoryAttributes {
    Quantity,
    ProcedureModelId,
    ToolCategoryId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableProcedureModelToolCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcedureModelToolCategoryAttributes::Quantity => {
                write!(f, "quantity")
            }
            InsertableProcedureModelToolCategoryAttributes::ProcedureModelId => {
                write!(f, "procedure_model_id")
            }
            InsertableProcedureModelToolCategoryAttributes::ToolCategoryId => {
                write!(f, "tool_category_id")
            }
            InsertableProcedureModelToolCategoryAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableProcedureModelToolCategoryAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableProcedureModelToolCategoryAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableProcedureModelToolCategoryAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_model_tool_categories::procedure_model_tool_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureModelToolCategory {
    quantity: i32,
    procedure_model_id: i32,
    tool_category_id: i16,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureModelToolCategory {
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
    pub async fn tool_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::tool_categories::ToolCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::tool_categories::ToolCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::tool_categories::tool_categories::dsl::id
                    .eq(&self.tool_category_id),
            )
            .first::<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>(conn)
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
pub struct InsertableProcedureModelToolCategoryBuilder {
    quantity: Option<i32>,
    procedure_model_id: Option<i32>,
    tool_category_id: Option<i16>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableProcedureModelToolCategoryBuilder {
    pub fn quantity(
        mut self,
        quantity: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_be_strictly_positive_i32(quantity).map_err(|e| {
            e.rename_field(InsertableProcedureModelToolCategoryAttributes::Quantity)
        })?;
        self.quantity = Some(quantity);
        Ok(self)
    }
    pub fn procedure_model_id(
        mut self,
        procedure_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
    pub fn tool_category_id(
        mut self,
        tool_category_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.tool_category_id = Some(tool_category_id);
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
impl common_traits::prelude::Builder for InsertableProcedureModelToolCategoryBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableProcedureModelToolCategoryAttributes>;
    type Object = InsertableProcedureModelToolCategory;
    type Attribute = InsertableProcedureModelToolCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            quantity: self.quantity.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelToolCategoryAttributes::Quantity,
                )
            })?,
            procedure_model_id: self.procedure_model_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelToolCategoryAttributes::ProcedureModelId,
                )
            })?,
            tool_category_id: self.tool_category_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelToolCategoryAttributes::ToolCategoryId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelToolCategoryAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelToolCategoryAttributes::CreatedAt,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelToolCategoryAttributes::UpdatedBy,
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelToolCategoryAttributes::UpdatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableProcedureModelToolCategory> for InsertableProcedureModelToolCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableProcedureModelToolCategory,
    ) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .quantity(insertable_variant.quantity)?
            .procedure_model_id(insertable_variant.procedure_model_id)?
            .tool_category_id(insertable_variant.tool_category_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?)
    }
}
