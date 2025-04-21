#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableToolModelAttributes {
    Id,
    ToolCategoryId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableToolModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableToolModelAttributes::Id => write!(f, "id"),
            InsertableToolModelAttributes::ToolCategoryId => {
                write!(f, "tool_category_id")
            }
            InsertableToolModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableToolModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertableToolModelAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableToolModelAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::tool_models::tool_models)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableToolModel {
    id: i32,
    tool_category_id: i16,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableToolModel {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_products::commercial_products::dsl::id
                    .eq(&self.id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
            >(conn)
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
pub struct InsertableToolModelBuilder {
    id: Option<i32>,
    tool_category_id: Option<i16>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableToolModelBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
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
impl common_traits::prelude::Builder for InsertableToolModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableToolModelAttributes>;
    type Object = InsertableToolModel;
    type Attribute = InsertableToolModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableToolModelAttributes::Id,
            ))?,
            tool_category_id: self.tool_category_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableToolModelAttributes::ToolCategoryId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableToolModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableToolModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableToolModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableToolModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableToolModel> for InsertableToolModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableToolModel) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .tool_category_id(insertable_variant.tool_category_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}
