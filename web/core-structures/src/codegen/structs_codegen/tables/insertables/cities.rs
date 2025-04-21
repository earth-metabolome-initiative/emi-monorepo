#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCityAttributes {
    Name,
    Code,
    Iso,
}
impl core::fmt::Display for InsertableCityAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCityAttributes::Name => write!(f, "name"),
            InsertableCityAttributes::Code => write!(f, "code"),
            InsertableCityAttributes::Iso => write!(f, "iso"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::cities::cities)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCity {
    name: String,
    code: String,
    iso: String,
}
impl InsertableCity {
    #[cfg(feature = "postgres")]
    pub async fn iso(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::countries::Country, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::countries::Country::table()
            .filter(
                crate::codegen::diesel_codegen::tables::countries::countries::dsl::iso
                    .eq(&self.iso),
            )
            .first::<crate::codegen::structs_codegen::tables::countries::Country>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableCityBuilder {
    name: Option<String>,
    code: Option<String>,
    iso: Option<String>,
}
impl InsertableCityBuilder {
    pub fn name(
        mut self,
        name: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.name = Some(name);
        Ok(self)
    }
    pub fn code(
        mut self,
        code: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.code = Some(code);
        Ok(self)
    }
    pub fn iso(
        mut self,
        iso: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.iso = Some(iso);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableCityBuilder {
    type Error = web_common_traits::database::InsertError<InsertableCityAttributes>;
    type Object = InsertableCity;
    type Attribute = InsertableCityAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCityAttributes::Name,
            ))?,
            code: self.code.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCityAttributes::Code,
            ))?,
            iso: self.iso.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCityAttributes::Iso,
            ))?,
        })
    }
}
impl TryFrom<InsertableCity> for InsertableCityBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCity) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .code(insertable_variant.code)?
            .iso(insertable_variant.iso)
    }
}
