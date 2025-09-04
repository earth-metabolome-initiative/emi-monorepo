#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCityAttribute {
    Id,
    Name,
    Iso,
}
impl core::str::FromStr for InsertableCityAttribute {
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
impl core::fmt::Display for InsertableCityAttribute {
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
/// Trait defining setters for attributes of an instance of `City` or descendant
/// tables.
pub trait CitySettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.cities.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.cities.name` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.cities.iso` column.
    ///
    /// # Arguments
    /// * `iso`: The value to set for the `public.cities.iso` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::iso_codes::CountryCode`.
    /// * If the provided value does not pass schema-defined validation.
    fn iso(
        self,
        iso: ::iso_codes::CountryCode,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl CitySettable for InsertableCityBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCityAttribute;
    /// Sets the value of the `public.cities.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableCityAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.cities.iso` column.
    fn iso(
        mut self,
        iso: ::iso_codes::CountryCode,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.iso = Some(iso);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableCityBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableCityBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::cities::City,
            Error = web_common_traits::database::InsertError<InsertableCityAttribute>,
        >,
{
    type Attributes = InsertableCityAttribute;
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
