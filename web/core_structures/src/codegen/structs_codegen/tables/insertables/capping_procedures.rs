#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CappingProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for CappingProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for CappingProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CappingProcedureAttribute {
    Extension(CappingProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    CappedContainer,
    CappedContainerModel,
    ProcedureTemplateCappedContainerModel,
    ProcedureCappedContainer(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    CappedWithModel,
    ProcedureTemplateCappedWithModel,
    ProcedureCappedWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for CappingProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "CappedContainer" => Ok(Self::CappedContainer),
            "CappedContainerModel" => Ok(Self::CappedContainerModel),
            "ProcedureTemplateCappedContainerModel" => {
                Ok(Self::ProcedureTemplateCappedContainerModel)
            }
            "ProcedureCappedContainer" => Ok(Self::ProcedureCappedContainer(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "CappedWithModel" => Ok(Self::CappedWithModel),
            "ProcedureTemplateCappedWithModel" => Ok(Self::ProcedureTemplateCappedWithModel),
            "ProcedureCappedWith" => Ok(Self::ProcedureCappedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "capped_container" => Ok(Self::CappedContainer),
            "capped_container_model" => Ok(Self::CappedContainerModel),
            "procedure_template_capped_container_model" => {
                Ok(Self::ProcedureTemplateCappedContainerModel)
            }
            "procedure_capped_container" => Ok(Self::ProcedureCappedContainer(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "capped_with_model" => Ok(Self::CappedWithModel),
            "procedure_template_capped_with_model" => Ok(Self::ProcedureTemplateCappedWithModel),
            "procedure_capped_with" => Ok(Self::ProcedureCappedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for CappingProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::CappedContainer => write!(f, "capped_container"),
            Self::CappedContainerModel => write!(f, "capped_container_model"),
            Self::ProcedureTemplateCappedContainerModel => {
                write!(f, "procedure_template_capped_container_model")
            }
            Self::ProcedureCappedContainer(e) => write!(f, "{e}"),
            Self::CappedWithModel => write!(f, "capped_with_model"),
            Self::ProcedureTemplateCappedWithModel => {
                write!(f, "procedure_template_capped_with_model")
            }
            Self::ProcedureCappedWith(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::capping_procedures::capping_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCappingProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) capped_container: ::rosetta_uuid::Uuid,
    pub(crate) capped_container_model: i32,
    pub(crate) procedure_template_capped_container_model: i32,
    pub(crate) procedure_capped_container: ::rosetta_uuid::Uuid,
    pub(crate) capped_with_model: i32,
    pub(crate) procedure_template_capped_with_model: i32,
    pub(crate) procedure_capped_with: ::rosetta_uuid::Uuid,
}
impl InsertableCappingProcedure {
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Procedure, diesel::result::Error>
    where
        crate::Procedure: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Procedure::read(self.procedure, conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::CappingProcedureTemplate, diesel::result::Error>
    where
        crate::CappingProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::CappingProcedureTemplate::read(self.procedure_template, conn)
    }
    pub fn capped_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainer, diesel::result::Error>
    where
        crate::VolumetricContainer: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainer::read(self.capped_container, conn)
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
    pub fn procedure_capped_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_capped_container, conn)
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
    pub fn procedure_capped_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_capped_with, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::Procedure, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<crate::Procedure>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_template_procedure_template_c_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::CappingProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::CappingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::capping_procedure_templates::capping_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::capping_procedure_templates::capping_procedure_templates::dsl::procedure_template_capped_container_model
                            .eq(&self.procedure_template_capped_container_model),
                    ),
            )
            .first::<crate::CappingProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_template_procedure_template_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::CappingProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::CappingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::capping_procedure_templates::capping_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::capping_procedure_templates::capping_procedure_templates::dsl::procedure_template_capped_with_model
                            .eq(&self.procedure_template_capped_with_model),
                    ),
            )
            .first::<crate::CappingProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_capped_container_procedure_te_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_capped_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_capped_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_capped_with_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_capped_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_capped_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_capped_container_capped_conta_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_capped_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.capped_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_capped_with_capped_with_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_capped_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.capped_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn capping_procedures_procedure_capped_container_capped_cont_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_capped_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.capped_container),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn capping_procedures_capped_container_model_capped_with_mode_fkey<
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
pub struct InsertableCappingProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) capped_container: Option<::rosetta_uuid::Uuid>,
    pub(crate) capped_container_model: Option<i32>,
    pub(crate) procedure_template_capped_container_model: Option<i32>,
    pub(crate) procedure_capped_container: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) capped_with_model: Option<i32>,
    pub(crate) procedure_template_capped_with_model: Option<i32>,
    pub(crate) procedure_capped_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl From<InsertableCappingProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableCappingProcedureBuilder,
    >
{
    fn from(builder: InsertableCappingProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of `CappingProcedure`
/// or descendant tables.
pub trait CappingProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.capping_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.capping_procedures.procedure_template` column.
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
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.capping_procedures.capped_container`
    /// column.
    ///
    /// # Arguments
    /// * `capped_container`: The value to set for the
    ///   `public.capping_procedures.capped_container` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn capped_container(
        self,
        capped_container: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.capping_procedures.capped_container_model`
    /// column.
    ///
    /// # Arguments
    /// * `capped_container_model`: The value to set for the
    ///   `public.capping_procedures.capped_container_model` column.
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
    /// `public.capping_procedures.procedure_template_capped_container_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_capped_container_model`: The value to set for the
    ///   `public.capping_procedures.procedure_template_capped_container_model`
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
    fn procedure_template_capped_container_model(
        self,
        procedure_template_capped_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.capping_procedures.procedure_capped_container` column.
    ///
    /// # Arguments
    /// * `procedure_capped_container`: The value to set for the
    ///   `public.capping_procedures.procedure_capped_container` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_capped_container<PCC>(
        self,
        procedure_capped_container: PCC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PCC: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.capping_procedures.capped_with_model`
    /// column.
    ///
    /// # Arguments
    /// * `capped_with_model`: The value to set for the
    ///   `public.capping_procedures.capped_with_model` column.
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
    /// `public.capping_procedures.procedure_template_capped_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_capped_with_model`: The value to set for the
    ///   `public.capping_procedures.procedure_template_capped_with_model`
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
    fn procedure_template_capped_with_model(
        self,
        procedure_template_capped_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.capping_procedures.procedure_capped_with`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_capped_with`: The value to set for the
    ///   `public.capping_procedures.procedure_capped_with` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_capped_with<PCW>(
        self,
        procedure_capped_with: PCW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PCW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> CappingProcedureSettable for InsertableCappingProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute;
    /// Sets the value of the `public.capping_procedures.procedure_template`
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
    /// subgraph v5 ["`capping_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_capped_with_model"}
    /// class v2 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_capped_container_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v6 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 undirectly-involved-column
    /// end
    /// subgraph v7 ["`procedures`"]
    ///    v3@{shape: rounded, label: "procedure_template"}
    /// class v3 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v3
    /// v0 -.->|"`foreign defines`"| v2
    /// v0 -.->|"`foreign defines`"| v1
    /// v2 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v5 --->|"`extends`"| v7
    /// v5 ---o|"`associated with`"| v6
    /// ```
    fn procedure_template(
        mut self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                self.procedure,
                procedure_template,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    /// Sets the value of the `public.capping_procedures.capped_container`
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
    /// subgraph v4 ["`capping_procedures`"]
    ///    v0@{shape: rounded, label: "capped_container"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_capped_container"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn capped_container(
        mut self,
        capped_container: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_capped_container) =
            self.procedure_capped_container
        {
            self.procedure_capped_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_capped_container,
                    Some(capped_container),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureCappedContainer(attribute)
                    })
                })?
                .into();
        }
        self.capped_container = Some(capped_container);
        Ok(self)
    }
    /// Sets the value of the `public.capping_procedures.capped_container_model`
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
    /// subgraph v5 ["`capping_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_capped_container"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "capped_container_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "capped_with_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v6 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "id"}
    /// class v4 undirectly-involved-column
    ///    v3@{shape: rounded, label: "asset_model"}
    /// class v3 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v2 --->|"`associated same as`"| v4
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
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_capped_container) =
            self.procedure_capped_container
        {
            self.procedure_capped_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_capped_container,
                    capped_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureCappedContainer(attribute)
                    })
                })?
                .into();
        }
        self.capped_container_model = Some(capped_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.capping_procedures.procedure_template_capped_container_model`
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
    /// subgraph v4 ["`capping_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_capped_container"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_capped_container_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_capped_container_model(
        mut self,
        procedure_template_capped_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_capped_container) =
            self.procedure_capped_container
        {
            self.procedure_capped_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_capped_container,
                    procedure_template_capped_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureCappedContainer(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_capped_container_model =
            Some(procedure_template_capped_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.capping_procedures.procedure_capped_container` column.
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
    /// subgraph v8 ["`capping_procedures`"]
    ///    v3@{shape: rounded, label: "procedure_template_capped_container_model"}
    /// class v3 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_capped_container"}
    /// class v2 column-of-interest
    ///    v0@{shape: rounded, label: "capped_container"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "capped_container_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v9 ["`procedure_assets`"]
    ///    v7@{shape: rounded, label: "id"}
    /// class v7 undirectly-involved-column
    ///    v5@{shape: rounded, label: "asset_model"}
    /// class v5 directly-involved-column
    ///    v4@{shape: rounded, label: "asset"}
    /// class v4 directly-involved-column
    ///    v6@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v6 directly-involved-column
    /// end
    /// v3 --->|"`associated same as`"| v6
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v2 -.->|"`foreign defines`"| v3
    /// v0 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v5
    /// v4 -.->|"`foreign defines`"| v5
    /// v8 ---o|"`associated with`"| v9
    /// ```
    fn procedure_capped_container<PCC>(
        mut self,
        procedure_capped_container: PCC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PCC: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_capped_container = procedure_capped_container.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_capped_container
        {
            procedure_capped_container = if let (
                Some(procedure_template_capped_container_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_capped_container_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_capped_container_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateCappedContainerModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_capped_container_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_capped_container_model) =
                self.procedure_template_capped_container_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_capped_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureCappedContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_capped_container
        {
            procedure_capped_container = if let (Some(capped_container_model), Some(asset_model)) =
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
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        capped_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureCappedContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_capped_container
        {
            procedure_capped_container = if let (Some(capped_container), Some(asset)) =
                (self.capped_container, builder.asset)
            {
                if capped_container != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::CappedContainer,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.capped_container = Some(asset);
                builder.into()
            } else if let Some(capped_container) = self.capped_container {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(capped_container),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureCappedContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_capped_container = procedure_capped_container;
        Ok(self)
    }
    /// Sets the value of the `public.capping_procedures.capped_with_model`
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
    /// subgraph v5 ["`capping_procedures`"]
    ///    v1@{shape: rounded, label: "capped_with_model"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_capped_with"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "capped_container_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v6 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset_model"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "id"}
    /// class v4 undirectly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v2 --->|"`associated same as`"| v4
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
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_capped_with) =
            self.procedure_capped_with
        {
            self.procedure_capped_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_capped_with,
                    capped_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureCappedWith(attribute)
                    })
                })?
                .into();
        }
        self.capped_with_model = Some(capped_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.capping_procedures.procedure_template_capped_with_model` column.
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
    /// subgraph v4 ["`capping_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_capped_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_capped_with_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_capped_with_model(
        mut self,
        procedure_template_capped_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_capped_with) =
            self.procedure_capped_with
        {
            self.procedure_capped_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_capped_with,
                    procedure_template_capped_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureCappedWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_capped_with_model = Some(procedure_template_capped_with_model);
        Ok(self)
    }
    /// Sets the value of the `public.capping_procedures.procedure_capped_with`
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
    /// subgraph v6 ["`capping_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_capped_with"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_capped_with_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "capped_with_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v3@{shape: rounded, label: "asset_model"}
    /// class v3 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v0 --->|"`associated same as`"| v3
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_capped_with<PCW>(
        mut self,
        procedure_capped_with: PCW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PCW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_capped_with = procedure_capped_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_capped_with {
            procedure_capped_with = if let (
                Some(procedure_template_capped_with_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_capped_with_model, builder.procedure_template_asset_model)
            {
                if procedure_template_capped_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateCappedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_capped_with_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_capped_with_model) =
                self.procedure_template_capped_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_capped_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureCappedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_capped_with {
            procedure_capped_with = if let (Some(capped_with_model), Some(asset_model)) =
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
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        capped_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureCappedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_capped_with = procedure_capped_with;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableCappingProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure(
        mut self,
        procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                self.procedure,
                procedure,
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
    ///Sets the value of the `public.procedures.procedure_template` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`capping_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v3 ["`procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v2 --->|"`extends`"| v3
    ///```
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CappingProcedureSettable>::procedure_template(self, procedure_template)
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure(
        mut self,
        parent_procedure: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                self.procedure,
                parent_procedure,
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
    ///Sets the value of the `public.procedures.parent_procedure_template` column.
    fn parent_procedure_template(
        mut self,
        parent_procedure_template: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                self.procedure,
                parent_procedure_template,
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
    ///Sets the value of the `public.procedures.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                self.procedure,
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
    ///Sets the value of the `public.procedures.created_at` column.
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
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                self.procedure,
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
    ///Sets the value of the `public.procedures.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                self.procedure,
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
    ///Sets the value of the `public.procedures.updated_at` column.
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
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                self.procedure,
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
impl<Procedure> web_common_traits::database::MostConcreteTable
    for InsertableCappingProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableCappingProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure = self.procedure.set_primary_key(primary_key);
        self
    }
}
impl<Procedure, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableCappingProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::CappingProcedure,
            Error = web_common_traits::database::InsertError<CappingProcedureAttribute>,
        >,
    Procedure: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        web_common_traits::database::TryInsertGeneric<C>,
{
    type Attributes = CappingProcedureAttribute;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete()
            && self.procedure_template.is_some()
            && self.capped_container.is_some()
            && self.capped_container_model.is_some()
            && self.procedure_template_capped_container_model.is_some()
            && self.procedure_capped_container.is_complete()
            && self.capped_with_model.is_some()
            && self.procedure_template_capped_with_model.is_some()
            && self.procedure_capped_with.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::CappingProcedure = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
