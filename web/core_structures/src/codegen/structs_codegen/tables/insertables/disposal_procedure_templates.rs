#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDisposalProcedureTemplateExtensionAttributes {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ),
}
impl core::fmt::Display for InsertableDisposalProcedureTemplateExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    > for InsertableDisposalProcedureTemplateExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDisposalProcedureTemplateAttributes {
    Extension(InsertableDisposalProcedureTemplateExtensionAttributes),
    ProcedureTemplate,
    DisposedAssetModel,
    ForeignProcedureTemplate,
    ProcedureTemplateDisposedAssetModel,
}
impl core::str::FromStr for InsertableDisposalProcedureTemplateAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DisposedAssetModel" => Ok(Self::DisposedAssetModel),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ProcedureTemplateDisposedAssetModel" => Ok(Self::ProcedureTemplateDisposedAssetModel),
            "disposed_asset_model" => Ok(Self::DisposedAssetModel),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "procedure_template_disposed_asset_model" => {
                Ok(Self::ProcedureTemplateDisposedAssetModel)
            }
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableDisposalProcedureTemplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::DisposedAssetModel => write!(f, "disposed_asset_model"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ProcedureTemplateDisposedAssetModel => {
                write!(f, "procedure_template_disposed_asset_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::disposal_procedure_templates::disposal_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableDisposalProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) disposed_asset_model: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) procedure_template_disposed_asset_model: i32,
}
impl InsertableDisposalProcedureTemplate {
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn disposed_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::table(),
                self.disposed_asset_model,
            ),
            conn,
        )
    }
    pub fn foreign_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.foreign_procedure_template,
            ),
            conn,
        )
    }
    pub fn procedure_template_disposed_asset_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_disposed_asset_model,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableDisposalProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) disposed_asset_model: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) procedure_template_disposed_asset_model: Option<i32>,
    pub(crate) procedure_template: ProcedureTemplate,
}
/// Trait defining setters for attributes of an instance of
/// `DisposalProcedureTemplate` or descendant tables.
pub trait DisposalProcedureTemplateBuildable:
    crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
{
    /// Sets the value of the
    /// `public.disposal_procedure_templates.disposed_asset_model` column.
    ///
    /// # Arguments
    /// * `disposed_asset_model`: The value to set for the
    ///   `public.disposal_procedure_templates.disposed_asset_model` column.
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
    fn disposed_asset_model(
        self,
        disposed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.disposal_procedure_templates.foreign_procedure_template` column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `public.disposal_procedure_templates.foreign_procedure_template`
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn foreign_procedure_template(
        self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.disposal_procedure_templates.
    /// procedure_template_disposed_asset_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_disposed_asset_model`: The value to set for the
    ///   `public.disposal_procedure_templates.
    ///   procedure_template_disposed_asset_model` column.
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
    fn procedure_template_disposed_asset_model(
        self,
        procedure_template_disposed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl DisposalProcedureTemplateBuildable for Option<i32> {
    fn disposed_asset_model(
        self,
        _disposed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn foreign_procedure_template(
        self,
        _foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn procedure_template_disposed_asset_model(
        self,
        _procedure_template_disposed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> DisposalProcedureTemplateBuildable
for InsertableDisposalProcedureTemplateBuilder<ProcedureTemplate> {
    ///Sets the value of the `public.disposal_procedure_templates.disposed_asset_model` column.
    fn disposed_asset_model(
        mut self,
        disposed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let disposed_asset_model = disposed_asset_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableDisposalProcedureTemplateAttributes::DisposedAssetModel,
                    )
            })?;
        self.disposed_asset_model = Some(disposed_asset_model);
        Ok(self)
    }
    ///Sets the value of the `public.disposal_procedure_templates.foreign_procedure_template` column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let foreign_procedure_template = foreign_procedure_template
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableDisposalProcedureTemplateAttributes::ForeignProcedureTemplate,
                    )
            })?;
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.disposal_procedure_templates.procedure_template_disposed_asset_model` column.
    fn procedure_template_disposed_asset_model(
        mut self,
        procedure_template_disposed_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let procedure_template_disposed_asset_model = procedure_template_disposed_asset_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableDisposalProcedureTemplateAttributes::ProcedureTemplateDisposedAssetModel,
                    )
            })?;
        self.procedure_template_disposed_asset_model = Some(
            procedure_template_disposed_asset_model,
        );
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
for InsertableDisposalProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateAttributes;
    #[inline]
    ///Sets the value of the `public.procedure_templates.most_concrete_table` column.
    fn most_concrete_table<MCT>(
        mut self,
        most_concrete_table: MCT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MCT: TryInto<String>,
        validation_errors::SingleFieldError: From<<MCT as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::most_concrete_table(
                self.procedure_template,
                most_concrete_table,
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
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::name(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::description(
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
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(
        mut self,
        deprecated: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<bool>,
        validation_errors::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::deprecated(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::icon(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::created_by(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::created_at(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::updated_by(
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::updated_at(
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
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableDisposalProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableDisposalProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            InsertableDisposalProcedureTemplateAttributes,
        >,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Attributes = InsertableDisposalProcedureTemplateAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.disposed_asset_model.is_some()
            && self.foreign_procedure_template.is_some()
            && self.procedure_template_disposed_asset_model.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
