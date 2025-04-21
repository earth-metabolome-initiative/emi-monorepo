#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhotographAttributes {
    Id,
    Path,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertablePhotographAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertablePhotographAttributes::Id => write!(f, "id"),
            InsertablePhotographAttributes::Path => write!(f, "path"),
            InsertablePhotographAttributes::CreatedBy => write!(f, "created_by"),
            InsertablePhotographAttributes::CreatedAt => write!(f, "created_at"),
            InsertablePhotographAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertablePhotographAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::photographs::photographs)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhotograph {
    id: rosetta_uuid::Uuid,
    path: String,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertablePhotograph {
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
pub struct InsertablePhotographBuilder {
    id: Option<rosetta_uuid::Uuid>,
    path: Option<String>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertablePhotographBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn path(
        mut self,
        path: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.path = Some(path);
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
impl common_traits::prelude::Builder for InsertablePhotographBuilder {
    type Error = web_common_traits::database::InsertError<InsertablePhotographAttributes>;
    type Object = InsertablePhotograph;
    type Attribute = InsertablePhotographAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertablePhotographAttributes::Id,
            ))?,
            path: self.path.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertablePhotographAttributes::Path,
            ))?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePhotographAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePhotographAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePhotographAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePhotographAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertablePhotograph> for InsertablePhotographBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertablePhotograph) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .path(insertable_variant.path)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}
