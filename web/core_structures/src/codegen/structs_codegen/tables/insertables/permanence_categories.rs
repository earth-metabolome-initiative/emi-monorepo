#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePermanenceCategoryAttributes {
    Name,
    Description,
    Icon,
    ColorId,
}
impl core::fmt::Display for InsertablePermanenceCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertablePermanenceCategoryAttributes::Name => write!(f, "name"),
            InsertablePermanenceCategoryAttributes::Description => {
                write!(f, "description")
            }
            InsertablePermanenceCategoryAttributes::Icon => write!(f, "icon"),
            InsertablePermanenceCategoryAttributes::ColorId => write!(f, "color_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePermanenceCategory {
    name: String,
    description: String,
    icon: String,
    color_id: i16,
}
impl InsertablePermanenceCategory {
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::colors::Color,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::colors::Color: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::colors::Color,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::colors::Color::table(),
                self.color_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertablePermanenceCategoryBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color_id: Option<i16>,
}
impl InsertablePermanenceCategoryBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePermanenceCategoryAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertablePermanenceCategoryAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePermanenceCategoryAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertablePermanenceCategoryAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePermanenceCategoryAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertablePermanenceCategoryAttributes::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn color_id<P>(
        mut self,
        color_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertablePermanenceCategoryAttributes>,
    >
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let color_id = color_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertablePermanenceCategoryAttributes::ColorId)
        })?;
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl TryFrom<InsertablePermanenceCategoryBuilder> for InsertablePermanenceCategory {
    type Error = common_traits::prelude::BuilderError<InsertablePermanenceCategoryAttributes>;
    fn try_from(
        builder: InsertablePermanenceCategoryBuilder,
    ) -> Result<InsertablePermanenceCategory, Self::Error> {
        Ok(Self {
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertablePermanenceCategoryAttributes::Name,
            ))?,
            description: builder.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePermanenceCategoryAttributes::Description,
                ),
            )?,
            icon: builder.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertablePermanenceCategoryAttributes::Icon,
            ))?,
            color_id: builder.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePermanenceCategoryAttributes::ColorId,
                ),
            )?,
        })
    }
}
