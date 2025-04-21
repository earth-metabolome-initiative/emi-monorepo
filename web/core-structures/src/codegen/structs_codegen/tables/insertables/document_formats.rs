#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDocumentFormatAttributes {
    Extension,
    MimeType,
    Description,
    IconId,
    Color,
}
impl core::fmt::Display for InsertableDocumentFormatAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableDocumentFormatAttributes::Extension => write!(f, "extension"),
            InsertableDocumentFormatAttributes::MimeType => write!(f, "mime_type"),
            InsertableDocumentFormatAttributes::Description => write!(f, "description"),
            InsertableDocumentFormatAttributes::IconId => write!(f, "icon_id"),
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
    icon_id: i16,
    color: String,
}
impl InsertableDocumentFormat {
    #[cfg(feature = "postgres")]
    pub async fn icon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::icons::Icon, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::icons::Icon::table()
            .filter(crate::codegen::diesel_codegen::tables::icons::icons::dsl::id.eq(&self.icon_id))
            .first::<crate::codegen::structs_codegen::tables::icons::Icon>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableDocumentFormatBuilder {
    extension: Option<String>,
    mime_type: Option<String>,
    description: Option<String>,
    icon_id: Option<i16>,
    color: Option<String>,
}
impl InsertableDocumentFormatBuilder {
    pub fn extension(
        mut self,
        extension: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.extension = Some(extension);
        Ok(self)
    }
    pub fn mime_type(
        mut self,
        mime_type: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.mime_type = Some(mime_type);
        Ok(self)
    }
    pub fn description(
        mut self,
        description: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.description = Some(description);
        Ok(self)
    }
    pub fn icon_id(
        mut self,
        icon_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.icon_id = Some(icon_id);
        Ok(self)
    }
    pub fn color(
        mut self,
        color: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
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
            icon_id: self.icon_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableDocumentFormatAttributes::IconId,
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
            .icon_id(insertable_variant.icon_id)?
            .color(insertable_variant.color)
    }
}
