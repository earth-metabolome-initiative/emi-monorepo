#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RankAttribute {
    Name,
    Description,
    Id,
}
impl core::str::FromStr for RankAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "Description" => Ok(Self::Description),
            "name" => Ok(Self::Name),
            "description" => Ok(Self::Description),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for RankAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::Description => write!(f, "description"),
            Self::Id => write!(f, "id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::ranks::ranks)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableRank {
    pub(crate) name: String,
    pub(crate) description: String,
}
impl InsertableRank {}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableRankBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}
impl From<InsertableRankBuilder>
    for web_common_traits::database::IdOrBuilder<i16, InsertableRankBuilder>
{
    fn from(builder: InsertableRankBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of `Rank` or descendant
/// tables.
pub trait RankSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.ranks.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.ranks.name` column.
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
    /// Sets the value of the `public.ranks.description` column.
    ///
    /// # Arguments
    /// * `description`: The value to set for the `public.ranks.description`
    ///   column.
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
    fn description<D>(
        self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>;
}
impl RankSettable for InsertableRankBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::RankAttribute;
    /// Sets the value of the `public.ranks.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(RankAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.ranks.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        let description = description.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(RankAttribute::Description)
        })?;
        self.description = Some(description);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableRankBuilder {
    type PrimaryKey = i16;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableRankBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::ranks::Rank,
            Error = web_common_traits::database::InsertError<RankAttribute>,
        >,
{
    type Attributes = RankAttribute;
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.description.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::ranks::Rank =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
