#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FractioningProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for FractioningProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for FractioningProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FractioningProcedureTemplateAttribute {
    Extension(FractioningProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    Kilograms,
    TolerancePercentage,
    WeighedWithModel,
    ProcedureTemplateWeighedWithModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    FragmentContainerModel,
    ProcedureTemplateFragmentContainerModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    FragmentPlacedIntoModel,
    ProcedureTemplateFragmentPlacedIntoModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for FractioningProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kilograms" => Ok(Self::Kilograms),
            "TolerancePercentage" => Ok(Self::TolerancePercentage),
            "WeighedWithModel" => Ok(Self::WeighedWithModel),
            "ProcedureTemplateWeighedWithModel" => {
                Ok(
                    Self::ProcedureTemplateWeighedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "FragmentContainerModel" => Ok(Self::FragmentContainerModel),
            "ProcedureTemplateFragmentContainerModel" => {
                Ok(
                    Self::ProcedureTemplateFragmentContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "FragmentPlacedIntoModel" => Ok(Self::FragmentPlacedIntoModel),
            "ProcedureTemplateFragmentPlacedIntoModel" => {
                Ok(
                    Self::ProcedureTemplateFragmentPlacedIntoModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "kilograms" => Ok(Self::Kilograms),
            "tolerance_percentage" => Ok(Self::TolerancePercentage),
            "weighed_with_model" => Ok(Self::WeighedWithModel),
            "procedure_template_weighed_with_model" => {
                Ok(
                    Self::ProcedureTemplateWeighedWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "fragment_container_model" => Ok(Self::FragmentContainerModel),
            "procedure_template_fragment_container_model" => {
                Ok(
                    Self::ProcedureTemplateFragmentContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "fragment_placed_into_model" => Ok(Self::FragmentPlacedIntoModel),
            "procedure_template_fragment_placed_into_model" => {
                Ok(
                    Self::ProcedureTemplateFragmentPlacedIntoModel(
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
impl core::fmt::Display for FractioningProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::Kilograms => write!(f, "kilograms"),
            Self::TolerancePercentage => write!(f, "tolerance_percentage"),
            Self::WeighedWithModel => write!(f, "weighed_with_model"),
            Self::ProcedureTemplateWeighedWithModel(e) => write!(f, "{e}"),
            Self::FragmentContainerModel => write!(f, "fragment_container_model"),
            Self::ProcedureTemplateFragmentContainerModel(e) => write!(f, "{e}"),
            Self::FragmentPlacedIntoModel => write!(f, "fragment_placed_into_model"),
            Self::ProcedureTemplateFragmentPlacedIntoModel(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) kilograms: f32,
    pub(crate) tolerance_percentage: f32,
    pub(crate) weighed_with_model: i32,
    pub(crate) procedure_template_weighed_with_model: i32,
    pub(crate) fragment_container_model: i32,
    pub(crate) procedure_template_fragment_container_model: i32,
    pub(crate) fragment_placed_into_model: i32,
    pub(crate) procedure_template_fragment_placed_into_model: i32,
}
impl InsertableFractioningProcedureTemplate {
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
    pub fn weighed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::WeighingDeviceModel, diesel::result::Error>
    where
        crate::WeighingDeviceModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::WeighingDeviceModel::read(self.weighed_with_model, conn)
    }
    pub fn procedure_template_weighed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_weighed_with_model, conn)
    }
    pub fn fragment_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.fragment_container_model, conn)
    }
    pub fn procedure_template_fragment_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_fragment_container_model,
            conn,
        )
    }
    pub fn fragment_placed_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.fragment_placed_into_model, conn)
    }
    pub fn procedure_template_fragment_placed_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_fragment_placed_into_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedure_templa_procedure_template_weighed_w_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_weighed_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.weighed_with_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedure_templa_procedure_template_fragment_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_fragment_container_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.fragment_container_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedure_templa_procedure_template_fragment_fkey3(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_fragment_placed_into_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.fragment_placed_into_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kilograms: Option<f32>,
    pub(crate) tolerance_percentage: Option<f32>,
    pub(crate) weighed_with_model: Option<i32>,
    pub(crate) procedure_template_weighed_with_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) fragment_container_model: Option<i32>,
    pub(crate) procedure_template_fragment_container_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) fragment_placed_into_model: Option<i32>,
    pub(crate) procedure_template_fragment_placed_into_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl From<InsertableFractioningProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableFractioningProcedureTemplateBuilder>
{
    fn from(builder: InsertableFractioningProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of
/// `FractioningProcedureTemplate` or descendant tables.
pub trait FractioningProcedureTemplateSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.fractioning_procedure_templates.kilograms`
    /// column.
    ///
    /// # Arguments
    /// * `kilograms`: The value to set for the
    ///   `public.fractioning_procedure_templates.kilograms` column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn kilograms<K>(
        self,
        kilograms: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.tolerance_percentage` column.
    ///
    /// # Arguments
    /// * `tolerance_percentage`: The value to set for the
    ///   `public.fractioning_procedure_templates.tolerance_percentage` column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn tolerance_percentage<TP>(
        self,
        tolerance_percentage: TP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        TP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<TP as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.weighed_with_model` column.
    ///
    /// # Arguments
    /// * `weighed_with_model`: The value to set for the
    ///   `public.fractioning_procedure_templates.weighed_with_model` column.
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
    fn weighed_with_model(
        self,
        weighed_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.
    /// procedure_template_weighed_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_weighed_with_model`: The value to set for the
    ///   `public.fractioning_procedure_templates.
    ///   procedure_template_weighed_with_model` column.
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
    fn procedure_template_weighed_with_model<PTWWM>(
        self,
        procedure_template_weighed_with_model: PTWWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTWWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.fragment_container_model`
    /// column.
    ///
    /// # Arguments
    /// * `fragment_container_model`: The value to set for the
    ///   `public.fractioning_procedure_templates.fragment_container_model`
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
    fn fragment_container_model(
        self,
        fragment_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.
    /// procedure_template_fragment_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_fragment_container_model`: The value to set for
    ///   the `public.fractioning_procedure_templates.
    ///   procedure_template_fragment_container_model` column.
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
    fn procedure_template_fragment_container_model<PTFCM>(
        self,
        procedure_template_fragment_container_model: PTFCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTFCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.fragment_placed_into_model`
    /// column.
    ///
    /// # Arguments
    /// * `fragment_placed_into_model`: The value to set for the
    ///   `public.fractioning_procedure_templates.fragment_placed_into_model`
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
    fn fragment_placed_into_model(
        self,
        fragment_placed_into_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.
    /// procedure_template_fragment_placed_into_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_fragment_placed_into_model`: The value to set for
    ///   the `public.fractioning_procedure_templates.
    ///   procedure_template_fragment_placed_into_model` column.
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
    fn procedure_template_fragment_placed_into_model<PTFPIM>(
        self,
        procedure_template_fragment_placed_into_model: PTFPIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTFPIM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> FractioningProcedureTemplateSettable
    for InsertableFractioningProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute;
    /// Sets the value of the `public.fractioning_procedure_templates.kilograms`
    /// column.
    fn kilograms<K>(
        mut self,
        kilograms: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>,
    {
        let kilograms = kilograms.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(FractioningProcedureTemplateAttribute::Kilograms)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::Kilograms,
                    )
            })?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.tolerance_percentage` column.
    fn tolerance_percentage<TP>(
        mut self,
        tolerance_percentage: TP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        TP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<TP as TryInto<f32>>::Error>,
    {
        let tolerance_percentage = tolerance_percentage.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(FractioningProcedureTemplateAttribute::TolerancePercentage)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::TolerancePercentage,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(tolerance_percentage, 100f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::TolerancePercentage,
                            )
                    })
            })?;
        self.tolerance_percentage = Some(tolerance_percentage);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.weighed_with_model` column.
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
    /// subgraph v4 ["`fractioning_procedure_templates`"]
    ///    v1@{shape: rounded, label: "weighed_with_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "procedure_template_weighed_with_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v2
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn weighed_with_model(
        mut self,
        weighed_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_weighed_with_model,
        ) = self.procedure_template_weighed_with_model
        {
            self.procedure_template_weighed_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_weighed_with_model,
                    weighed_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateWeighedWithModel(attribute)
                    })
                })?
                .into();
        }
        self.weighed_with_model = Some(weighed_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.
    /// procedure_template_weighed_with_model` column.
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
    /// subgraph v4 ["`fractioning_procedure_templates`"]
    ///    v1@{shape: rounded, label: "weighed_with_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "procedure_template_weighed_with_model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v2
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_weighed_with_model<PTWWM>(
        mut self,
        procedure_template_weighed_with_model: PTWWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTWWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_weighed_with_model =
            procedure_template_weighed_with_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_weighed_with_model
        {
            procedure_template_weighed_with_model = if let (
                Some(weighed_with_model),
                Some(asset_model),
            ) =
                (self.weighed_with_model, builder.asset_model)
            {
                if weighed_with_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::WeighedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.weighed_with_model = Some(asset_model);
                builder.into()
            } else if let Some(weighed_with_model) = self.weighed_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        weighed_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateWeighedWithModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_weighed_with_model = procedure_template_weighed_with_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.fragment_container_model`
    /// column.
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
    /// subgraph v4 ["`fractioning_procedure_templates`"]
    ///    v0@{shape: rounded, label: "fragment_container_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_fragment_container_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn fragment_container_model(
        mut self,
        fragment_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_fragment_container_model,
        ) = self.procedure_template_fragment_container_model
        {
            self.procedure_template_fragment_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_fragment_container_model,
                    fragment_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateFragmentContainerModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.fragment_container_model = Some(fragment_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.
    /// procedure_template_fragment_container_model` column.
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
    /// subgraph v4 ["`fractioning_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_fragment_container_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "fragment_container_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v0 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_fragment_container_model<PTFCM>(
        mut self,
        procedure_template_fragment_container_model: PTFCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTFCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_fragment_container_model =
            procedure_template_fragment_container_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_fragment_container_model
        {
            procedure_template_fragment_container_model = if let (
                Some(fragment_container_model),
                Some(asset_model),
            ) =
                (self.fragment_container_model, builder.asset_model)
            {
                if fragment_container_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::FragmentContainerModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.fragment_container_model = Some(asset_model);
                builder.into()
            } else if let Some(fragment_container_model) = self.fragment_container_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        fragment_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateFragmentContainerModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_fragment_container_model =
            procedure_template_fragment_container_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.fragment_placed_into_model`
    /// column.
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
    /// subgraph v4 ["`fractioning_procedure_templates`"]
    ///    v0@{shape: rounded, label: "fragment_placed_into_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_fragment_placed_into_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn fragment_placed_into_model(
        mut self,
        fragment_placed_into_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_fragment_placed_into_model,
        ) = self.procedure_template_fragment_placed_into_model
        {
            self.procedure_template_fragment_placed_into_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_fragment_placed_into_model,
                    fragment_placed_into_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateFragmentPlacedIntoModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.fragment_placed_into_model = Some(fragment_placed_into_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedure_templates.
    /// procedure_template_fragment_placed_into_model` column.
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
    /// subgraph v4 ["`fractioning_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_fragment_placed_into_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "fragment_placed_into_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v0 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_fragment_placed_into_model<PTFPIM>(
        mut self,
        procedure_template_fragment_placed_into_model: PTFPIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTFPIM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_fragment_placed_into_model =
            procedure_template_fragment_placed_into_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_fragment_placed_into_model
        {
            procedure_template_fragment_placed_into_model = if let (
                Some(fragment_placed_into_model),
                Some(asset_model),
            ) =
                (self.fragment_placed_into_model, builder.asset_model)
            {
                if fragment_placed_into_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::FragmentPlacedIntoModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.fragment_placed_into_model = Some(asset_model);
                builder.into()
            } else if let Some(fragment_placed_into_model) = self.fragment_placed_into_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        fragment_placed_into_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateFragmentPlacedIntoModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_fragment_placed_into_model =
            procedure_template_fragment_placed_into_model;
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable
for InsertableFractioningProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute;
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
    for InsertableFractioningProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableFractioningProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableFractioningProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::FractioningProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            FractioningProcedureTemplateAttribute,
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
    type Attributes = FractioningProcedureTemplateAttribute;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kilograms.is_some()
            && self.tolerance_percentage.is_some() && self.weighed_with_model.is_some()
            && self.procedure_template_weighed_with_model.is_complete()
            && self.fragment_container_model.is_some()
            && self.procedure_template_fragment_container_model.is_complete()
            && self.fragment_placed_into_model.is_some()
            && self.procedure_template_fragment_placed_into_model.is_complete()
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
        let insertable: crate::FractioningProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
