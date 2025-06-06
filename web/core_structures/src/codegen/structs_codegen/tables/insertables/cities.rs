#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCityAttributes {
    Name,
    Iso,
}
impl core::fmt::Display for InsertableCityAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCityAttributes::Name => write!(f, "name"),
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
    iso: ::iso_codes::CountryCode,
}
impl InsertableCity {
    pub fn iso<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::countries::Country,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::countries::Country: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::countries::Country as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::countries::Country as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::countries::Country as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::countries::Country as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::countries::Country as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::countries::Country as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::countries::Country,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::countries::Country::table(),
                self.iso,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableCityBuilder {
    name: Option<String>,
    iso: Option<::iso_codes::CountryCode>,
}
impl InsertableCityBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCityAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableCityAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn iso<P>(
        mut self,
        iso: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCityAttributes>>
    where
        P: TryInto<::iso_codes::CountryCode>,
        <P as TryInto<::iso_codes::CountryCode>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let iso =
            iso.try_into().map_err(|err: <P as TryInto<::iso_codes::CountryCode>>::Error| {
                Into::into(err).rename_field(InsertableCityAttributes::Iso)
            })?;
        self.iso = Some(iso);
        Ok(self)
    }
}
impl TryFrom<InsertableCityBuilder> for InsertableCity {
    type Error = common_traits::prelude::BuilderError<InsertableCityAttributes>;
    fn try_from(builder: InsertableCityBuilder) -> Result<InsertableCity, Self::Error> {
        Ok(Self {
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCityAttributes::Name,
            ))?,
            iso: builder.iso.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCityAttributes::Iso,
            ))?,
        })
    }
}
