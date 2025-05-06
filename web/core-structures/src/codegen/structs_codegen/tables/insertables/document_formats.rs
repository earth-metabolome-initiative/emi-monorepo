#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDocumentFormatAttributes {
    Extension,
    MimeType,
    Description,
    Icon,
    Color,
}
impl core::fmt::Display for InsertableDocumentFormatAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableDocumentFormatAttributes::Extension => write!(f, "extension"),
            InsertableDocumentFormatAttributes::MimeType => write!(f, "mime_type"),
            InsertableDocumentFormatAttributes::Description => write!(f, "description"),
            InsertableDocumentFormatAttributes::Icon => write!(f, "icon"),
            InsertableDocumentFormatAttributes::Color => write!(f, "color"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::document_formats::document_formats
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableDocumentFormat {
    extension: String,
    mime_type: String,
    description: String,
    icon: String,
    color: String,
}
impl InsertableDocumentFormat {}
#[derive(Default)]
pub struct InsertableDocumentFormatBuilder {
    extension: Option<String>,
    mime_type: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    color: Option<String>,
}
impl InsertableDocumentFormatBuilder {
    pub fn extension<P: Into<String>>(
        mut self,
        extension: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let extension = extension.into();
        self.extension = Some(extension);
        Ok(self)
    }
    pub fn mime_type<P: Into<String>>(
        mut self,
        mime_type: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let mime_type = mime_type.into();
        self.mime_type = Some(mime_type);
        Ok(self)
    }
    pub fn description<P: Into<String>>(
        mut self,
        description: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let description = description.into();
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon<P: Into<String>>(
        mut self,
        icon: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let icon = icon.into();
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn color<P: Into<String>>(
        mut self,
        color: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let color = color.into();
        self.color = Some(color);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableDocumentFormatBuilder {
    type Error = web_common_traits::database::InsertError<InsertableDocumentFormatAttributes>;
    type Object = InsertableDocumentFormat;
    type Attribute = InsertableDocumentFormatAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            extension: self.extension.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDocumentFormatAttributes::Extension,
                ),
            )?,
            mime_type: self.mime_type.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDocumentFormatAttributes::MimeType,
                ),
            )?,
            description: self.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDocumentFormatAttributes::Description,
                ),
            )?,
            icon: self.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableDocumentFormatAttributes::Icon,
            ))?,
            color: self.color.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableDocumentFormatAttributes::Color,
            ))?,
        })
    }
}
impl TryFrom<InsertableDocumentFormat> for InsertableDocumentFormatBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableDocumentFormat) -> Result<Self, Self::Error> {
        Self::default()
            .extension(insertable_variant.extension)?
            .mime_type(insertable_variant.mime_type)?
            .description(insertable_variant.description)?
            .icon(insertable_variant.icon)?
            .color(insertable_variant.color)
    }
}
