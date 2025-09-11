#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FractioningProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for FractioningProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for FractioningProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FractioningProcedureAttribute {
    Extension(FractioningProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    FragmentContainer,
    ProcedureTemplateFragmentContainerModel,
    ProcedureFragmentContainer(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    FragmentPlacedInto,
    ProcedureTemplateFragmentPlacedIntoModel,
    ProcedureFragmentPlacedInto(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    Kilograms,
    WeighedWith,
    ProcedureTemplateWeighedWithModel,
    ProcedureWeighedWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for FractioningProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "FragmentContainer" => Ok(Self::FragmentContainer),
            "ProcedureTemplateFragmentContainerModel" => {
                Ok(Self::ProcedureTemplateFragmentContainerModel)
            }
            "ProcedureFragmentContainer" => Ok(Self::ProcedureFragmentContainer(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "FragmentPlacedInto" => Ok(Self::FragmentPlacedInto),
            "ProcedureTemplateFragmentPlacedIntoModel" => {
                Ok(Self::ProcedureTemplateFragmentPlacedIntoModel)
            }
            "ProcedureFragmentPlacedInto" => Ok(Self::ProcedureFragmentPlacedInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "Kilograms" => Ok(Self::Kilograms),
            "WeighedWith" => Ok(Self::WeighedWith),
            "ProcedureTemplateWeighedWithModel" => Ok(Self::ProcedureTemplateWeighedWithModel),
            "ProcedureWeighedWith" => Ok(Self::ProcedureWeighedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "fragment_container" => Ok(Self::FragmentContainer),
            "procedure_template_fragment_container_model" => {
                Ok(Self::ProcedureTemplateFragmentContainerModel)
            }
            "procedure_fragment_container" => Ok(Self::ProcedureFragmentContainer(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "fragment_placed_into" => Ok(Self::FragmentPlacedInto),
            "procedure_template_fragment_placed_into_model" => {
                Ok(Self::ProcedureTemplateFragmentPlacedIntoModel)
            }
            "procedure_fragment_placed_into" => Ok(Self::ProcedureFragmentPlacedInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "kilograms" => Ok(Self::Kilograms),
            "weighed_with" => Ok(Self::WeighedWith),
            "procedure_template_weighed_with_model" => Ok(Self::ProcedureTemplateWeighedWithModel),
            "procedure_weighed_with" => Ok(Self::ProcedureWeighedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    > for FractioningProcedureAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure
                .into(),
        )
    }
}
impl
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    > for FractioningProcedureAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for FractioningProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "fractioning_procedures.procedure"),
            Self::ProcedureTemplate => {
                write!(f, "fractioning_procedures.procedure_template")
            }
            Self::FragmentContainer => {
                write!(f, "fractioning_procedures.fragment_container")
            }
            Self::ProcedureTemplateFragmentContainerModel => {
                write!(f, "fractioning_procedures.procedure_template_fragment_container_model")
            }
            Self::ProcedureFragmentContainer(e) => {
                write!(f, "fractioning_procedures.{e}")
            }
            Self::FragmentPlacedInto => {
                write!(f, "fractioning_procedures.fragment_placed_into")
            }
            Self::ProcedureTemplateFragmentPlacedIntoModel => {
                write!(f, "fractioning_procedures.procedure_template_fragment_placed_into_model")
            }
            Self::ProcedureFragmentPlacedInto(e) => {
                write!(f, "fractioning_procedures.{e}")
            }
            Self::Kilograms => write!(f, "fractioning_procedures.kilograms"),
            Self::WeighedWith => write!(f, "fractioning_procedures.weighed_with"),
            Self::ProcedureTemplateWeighedWithModel => {
                write!(f, "fractioning_procedures.procedure_template_weighed_with_model")
            }
            Self::ProcedureWeighedWith(e) => write!(f, "fractioning_procedures.{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) fragment_container: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_fragment_container_model: i32,
    pub(crate) procedure_fragment_container: ::rosetta_uuid::Uuid,
    pub(crate) fragment_placed_into: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_fragment_placed_into_model: i32,
    pub(crate) procedure_fragment_placed_into: ::rosetta_uuid::Uuid,
    pub(crate) kilograms: f32,
    pub(crate) weighed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_weighed_with_model: i32,
    pub(crate) procedure_weighed_with: ::rosetta_uuid::Uuid,
}
impl InsertableFractioningProcedure {
    pub fn fragment_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
            self.fragment_container,
            conn,
        )
    }
    pub fn fragment_placed_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
            self.fragment_placed_into,
            conn,
        )
    }
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedures::Procedure::read(self.procedure, conn)
    }
    pub fn procedure_fragment_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_fragment_container,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_fragment_container_fragme_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_fragment_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.fragment_container),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_fragment_container_proced_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_fragment_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_fragment_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_fragment_placed_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_fragment_placed_into,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_fragment_placed_into_frag_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_fragment_placed_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.fragment_placed_into),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_fragment_placed_into_proc_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_fragment_placed_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_fragment_placed_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    pub fn procedure_template_fragment_container_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_fragment_container_model,
            conn,
        )
    }
    pub fn procedure_template_fragment_placed_into_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_fragment_placed_into_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_template_procedure_templ_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::dsl::procedure_template_fragment_container_model
                            .eq(&self.procedure_template_fragment_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_template_procedure_templ_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::dsl::procedure_template_fragment_placed_into_model
                            .eq(&self.procedure_template_fragment_placed_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_template_procedure_templa_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::fractioning_procedure_templates::fractioning_procedure_templates::dsl::procedure_template_weighed_with_model
                            .eq(&self.procedure_template_weighed_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
            >(conn)
    }
    pub fn procedure_template_weighed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_weighed_with_model,
            conn,
        )
    }
    pub fn procedure_weighed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_weighed_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_weighed_with_procedure_te_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_weighed_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_weighed_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_weighed_with_weighed_with_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(weighed_with) = self.weighed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_weighed_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(weighed_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    pub fn weighed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(weighed_with) = self.weighed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice::read(
            weighed_with,
            conn,
        )
        .optional()
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) fragment_container: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_fragment_container_model: Option<i32>,
    pub(crate) procedure_fragment_container: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) fragment_placed_into: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_fragment_placed_into_model: Option<i32>,
    pub(crate) procedure_fragment_placed_into: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) kilograms: Option<f32>,
    pub(crate) weighed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_weighed_with_model: Option<i32>,
    pub(crate) procedure_weighed_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl From<InsertableFractioningProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableFractioningProcedureBuilder,
    >
{
    fn from(builder: InsertableFractioningProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Procedure> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder<
        Procedure,
    >
where
    Procedure: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure.is_complete()
            && self.procedure_template.is_some()
            && (self.fragment_container.is_some()
                || self.procedure_fragment_container.is_complete())
            && (self.procedure_template_fragment_container_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_fragment_container.is_complete())
            && self.procedure_fragment_container.is_complete()
            && (self.fragment_placed_into.is_some()
                || self.procedure_fragment_placed_into.is_complete())
            && (self.procedure_template_fragment_placed_into_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_fragment_placed_into.is_complete())
            && self.procedure_fragment_placed_into.is_complete()
            && self.kilograms.is_some()
            && (self.procedure_template_weighed_with_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_weighed_with.is_complete())
            && self.procedure_weighed_with.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `FractioningProcedure` or descendant tables.
pub trait FractioningProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.fractioning_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.fractioning_procedures.procedure_template` column.
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
    fn procedure_template<PT>(
        self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.fractioning_procedures.fragment_container`
    /// column.
    ///
    /// # Arguments
    /// * `fragment_container`: The value to set for the
    ///   `public.fractioning_procedures.fragment_container` column.
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
    fn fragment_container<FC>(
        self,
        fragment_container: FC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        FC: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.fractioning_procedures.
    /// procedure_template_fragment_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_fragment_container_model`: The value to set for
    ///   the `public.fractioning_procedures.
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
        PTFCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_fragment_container` column.
    ///
    /// # Arguments
    /// * `procedure_fragment_container`: The value to set for the
    ///   `public.fractioning_procedures.procedure_fragment_container` column.
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
    fn procedure_fragment_container<PFC>(
        self,
        procedure_fragment_container: PFC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PFC: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.fractioning_procedures.fragment_placed_into` column.
    ///
    /// # Arguments
    /// * `fragment_placed_into`: The value to set for the
    ///   `public.fractioning_procedures.fragment_placed_into` column.
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
    fn fragment_placed_into<FPI>(
        self,
        fragment_placed_into: FPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        FPI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.fractioning_procedures.
    /// procedure_template_fragment_placed_into_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_fragment_placed_into_model`: The value to set for
    ///   the `public.fractioning_procedures.
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
        PTFPIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_fragment_placed_into` column.
    ///
    /// # Arguments
    /// * `procedure_fragment_placed_into`: The value to set for the
    ///   `public.fractioning_procedures.procedure_fragment_placed_into` column.
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
    fn procedure_fragment_placed_into<PFPI>(
        self,
        procedure_fragment_placed_into: PFPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PFPI: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.fractioning_procedures.kilograms` column.
    ///
    /// # Arguments
    /// * `kilograms`: The value to set for the
    ///   `public.fractioning_procedures.kilograms` column.
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
    /// Sets the value of the `public.fractioning_procedures.weighed_with`
    /// column.
    ///
    /// # Arguments
    /// * `weighed_with`: The value to set for the
    ///   `public.fractioning_procedures.weighed_with` column.
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
    fn weighed_with<WW>(
        self,
        weighed_with: WW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        WW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_template_weighed_with_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_weighed_with_model`: The value to set for the
    ///   `public.fractioning_procedures.procedure_template_weighed_with_model`
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
    fn procedure_template_weighed_with_model<PTWWM>(
        self,
        procedure_template_weighed_with_model: PTWWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTWWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_weighed_with` column.
    ///
    /// # Arguments
    /// * `procedure_weighed_with`: The value to set for the
    ///   `public.fractioning_procedures.procedure_weighed_with` column.
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
    fn procedure_weighed_with<PWW>(
        self,
        procedure_weighed_with: PWW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PWW: Into<
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
> FractioningProcedureSettable for InsertableFractioningProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute;
    /// Sets the value of the `public.fractioning_procedures.procedure_template`
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
    /// subgraph v6 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_fragment_container_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_fragment_placed_into_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_template_weighed_with_model"}
    /// class v3 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v5 undirectly-involved-column
    /// end
    /// subgraph v8 ["`procedures`"]
    ///    v4@{shape: rounded, label: "procedure_template"}
    /// class v4 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v4
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v0 -.->|"`foreign defines`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v6 --->|"`extends`"| v8
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_template<PT>(
        mut self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template =
            <PT as web_common_traits::database::PrimaryKeyLike>::primary_key(&procedure_template);
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
    /// Sets the value of the `public.fractioning_procedures.fragment_container`
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
    /// subgraph v4 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "fragment_container"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_fragment_container"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn fragment_container<FC>(
        mut self,
        fragment_container: FC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        FC: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let fragment_container =
            <FC as web_common_traits::database::PrimaryKeyLike>::primary_key(&fragment_container);
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_fragment_container) =
            self.procedure_fragment_container
        {
            self.procedure_fragment_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_fragment_container,
                    fragment_container,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureFragmentContainer(attribute)
                    })
                })?
                .into();
        }
        self.fragment_container = Some(fragment_container);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedures.
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
    /// subgraph v4 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_fragment_container"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_fragment_container_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_fragment_container_model<PTFCM>(
        mut self,
        procedure_template_fragment_container_model: PTFCM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTFCM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_fragment_container_model =
            <PTFCM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_fragment_container_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_fragment_container) =
            self.procedure_fragment_container
        {
            self.procedure_fragment_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_fragment_container,
                    procedure_template_fragment_container_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureFragmentContainer(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_fragment_container_model =
            Some(procedure_template_fragment_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_fragment_container` column.
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
    /// subgraph v6 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "fragment_container"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_fragment_container"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_fragment_container_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_fragment_container<PFC>(
        mut self,
        procedure_fragment_container: PFC,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PFC: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_fragment_container = procedure_fragment_container.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_fragment_container
        {
            procedure_fragment_container = if let (Some(fragment_container), Some(asset)) =
                (self.fragment_container, builder.asset)
            {
                if fragment_container != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::FragmentContainer,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.fragment_container = Some(asset);
                builder.into()
            } else if let Some(fragment_container) = self.fragment_container {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        fragment_container,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureFragmentContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_fragment_container
        {
            procedure_fragment_container = if let (
                Some(procedure_template_fragment_container_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_fragment_container_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_fragment_container_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateFragmentContainerModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_fragment_container_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_fragment_container_model) =
                self.procedure_template_fragment_container_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_fragment_container_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureFragmentContainer(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_fragment_container = procedure_fragment_container;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedures.fragment_placed_into` column.
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
    /// subgraph v4 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "fragment_placed_into"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_fragment_placed_into"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn fragment_placed_into<FPI>(
        mut self,
        fragment_placed_into: FPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        FPI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let fragment_placed_into =
            <FPI as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &fragment_placed_into,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_fragment_placed_into) =
            self.procedure_fragment_placed_into
        {
            self.procedure_fragment_placed_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_fragment_placed_into,
                    fragment_placed_into,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureFragmentPlacedInto(attribute)
                    })
                })?
                .into();
        }
        self.fragment_placed_into = Some(fragment_placed_into);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedures.
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
    /// subgraph v4 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_fragment_placed_into"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_fragment_placed_into_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_fragment_placed_into_model<PTFPIM>(
        mut self,
        procedure_template_fragment_placed_into_model: PTFPIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTFPIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_fragment_placed_into_model =
            <PTFPIM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_fragment_placed_into_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_fragment_placed_into) =
            self.procedure_fragment_placed_into
        {
            self.procedure_fragment_placed_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_fragment_placed_into,
                    procedure_template_fragment_placed_into_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureFragmentPlacedInto(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_fragment_placed_into_model =
            Some(procedure_template_fragment_placed_into_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_fragment_placed_into` column.
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
    /// subgraph v6 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "fragment_placed_into"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_fragment_placed_into"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_fragment_placed_into_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_fragment_placed_into<PFPI>(
        mut self,
        procedure_fragment_placed_into: PFPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PFPI: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_fragment_placed_into = procedure_fragment_placed_into.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_fragment_placed_into
        {
            procedure_fragment_placed_into = if let (Some(fragment_placed_into), Some(asset)) =
                (self.fragment_placed_into, builder.asset)
            {
                if fragment_placed_into != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::FragmentPlacedInto,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.fragment_placed_into = Some(asset);
                builder.into()
            } else if let Some(fragment_placed_into) = self.fragment_placed_into {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        fragment_placed_into,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureFragmentPlacedInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_fragment_placed_into
        {
            procedure_fragment_placed_into = if let (
                Some(procedure_template_fragment_placed_into_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_fragment_placed_into_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_fragment_placed_into_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateFragmentPlacedIntoModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_fragment_placed_into_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_fragment_placed_into_model) =
                self.procedure_template_fragment_placed_into_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_fragment_placed_into_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureFragmentPlacedInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_fragment_placed_into = procedure_fragment_placed_into;
        Ok(self)
    }
    /// Sets the value of the `public.fractioning_procedures.kilograms` column.
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
                .rename_field(FractioningProcedureAttribute::Kilograms)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::Kilograms,
                    )
            })?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
    /// Sets the value of the `public.fractioning_procedures.weighed_with`
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
    /// subgraph v4 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_weighed_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "weighed_with"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn weighed_with<WW>(
        mut self,
        weighed_with: WW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        WW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let weighed_with =
            <WW as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &weighed_with,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_weighed_with) =
            self.procedure_weighed_with
        {
            self.procedure_weighed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_weighed_with,
                    weighed_with,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureWeighedWith(attribute)
                    })
                })?
                .into();
        }
        self.weighed_with = weighed_with;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_template_weighed_with_model`
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
    /// subgraph v4 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template_weighed_with_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_weighed_with"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_weighed_with_model<PTWWM>(
        mut self,
        procedure_template_weighed_with_model: PTWWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTWWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_weighed_with_model =
            <PTWWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_weighed_with_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_weighed_with) =
            self.procedure_weighed_with
        {
            self.procedure_weighed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_weighed_with,
                    procedure_template_weighed_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureWeighedWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_weighed_with_model = Some(procedure_template_weighed_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.fractioning_procedures.procedure_weighed_with` column.
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
    /// subgraph v6 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template_weighed_with_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_weighed_with"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "weighed_with"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v3
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_weighed_with<PWW>(
        mut self,
        procedure_weighed_with: PWW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PWW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_weighed_with = procedure_weighed_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_weighed_with {
            procedure_weighed_with = if let (
                Some(procedure_template_weighed_with_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_weighed_with_model, builder.procedure_template_asset_model)
            {
                if procedure_template_weighed_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateWeighedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_weighed_with_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_weighed_with_model) =
                self.procedure_template_weighed_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_weighed_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureWeighedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_weighed_with {
            procedure_weighed_with = if let (Some(weighed_with), Some(asset)) =
                (self.weighed_with, builder.asset)
            {
                if weighed_with != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::WeighedWith,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.weighed_with = Some(asset);
                builder.into()
            } else if let Some(weighed_with) = self.weighed_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        weighed_with,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureWeighedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_weighed_with = procedure_weighed_with;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableFractioningProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure<P>(
        mut self,
        procedure: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
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
    ///subgraph v2 ["`fractioning_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v3 ["`procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v1
    ///v2 --->|"`extends`"| v3
    ///```
    fn procedure_template<PT>(
        self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as FractioningProcedureSettable>::procedure_template(
            self,
            procedure_template,
        )
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure<PP>(
        mut self,
        parent_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
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
    fn parent_procedure_template<PPT>(
        mut self,
        parent_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
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
    ///Sets the value of the `public.procedures.predecessor_procedure` column.
    fn predecessor_procedure<PP>(
        mut self,
        predecessor_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                self.procedure,
                predecessor_procedure,
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
    ///Sets the value of the `public.procedures.predecessor_procedure_template` column.
    fn predecessor_procedure_template<PPT>(
        mut self,
        predecessor_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                self.procedure,
                predecessor_procedure_template,
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
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    #[inline]
    ///Sets the value of the `public.procedures.number_of_completed_subprocedures` column.
    fn number_of_completed_subprocedures<NOCS>(
        mut self,
        number_of_completed_subprocedures: NOCS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOCS: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOCS as TryInto<i16>>::Error>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::number_of_completed_subprocedures(
                self.procedure,
                number_of_completed_subprocedures,
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
    for InsertableFractioningProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableFractioningProcedureBuilder<Procedure>
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
for InsertableFractioningProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
        Error = web_common_traits::database::InsertError<FractioningProcedureAttribute>,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attribute = FractioningProcedureAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
