#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CappingProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for CappingProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for CappingProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CappingProcedureTemplateAttribute {
    Extension(CappingProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    CappedContainerModel,
    ProcedureTemplateCappedContainerModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    CappedWithModel,
    ProcedureTemplateCappedWithModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for CappingProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CappedContainerModel" => Ok(Self::CappedContainerModel),
            "ProcedureTemplateCappedContainerModel" => {
                Ok(
                    Self::ProcedureTemplateCappedContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "CappedWithModel" => Ok(Self::CappedWithModel),
            "ProcedureTemplateCappedWithModel" => {
                Ok(
                    Self::ProcedureTemplateCappedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "capped_container_model" => Ok(Self::CappedContainerModel),
            "procedure_template_capped_container_model" => {
                Ok(
                    Self::ProcedureTemplateCappedContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "capped_with_model" => Ok(Self::CappedWithModel),
            "procedure_template_capped_with_model" => {
                Ok(
                    Self::ProcedureTemplateCappedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            _ => {
                Err(
                    web_common_traits::database::InsertError::UnknownAttribute(
                        s.to_owned(),
                    ),
                )
            }
        }
    }
}
impl core::fmt::Display for CappingProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::CappedContainerModel => write!(f, "capped_container_model"),
            Self::ProcedureTemplateCappedContainerModel(e) => write!(f, "{e}"),
            Self::CappedWithModel => write!(f, "capped_with_model"),
            Self::ProcedureTemplateCappedWithModel(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::capping_procedure_templates::capping_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCappingProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) capped_container_model: i32,
    pub(crate) procedure_template_capped_container_model: i32,
    pub(crate) capped_with_model: i32,
    pub(crate) procedure_template_capped_with_model: i32,
}
impl InsertableCappingProcedureTemplate {
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplate, diesel::result::Error>
    where
        crate::ProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplate::read(self.procedure_template, conn)
    }
    pub fn capped_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.capped_container_model, conn)
    }
    pub fn procedure_template_capped_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_capped_container_model,
            conn,
        )
    }
    pub fn capped_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CapModel, diesel::result::Error>
    where
        crate::CapModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CapModel::read(self.capped_with_model, conn)
    }
    pub fn procedure_template_capped_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_capped_with_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedure_templates_procedure_template_capped_con_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_capped_container_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.capped_container_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedure_templates_procedure_template_capped_wit_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_capped_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.capped_with_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    pub fn capping_procedure_templates_capped_container_model_capped_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read(
            (self.capped_container_model, self.capped_with_model),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCappingProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) capped_container_model: Option<i32>,
    pub(crate) procedure_template_capped_container_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) capped_with_model: Option<i32>,
    pub(crate) procedure_template_capped_with_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl From<InsertableCappingProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCappingProcedureTemplateBuilder>
{
    fn from(builder: InsertableCappingProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of
/// `CappingProcedureTemplate` or descendant tables.
pub trait CappingProcedureTemplateSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.capping_procedure_templates.capped_container_model` column.
    ///
    /// # Arguments
    /// * `capped_container_model`: The value to set for the
    ///   `public.capping_procedure_templates.capped_container_model` column.
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn capped_container_model(
        self,
        capped_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.capping_procedure_templates.
    /// procedure_template_capped_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_capped_container_model`: The value to set for the
    ///   `public.capping_procedure_templates.
    ///   procedure_template_capped_container_model` column.
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template_capped_container_model<PTCCM>(
        self,
        procedure_template_capped_container_model: PTCCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTCCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.capping_procedure_templates.capped_with_model` column.
    ///
    /// # Arguments
    /// * `capped_with_model`: The value to set for the
    ///   `public.capping_procedure_templates.capped_with_model` column.
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn capped_with_model(
        self,
        capped_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.capping_procedure_templates.
    /// procedure_template_capped_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_capped_with_model`: The value to set for the
    ///   `public.capping_procedure_templates.
    ///   procedure_template_capped_with_model` column.
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template_capped_with_model<PTCWM>(
        self,
        procedure_template_capped_with_model: PTCWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTCWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> CappingProcedureTemplateSettable
    for InsertableCappingProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::CappingProcedureTemplateAttribute;
    /// Sets the value of the
    /// `public.capping_procedure_templates.capped_container_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v5 ["`capping_procedure_templates`"]
    ///    v2@{shape: rounded, label: "procedure_template_capped_container_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "capped_container_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "capped_with_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v6 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "asset_model"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "id"}
    /// class v4 undirectly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v0
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v5 ---o|"`associated with`"| v6
    /// ```
    fn capped_container_model(
        mut self,
        capped_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_capped_container_model,
        ) = self.procedure_template_capped_container_model
        {
            self.procedure_template_capped_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_capped_container_model,
                    capped_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateCappedContainerModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.capped_container_model = Some(capped_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.capping_procedure_templates.
    /// procedure_template_capped_container_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`capping_procedure_templates`"]
    ///    v0@{shape: rounded, label: "capped_container_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_capped_container_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_capped_container_model<PTCCM>(
        mut self,
        procedure_template_capped_container_model: PTCCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTCCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_capped_container_model =
            procedure_template_capped_container_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_capped_container_model
        {
            procedure_template_capped_container_model = if let (
                Some(capped_container_model),
                Some(asset_model),
            ) =
                (self.capped_container_model, builder.asset_model)
            {
                if capped_container_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::CappedContainerModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.capped_container_model = Some(asset_model);
                builder.into()
            } else if let Some(capped_container_model) = self.capped_container_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        capped_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateCappedContainerModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_capped_container_model = procedure_template_capped_container_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.capping_procedure_templates.capped_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v5 ["`capping_procedure_templates`"]
    ///    v1@{shape: rounded, label: "capped_with_model"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_capped_with_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "capped_container_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v6 ["`procedure_template_asset_models`"]
    ///    v4@{shape: rounded, label: "id"}
    /// class v4 undirectly-involved-column
    ///    v3@{shape: rounded, label: "asset_model"}
    /// class v3 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v2 --->|"`associated same as`"| v4
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v1
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v5 ---o|"`associated with`"| v6
    /// ```
    fn capped_with_model(
        mut self,
        capped_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_capped_with_model,
        ) = self.procedure_template_capped_with_model
        {
            self.procedure_template_capped_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_capped_with_model,
                    capped_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateCappedWithModel(attribute)
                    })
                })?
                .into();
        }
        self.capped_with_model = Some(capped_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.capping_procedure_templates.
    /// procedure_template_capped_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`capping_procedure_templates`"]
    ///    v0@{shape: rounded, label: "capped_with_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_capped_with_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_capped_with_model<PTCWM>(
        mut self,
        procedure_template_capped_with_model: PTCWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTCWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_capped_with_model = procedure_template_capped_with_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_capped_with_model
        {
            procedure_template_capped_with_model = if let (
                Some(capped_with_model),
                Some(asset_model),
            ) =
                (self.capped_with_model, builder.asset_model)
            {
                if capped_with_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::CappedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.capped_with_model = Some(asset_model);
                builder.into()
            } else if let Some(capped_with_model) = self.capped_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        capped_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateCappedWithModel(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_capped_with_model = procedure_template_capped_with_model;
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable
for InsertableCappingProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CappingProcedureTemplateAttribute;
    #[inline]
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                self.procedure_template,
                name,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                self.procedure_template,
                description,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::icon(
                self.procedure_template,
                icon,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                self.procedure_template,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                self.procedure_template,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                self.procedure_template,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.updated_at` column.
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                self.procedure_template,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(
        mut self,
        deprecated: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<bool>,
        validation_errors::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                self.procedure_template,
                deprecated,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<ProcedureTemplate> web_common_traits::database::MostConcreteTable
    for InsertableCappingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableCappingProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_template = self.procedure_template.set_primary_key(primary_key);
        self
    }
}
impl<ProcedureTemplate, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableCappingProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::CappingProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            CappingProcedureTemplateAttribute,
        >,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = CappingProcedureTemplateAttribute;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.capped_container_model.is_some()
            && self.procedure_template_capped_container_model.is_complete()
            && self.capped_with_model.is_some()
            && self.procedure_template_capped_with_model.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::CappingProcedureTemplate = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
