#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableObservationSubjectAttributes {
    Name,
    Description,
    Icon,
    ColorId,
}
impl core::fmt::Display for InsertableObservationSubjectAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableObservationSubjectAttributes::Name => write!(f, "name"),
            InsertableObservationSubjectAttributes::Description => {
                write!(f, "description")
            }
            InsertableObservationSubjectAttributes::Icon => write!(f, "icon"),
            InsertableObservationSubjectAttributes::ColorId => write!(f, "color_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableObservationSubject {
    name: String,
    description: String,
    icon: String,
    color_id: i16,
}
impl InsertableObservationSubject {
    #[cfg(feature = "postgres")]
    pub async fn color(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::colors::Color::table()
            .filter(
                crate::codegen::diesel_codegen::tables::colors::colors::dsl::id.eq(&self.color_id),
            )
            .first::<crate::codegen::structs_codegen::tables::colors::Color>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableObservationSubjectBuilder {
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    color_id: Option<i16>,
}
impl InsertableObservationSubjectBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableObservationSubjectAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableObservationSubjectAttributes::Description)
            })?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableObservationSubjectAttributes::Icon)
        })?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn color_id<P>(
        mut self,
        color_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let color_id = color_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableObservationSubjectAttributes::ColorId)
        })?;
        self.color_id = Some(color_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableObservationSubjectBuilder {
    type Error = web_common_traits::database::InsertError<InsertableObservationSubjectAttributes>;
    type Object = InsertableObservationSubject;
    type Attribute = InsertableObservationSubjectAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableObservationSubjectAttributes::Name,
            ))?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationSubjectAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableObservationSubjectAttributes::Icon,
            ))?,
            color_id: self.color_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableObservationSubjectAttributes::ColorId,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableObservationSubject> for InsertableObservationSubjectBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableObservationSubject) -> Result<Self, Self::Error> {
        Self::default()
            .name(insertable_variant.name)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)?
            .color_id(insertable_variant.color_id)
    }
}
