#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableOrganismAttributes {
    Id,
    Name,
    NameplateCategory,
}
impl core::fmt::Display for InsertableOrganismAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableOrganismAttributes::Id => write!(f, "id"),
            InsertableOrganismAttributes::Name => write!(f, "name"),
            InsertableOrganismAttributes::NameplateCategory => {
                write!(f, "nameplate_category")
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
    nameplate_category: nameplate_categories::NameplateCategory,
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
}
#[derive(Default)]
pub struct InsertableOrganismBuilder {
    id: Option<rosetta_uuid::Uuid>,
    name: Option<String>,
    nameplate_category: Option<nameplate_categories::NameplateCategory>,
}
impl InsertableOrganismBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableOrganismAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<Option<String>>>::Error| {
            Into::into(err).rename_field(InsertableOrganismAttributes::Name)
        })?;
        if let Some(name) = name.as_ref() {
            pgrx_validation::must_be_paragraph(name)
                .map_err(|e| e.rename_field(InsertableOrganismAttributes::Name))?;
        }
        self.name = name;
        Ok(self)
    }
    pub fn nameplate_category<P>(
        mut self,
        nameplate_category: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<nameplate_categories::NameplateCategory>,
        <P as TryInto<nameplate_categories::NameplateCategory>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let nameplate_category = nameplate_category.try_into().map_err(
            |err: <P as TryInto<nameplate_categories::NameplateCategory>>::Error| {
                Into::into(err).rename_field(InsertableOrganismAttributes::NameplateCategory)
            },
        )?;
        self.nameplate_category = Some(nameplate_category);
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
            nameplate_category: self.nameplate_category.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableOrganismAttributes::NameplateCategory,
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
            .nameplate_category(insertable_variant.nameplate_category)
    }
}
