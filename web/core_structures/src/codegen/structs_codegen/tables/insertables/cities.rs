#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCityAttributes {
    Id,
    Name,
    Iso,
}
impl core::str::FromStr for InsertableCityAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "Iso" => Ok(Self::Iso),
            "name" => Ok(Self::Name),
            "iso" => Ok(Self::Iso),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCityAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::Iso => write!(f, "iso"),
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
    pub(crate) name: String,
    pub(crate) iso: ::iso_codes::CountryCode,
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
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCityBuilder {
    pub(crate) name: Option<String>,
    pub(crate) iso: Option<::iso_codes::CountryCode>,
}
impl web_common_traits::database::ExtendableBuilder for InsertableCityBuilder {
    type Attributes = InsertableCityAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(iso) = other.iso {
            self = self.iso(iso)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableCityBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder {
    /// Sets the value of the `cities.iso` column from table `cities`.
    pub fn iso(
        mut self,
        iso: ::iso_codes::CountryCode,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCityAttributes>> {
        self.iso = Some(iso);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder {
    /// Sets the value of the `cities.name` column from table `cities`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCityAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableCityAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableCityBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::cities::City,
            Error = web_common_traits::database::InsertError<InsertableCityAttributes>,
        >,
{
    type Attributes = InsertableCityAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.iso.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::cities::City =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
