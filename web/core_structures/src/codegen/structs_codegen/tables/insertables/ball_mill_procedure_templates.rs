#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BallMillProcedureTemplateExtensionAttribute {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for BallMillProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute>
    for BallMillProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BallMillProcedureTemplateAttribute {
    Extension(BallMillProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
    Kelvin,
    KelvinTolerancePercentage,
    Seconds,
    Hertz,
    BeadModel,
    ProcedureTemplateBeadModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    NumberOfBeads,
    MilledWithModel,
    ProcedureTemplateMilledWithModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
    MilledContainerModel,
    ProcedureTemplateMilledContainerModel(
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    ),
}
impl core::str::FromStr for BallMillProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "Seconds" => Ok(Self::Seconds),
            "Hertz" => Ok(Self::Hertz),
            "BeadModel" => Ok(Self::BeadModel),
            "ProcedureTemplateBeadModel" => {
                Ok(
                    Self::ProcedureTemplateBeadModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "NumberOfBeads" => Ok(Self::NumberOfBeads),
            "MilledWithModel" => Ok(Self::MilledWithModel),
            "ProcedureTemplateMilledWithModel" => {
                Ok(
                    Self::ProcedureTemplateMilledWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "MilledContainerModel" => Ok(Self::MilledContainerModel),
            "ProcedureTemplateMilledContainerModel" => {
                Ok(
                    Self::ProcedureTemplateMilledContainerModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "seconds" => Ok(Self::Seconds),
            "hertz" => Ok(Self::Hertz),
            "bead_model" => Ok(Self::BeadModel),
            "procedure_template_bead_model" => {
                Ok(
                    Self::ProcedureTemplateBeadModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "number_of_beads" => Ok(Self::NumberOfBeads),
            "milled_with_model" => Ok(Self::MilledWithModel),
            "procedure_template_milled_with_model" => {
                Ok(
                    Self::ProcedureTemplateMilledWithModel(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                    ),
                )
            }
            "milled_container_model" => Ok(Self::MilledContainerModel),
            "procedure_template_milled_container_model" => {
                Ok(
                    Self::ProcedureTemplateMilledContainerModel(
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
impl core::fmt::Display for BallMillProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::Kelvin => write!(f, "kelvin"),
            Self::KelvinTolerancePercentage => write!(f, "kelvin_tolerance_percentage"),
            Self::Seconds => write!(f, "seconds"),
            Self::Hertz => write!(f, "hertz"),
            Self::BeadModel => write!(f, "bead_model"),
            Self::ProcedureTemplateBeadModel(e) => write!(f, "{e}"),
            Self::NumberOfBeads => write!(f, "number_of_beads"),
            Self::MilledWithModel => write!(f, "milled_with_model"),
            Self::ProcedureTemplateMilledWithModel(e) => write!(f, "{e}"),
            Self::MilledContainerModel => write!(f, "milled_container_model"),
            Self::ProcedureTemplateMilledContainerModel(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBallMillProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) seconds: f32,
    pub(crate) hertz: f32,
    pub(crate) bead_model: i32,
    pub(crate) procedure_template_bead_model: i32,
    pub(crate) number_of_beads: i16,
    pub(crate) milled_with_model: i32,
    pub(crate) procedure_template_milled_with_model: i32,
    pub(crate) milled_container_model: i32,
    pub(crate) procedure_template_milled_container_model: i32,
}
impl InsertableBallMillProcedureTemplate {
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
    pub fn bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BeadModel, diesel::result::Error>
    where
        crate::BeadModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BeadModel::read(self.bead_model, conn)
    }
    pub fn procedure_template_bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_bead_model, conn)
    }
    pub fn milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BallMillMachineModel, diesel::result::Error>
    where
        crate::BallMillMachineModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BallMillMachineModel::read(self.milled_with_model, conn)
    }
    pub fn procedure_template_milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_milled_with_model, conn)
    }
    pub fn milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.milled_container_model, conn)
    }
    pub fn procedure_template_milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_milled_container_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedure_template_procedure_template_bead_mode_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_bead_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.bead_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedure_template_procedure_template_milled_wi_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_milled_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.milled_with_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedure_template_procedure_template_milled_co_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_milled_container_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.milled_container_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    pub fn ball_mill_procedure_templates_milled_with_model_milled_con_fkey<
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
            (self.milled_with_model, self.milled_container_model),
            conn,
        )
    }
    pub fn ball_mill_procedure_templates_milled_with_model_bead_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read((self.milled_with_model, self.bead_model), conn)
    }
    pub fn ball_mill_procedure_templates_bead_model_milled_container_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read((self.bead_model, self.milled_container_model), conn)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBallMillProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) hertz: Option<f32>,
    pub(crate) bead_model: Option<i32>,
    pub(crate) procedure_template_bead_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) number_of_beads: Option<i16>,
    pub(crate) milled_with_model: Option<i32>,
    pub(crate) procedure_template_milled_with_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) milled_container_model: Option<i32>,
    pub(crate) procedure_template_milled_container_model: web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    >,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl From<InsertableBallMillProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableBallMillProcedureTemplateBuilder>
{
    fn from(builder: InsertableBallMillProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<ProcedureTemplate> Default for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: Default,
{
    fn default() -> Self {
        Self {
            procedure_template: Default::default(),
            kelvin: Some(293.15f32),
            kelvin_tolerance_percentage: Some(1f32),
            seconds: Some(150f32),
            hertz: Some(25f32),
            bead_model: Default::default(),
            procedure_template_bead_model: Default::default(),
            number_of_beads: Some(3i16),
            milled_with_model: Default::default(),
            procedure_template_milled_with_model: Default::default(),
            milled_container_model: Default::default(),
            procedure_template_milled_container_model: Default::default(),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `BallMillProcedureTemplate` or descendant tables.
pub trait BallMillProcedureTemplateSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.ball_mill_procedure_templates.kelvin`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin`: The value to set for the
    ///   `public.ball_mill_procedure_templates.kelvin` column.
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
    fn kelvin<K>(
        self,
        kelvin: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.kelvin_tolerance_percentage`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin_tolerance_percentage`: The value to set for the
    ///   `public.ball_mill_procedure_templates.kelvin_tolerance_percentage`
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn kelvin_tolerance_percentage<KTP>(
        self,
        kelvin_tolerance_percentage: KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<KTP as TryInto<f32>>::Error>;
    /// Sets the value of the `public.ball_mill_procedure_templates.seconds`
    /// column.
    ///
    /// # Arguments
    /// * `seconds`: The value to set for the
    ///   `public.ball_mill_procedure_templates.seconds` column.
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
    fn seconds<S>(
        self,
        seconds: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<S as TryInto<f32>>::Error>;
    /// Sets the value of the `public.ball_mill_procedure_templates.hertz`
    /// column.
    ///
    /// # Arguments
    /// * `hertz`: The value to set for the
    ///   `public.ball_mill_procedure_templates.hertz` column.
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
    fn hertz<H>(
        self,
        hertz: H,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        H: TryInto<f32>,
        validation_errors::SingleFieldError: From<<H as TryInto<f32>>::Error>;
    /// Sets the value of the `public.ball_mill_procedure_templates.bead_model`
    /// column.
    ///
    /// # Arguments
    /// * `bead_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.bead_model` column.
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
    fn bead_model(
        self,
        bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.procedure_template_bead_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_bead_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.procedure_template_bead_model`
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
    fn procedure_template_bead_model<PTBM>(
        self,
        procedure_template_bead_model: PTBM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTBM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.number_of_beads` column.
    ///
    /// # Arguments
    /// * `number_of_beads`: The value to set for the
    ///   `public.ball_mill_procedure_templates.number_of_beads` column.
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
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn number_of_beads<NOB>(
        self,
        number_of_beads: NOB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOB: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOB as TryInto<i16>>::Error>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_with_model` column.
    ///
    /// # Arguments
    /// * `milled_with_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.milled_with_model` column.
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
    fn milled_with_model(
        self,
        milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_milled_with_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.
    ///   procedure_template_milled_with_model` column.
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
    fn procedure_template_milled_with_model<PTMWM>(
        self,
        procedure_template_milled_with_model: PTMWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTMWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_container_model` column.
    ///
    /// # Arguments
    /// * `milled_container_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.milled_container_model` column.
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
    fn milled_container_model(
        self,
        milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_milled_container_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.
    ///   procedure_template_milled_container_model` column.
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
    fn procedure_template_milled_container_model<PTMCM>(
        self,
        procedure_template_milled_container_model: PTMCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTMCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >;
}
impl<ProcedureTemplate> BallMillProcedureTemplateSettable
    for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute;
    /// Sets the value of the `public.ball_mill_procedure_templates.kelvin`
    /// column.
    fn kelvin<K>(
        mut self,
        kelvin: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>,
    {
        let kelvin = kelvin.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(BallMillProcedureTemplateAttribute::Kelvin)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.kelvin_tolerance_percentage`
    /// column.
    fn kelvin_tolerance_percentage<KTP>(
        mut self,
        kelvin_tolerance_percentage: KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<KTP as TryInto<f32>>::Error>,
    {
        let kelvin_tolerance_percentage =
            kelvin_tolerance_percentage.try_into().map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(BallMillProcedureTemplateAttribute::KelvinTolerancePercentage)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::KelvinTolerancePercentage,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(
                        kelvin_tolerance_percentage,
                        100f32,
                    )
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedure_templates.seconds`
    /// column.
    fn seconds<S>(
        mut self,
        seconds: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<S as TryInto<f32>>::Error>,
    {
        let seconds = seconds.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(BallMillProcedureTemplateAttribute::Seconds)
        })?;
        pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 900f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Seconds,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_strictly_greater_than_f32(seconds, 30f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Seconds,
                            )
                    })
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedure_templates.hertz`
    /// column.
    fn hertz<H>(
        mut self,
        hertz: H,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        H: TryInto<f32>,
        validation_errors::SingleFieldError: From<<H as TryInto<f32>>::Error>,
    {
        let hertz = hertz.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(BallMillProcedureTemplateAttribute::Hertz)
        })?;
        pgrx_validation::must_be_strictly_smaller_than_f32(hertz, 50f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Hertz,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_strictly_greater_than_f32(hertz, 15f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Hertz,
                            )
                    })
            })?;
        self.hertz = Some(hertz);
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedure_templates.bead_model`
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
    /// subgraph v6 ["`ball_mill_procedure_templates`"]
    ///    v2@{shape: rounded, label: "milled_with_model"}
    /// class v2 directly-involved-column
    ///    v1@{shape: rounded, label: "milled_container_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_template_bead_model"}
    /// class v3 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_template_asset_models`"]
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v4@{shape: rounded, label: "asset_model"}
    /// class v4 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v0 --->|"`associated same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 -.->|"`foreign defines`"| v0
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn bead_model(
        mut self,
        bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_template_bead_model) =
            self.procedure_template_bead_model
        {
            self.procedure_template_bead_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_bead_model,
                    bead_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateBeadModel(attribute)
                    })
                })?
                .into();
        }
        self.bead_model = Some(bead_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.procedure_template_bead_model`
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
    /// subgraph v4 ["`ball_mill_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_bead_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v0 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_bead_model<PTBM>(
        mut self,
        procedure_template_bead_model: PTBM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTBM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_bead_model = procedure_template_bead_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_bead_model
        {
            procedure_template_bead_model = if let (Some(bead_model), Some(asset_model)) =
                (self.bead_model, builder.asset_model)
            {
                if bead_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::BeadModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.bead_model = Some(asset_model);
                builder.into()
            } else if let Some(bead_model) = self.bead_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        bead_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateBeadModel(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_bead_model = procedure_template_bead_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.number_of_beads` column.
    fn number_of_beads<NOB>(
        mut self,
        number_of_beads: NOB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOB: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOB as TryInto<i16>>::Error>,
    {
        let number_of_beads = number_of_beads.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(BallMillProcedureTemplateAttribute::NumberOfBeads)
        })?;
        pgrx_validation::must_be_strictly_positive_i16(number_of_beads)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::NumberOfBeads,
                    )
            })?;
        self.number_of_beads = Some(number_of_beads);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_with_model` column.
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
    /// subgraph v6 ["`ball_mill_procedure_templates`"]
    ///    v2@{shape: rounded, label: "milled_with_model"}
    /// class v2 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_template_milled_with_model"}
    /// class v3 directly-involved-column
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "milled_container_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_template_asset_models`"]
    ///    v4@{shape: rounded, label: "asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 -.->|"`foreign defines`"| v2
    /// v0 --->|"`associated same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn milled_with_model(
        mut self,
        milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_milled_with_model,
        ) = self.procedure_template_milled_with_model
        {
            self.procedure_template_milled_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_milled_with_model,
                    milled_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateMilledWithModel(attribute)
                    })
                })?
                .into();
        }
        self.milled_with_model = Some(milled_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_with_model` column.
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
    /// subgraph v4 ["`ball_mill_procedure_templates`"]
    ///    v0@{shape: rounded, label: "milled_with_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_milled_with_model"}
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
    fn procedure_template_milled_with_model<PTMWM>(
        mut self,
        procedure_template_milled_with_model: PTMWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTMWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_milled_with_model = procedure_template_milled_with_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_milled_with_model
        {
            procedure_template_milled_with_model = if let (
                Some(milled_with_model),
                Some(asset_model),
            ) =
                (self.milled_with_model, builder.asset_model)
            {
                if milled_with_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::MilledWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.milled_with_model = Some(asset_model);
                builder.into()
            } else if let Some(milled_with_model) = self.milled_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        milled_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateMilledWithModel(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_milled_with_model = procedure_template_milled_with_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_container_model` column.
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
    /// subgraph v6 ["`ball_mill_procedure_templates`"]
    ///    v2@{shape: rounded, label: "milled_with_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "milled_container_model"}
    /// class v1 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_template_milled_container_model"}
    /// class v3 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_template_asset_models`"]
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v4@{shape: rounded, label: "asset_model"}
    /// class v4 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v0 --->|"`associated same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 -.->|"`foreign defines`"| v1
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn milled_container_model(
        mut self,
        milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_template_milled_container_model,
        ) = self.procedure_template_milled_container_model
        {
            self.procedure_template_milled_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                    procedure_template_milled_container_model,
                    milled_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateMilledContainerModel(
                            attribute,
                        )
                    })
                })?
                .into();
        }
        self.milled_container_model = Some(milled_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_container_model` column.
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
    /// subgraph v4 ["`ball_mill_procedure_templates`"]
    ///    v0@{shape: rounded, label: "milled_container_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_milled_container_model"}
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
    fn procedure_template_milled_container_model<PTMCM>(
        mut self,
        procedure_template_milled_container_model: PTMCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTMCM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        let mut procedure_template_milled_container_model =
            procedure_template_milled_container_model.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_template_milled_container_model
        {
            procedure_template_milled_container_model = if let (
                Some(milled_container_model),
                Some(asset_model),
            ) =
                (self.milled_container_model, builder.asset_model)
            {
                if milled_container_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::MilledContainerModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.milled_container_model = Some(asset_model);
                builder.into()
            } else if let Some(milled_container_model) = self.milled_container_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::asset_model(
                        builder,
                        milled_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTemplateMilledContainerModel(
                                attribute,
                            )
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_template_milled_container_model = procedure_template_milled_container_model;
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable
for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute;
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
    for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::BallMillProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            BallMillProcedureTemplateAttribute,
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
    type Attributes = BallMillProcedureTemplateAttribute;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some() && self.seconds.is_some()
            && self.hertz.is_some() && self.bead_model.is_some()
            && self.procedure_template_bead_model.is_complete()
            && self.number_of_beads.is_some() && self.milled_with_model.is_some()
            && self.procedure_template_milled_with_model.is_complete()
            && self.milled_container_model.is_some()
            && self.procedure_template_milled_container_model.is_complete()
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
        let insertable: crate::BallMillProcedureTemplate = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
