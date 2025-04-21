#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismAttributes {
    Id,
    Name,
    NameplateCategoryId,
}
impl core::fmt::Display for InsertableOrganismAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganismAttributes::Id => write!(f, "id"),
            InsertableOrganismAttributes::Name => write!(f, "name"),
            InsertableOrganismAttributes::NameplateCategoryId => {
                write!(f, "nameplate_category_id")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::organisms::organisms)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableOrganism {
    id: rosetta_uuid::Uuid,
    name: Option<String>,
    nameplate_category_id: i16,
}
impl InsertableOrganism {
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
    #[cfg(feature = "postgres")]
    pub async fn nameplate_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories::dsl::id
                    .eq(&self.nameplate_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
            >(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableOrganismBuilder {
    id: Option<rosetta_uuid::Uuid>,
    name: Option<String>,
    nameplate_category_id: Option<i16>,
}
impl InsertableOrganismBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn name(
        mut self,
        name: Option<String>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(name) = name.as_ref() {
            pgrx_validation::must_not_be_empty(name)
                .map_err(|e| e.rename_field(InsertableOrganismAttributes::Name))?;
        }
        self.name = name;
        Ok(self)
    }
    pub fn nameplate_category_id(
        mut self,
        nameplate_category_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.nameplate_category_id = Some(nameplate_category_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableOrganismBuilder {
    type Error = web_common_traits::database::InsertError<InsertableOrganismAttributes>;
    type Object = InsertableOrganism;
    type Attribute = InsertableOrganismAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableOrganismAttributes::Id,
            ))?,
            name: self.name,
            nameplate_category_id: self.nameplate_category_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::NameplateCategoryId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableOrganism> for InsertableOrganismBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableOrganism) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .name(insertable_variant.name)?
            .nameplate_category_id(insertable_variant.nameplate_category_id)
    }
}
