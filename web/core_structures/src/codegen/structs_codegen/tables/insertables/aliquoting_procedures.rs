#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingProcedureExtensionAttributes {
    Procedure(crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes),
}
impl core::fmt::Display for InsertableAliquotingProcedureExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes>
    for InsertableAliquotingProcedureExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingProcedureAttributes {
    Extension(InsertableAliquotingProcedureExtensionAttributes),
    Procedure,
    ProcedureTemplate,
    ForeignProcedureTemplate,
    ForeignProcedure,
    AliquotedWith,
    PipetteTipModel,
    AliquotedFrom,
}
impl core::str::FromStr for InsertableAliquotingProcedureAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ForeignProcedure" => Ok(Self::ForeignProcedure),
            "AliquotedWith" => Ok(Self::AliquotedWith),
            "PipetteTipModel" => Ok(Self::PipetteTipModel),
            "AliquotedFrom" => Ok(Self::AliquotedFrom),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "foreign_procedure" => Ok(Self::ForeignProcedure),
            "aliquoted_with" => Ok(Self::AliquotedWith),
            "pipette_tip_model" => Ok(Self::PipetteTipModel),
            "aliquoted_from" => Ok(Self::AliquotedFrom),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableAliquotingProcedureAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ForeignProcedure => write!(f, "foreign_procedure"),
            Self::AliquotedWith => write!(f, "aliquoted_with"),
            Self::PipetteTipModel => write!(f, "pipette_tip_model"),
            Self::AliquotedFrom => write!(f, "aliquoted_from"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) foreign_procedure: ::rosetta_uuid::Uuid,
    pub(crate) aliquoted_with: ::rosetta_uuid::Uuid,
    pub(crate) pipette_tip_model: i32,
    pub(crate) aliquoted_from: ::rosetta_uuid::Uuid,
}
impl InsertableAliquotingProcedure {
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedures::Procedure,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedures::Procedure::table(),
                self.procedure,
            ),
            conn,
        )
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table(),
                self.procedure_template,
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
    pub fn foreign_procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedures::Procedure,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedures::Procedure::table(),
                self.foreign_procedure,
            ),
            conn,
        )
    }
    pub fn aliquoted_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipettes::Pipette,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipettes::Pipette: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipettes::Pipette as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipettes::Pipette,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipettes::Pipette::table(),
                self.aliquoted_with,
            ),
            conn,
        )
    }
    pub fn pipette_tip_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::table(
                ),
                self.pipette_tip_model,
            ),
            conn,
        )
    }
    pub fn aliquoted_from<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::table(),
                self.aliquoted_from,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_with_fkey(
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
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.aliquoted_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn aliquoting_procedures_procedure_pipette_tip_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table(),
                (self.procedure, self.pipette_tip_model),
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_foreign_procedure_aliquoted_from_fkey(
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
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure
                    .eq(&self.foreign_procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.aliquoted_from),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) foreign_procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) aliquoted_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) pipette_tip_model: Option<i32>,
    pub(crate) aliquoted_from: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure: Procedure,
}
/// Trait defining setters for attributes of an instance of
/// `AliquotingProcedure` or descendant tables.
pub trait AliquotingProcedureBuildable:
    crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable
{
    /// Sets the value of the `public.aliquoting_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_template` column.
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
    /// Sets the value of the
    /// `public.aliquoting_procedures.foreign_procedure_template` column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `public.aliquoting_procedures.foreign_procedure_template` column.
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
    /// Sets the value of the `public.aliquoting_procedures.foreign_procedure`
    /// column.
    ///
    /// # Arguments
    /// * `foreign_procedure`: The value to set for the
    ///   `public.aliquoting_procedures.foreign_procedure` column.
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
    fn foreign_procedure(
        self,
        foreign_procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_with`
    /// column.
    ///
    /// # Arguments
    /// * `aliquoted_with`: The value to set for the
    ///   `public.aliquoting_procedures.aliquoted_with` column.
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
    fn aliquoted_with(
        self,
        aliquoted_with: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.aliquoting_procedures.pipette_tip_model`
    /// column.
    ///
    /// # Arguments
    /// * `pipette_tip_model`: The value to set for the
    ///   `public.aliquoting_procedures.pipette_tip_model` column.
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
    fn pipette_tip_model(
        self,
        pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_from`
    /// column.
    ///
    /// # Arguments
    /// * `aliquoted_from`: The value to set for the
    ///   `public.aliquoting_procedures.aliquoted_from` column.
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
    fn aliquoted_from(
        self,
        aliquoted_from: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl AliquotingProcedureBuildable for Option<::rosetta_uuid::Uuid> {
    fn procedure_template(
        self,
        _procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn foreign_procedure_template(
        self,
        _foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn foreign_procedure(
        self,
        _foreign_procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn aliquoted_with(
        self,
        _aliquoted_with: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn pipette_tip_model(
        self,
        _pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
    fn aliquoted_from(
        self,
        _aliquoted_from: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
        >,
> AliquotingProcedureBuildable for InsertableAliquotingProcedureBuilder<Procedure> {
    ///Sets the value of the `public.aliquoting_procedures.procedure_template` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v1
    ///v2 --->|"`extends`"| v3
    ///```
    fn procedure_template(
        mut self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let procedure_template = procedure_template
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableAliquotingProcedureAttributes::ProcedureTemplate,
                    )
            })?;
        self.procedure = self
            .procedure
            .procedure_template(procedure_template)
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        if let Some(foreign_procedure_template) = self.foreign_procedure_template {
            pgrx_validation::must_be_distinct_i32(
                    procedure_template,
                    foreign_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::ForeignProcedureTemplate,
                        )
                })?;
        }
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.aliquoting_procedures.foreign_procedure_template` column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let foreign_procedure_template = foreign_procedure_template
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableAliquotingProcedureAttributes::ForeignProcedureTemplate,
                    )
            })?;
        if let Some(procedure_template) = self.procedure_template {
            pgrx_validation::must_be_distinct_i32(
                    procedure_template,
                    foreign_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::ForeignProcedureTemplate,
                        )
                })?;
        }
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.aliquoting_procedures.foreign_procedure` column.
    fn foreign_procedure(
        mut self,
        foreign_procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let foreign_procedure = foreign_procedure
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableAliquotingProcedureAttributes::ForeignProcedure,
                    )
            })?;
        self.foreign_procedure = Some(foreign_procedure);
        Ok(self)
    }
    ///Sets the value of the `public.aliquoting_procedures.aliquoted_with` column.
    fn aliquoted_with(
        mut self,
        aliquoted_with: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let aliquoted_with = aliquoted_with
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableAliquotingProcedureAttributes::AliquotedWith)
            })?;
        self.aliquoted_with = Some(aliquoted_with);
        Ok(self)
    }
    ///Sets the value of the `public.aliquoting_procedures.pipette_tip_model` column.
    fn pipette_tip_model(
        mut self,
        pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let pipette_tip_model = pipette_tip_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableAliquotingProcedureAttributes::PipetteTipModel,
                    )
            })?;
        self.pipette_tip_model = Some(pipette_tip_model);
        Ok(self)
    }
    ///Sets the value of the `public.aliquoting_procedures.aliquoted_from` column.
    fn aliquoted_from(
        mut self,
        aliquoted_from: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let aliquoted_from = aliquoted_from
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableAliquotingProcedureAttributes::AliquotedFrom)
            })?;
        self.aliquoted_from = Some(aliquoted_from);
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable
for InsertableAliquotingProcedureBuilder<Procedure> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure(
        mut self,
        procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::procedure(
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
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`aliquoting_procedures`"]
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
        <Self as AliquotingProcedureBuildable>::procedure_template(
            self,
            procedure_template,
        )
    }
    #[inline]
    ///Sets the value of the `public.procedures.most_concrete_table` column.
    fn most_concrete_table<MCT>(
        mut self,
        most_concrete_table: MCT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        MCT: TryInto<String>,
        validation_errors::SingleFieldError: From<<MCT as TryInto<String>>::Error>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::most_concrete_table(
                self.procedure,
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
    ///Sets the value of the `public.procedures.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::created_by(
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
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::created_at(
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
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::updated_by(
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
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::updated_at(
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
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableAliquotingProcedureBuilder<Procedure>
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
for InsertableAliquotingProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
        Error = web_common_traits::database::InsertError<
            InsertableAliquotingProcedureAttributes,
        >,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Attributes = InsertableAliquotingProcedureAttributes;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete() && self.procedure_template.is_some()
            && self.foreign_procedure_template.is_some()
            && self.foreign_procedure.is_some() && self.aliquoted_with.is_some()
            && self.pipette_tip_model.is_some() && self.aliquoted_from.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
