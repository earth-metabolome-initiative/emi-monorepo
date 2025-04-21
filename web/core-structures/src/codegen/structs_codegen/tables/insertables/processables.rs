#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcessableAttributes {
    Id,
}
impl core::fmt::Display for InsertableProcessableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcessableAttributes::Id => write!(f, "id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::processables::processables
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcessable {
    id: rosetta_uuid::Uuid,
}
impl InsertableProcessable {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::trackables::Trackable, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::trackables::Trackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::id
                    .eq(&self.id),
            )
            .first::<crate::codegen::structs_codegen::tables::trackables::Trackable>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableProcessableBuilder {
    id: Option<rosetta_uuid::Uuid>,
}
impl InsertableProcessableBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableProcessableBuilder {
    type Error = web_common_traits::database::InsertError<InsertableProcessableAttributes>;
    type Object = InsertableProcessable;
    type Attribute = InsertableProcessableAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableProcessableAttributes::Id,
            ))?,
        })
    }
}
impl TryFrom<InsertableProcessable> for InsertableProcessableBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableProcessable) -> Result<Self, Self::Error> {
        Self::default().id(insertable_variant.id)?
    }
}
