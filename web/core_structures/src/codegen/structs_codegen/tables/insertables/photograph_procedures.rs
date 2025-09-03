#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhotographProcedureExtensionAttributes {
    Procedure(crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes),
}
impl core::fmt::Display for InsertablePhotographProcedureExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes>
    for InsertablePhotographProcedureExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePhotographProcedureAttributes {
    Extension(InsertablePhotographProcedureExtensionAttributes),
    Procedure,
    ProcedureTemplate,
    ForeignProcedureTemplate,
    ForeignProcedure,
    PhotographedAsset,
    PhotographedWith,
    PhotographedWithModel,
}
impl core::str::FromStr for InsertablePhotographProcedureAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ForeignProcedure" => Ok(Self::ForeignProcedure),
            "PhotographedAsset" => Ok(Self::PhotographedAsset),
            "PhotographedWith" => Ok(Self::PhotographedWith),
            "PhotographedWithModel" => Ok(Self::PhotographedWithModel),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "foreign_procedure" => Ok(Self::ForeignProcedure),
            "photographed_asset" => Ok(Self::PhotographedAsset),
            "photographed_with" => Ok(Self::PhotographedWith),
            "photographed_with_model" => Ok(Self::PhotographedWithModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertablePhotographProcedureAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ForeignProcedure => write!(f, "foreign_procedure"),
            Self::PhotographedAsset => write!(f, "photographed_asset"),
            Self::PhotographedWith => write!(f, "photographed_with"),
            Self::PhotographedWithModel => write!(f, "photographed_with_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhotographProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) foreign_procedure: ::rosetta_uuid::Uuid,
    pub(crate) photographed_asset: ::rosetta_uuid::Uuid,
    pub(crate) photographed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) photographed_with_model: i32,
}
impl InsertablePhotographProcedure {
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
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table(),
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
    pub fn photographed_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::table(),
                self.photographed_asset,
            ),
            conn,
        )
    }
    pub fn photographed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::cameras::Camera>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cameras::Camera: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::cameras::Camera as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::cameras::Camera as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cameras::Camera as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::cameras::Camera,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(photographed_with) = self.photographed_with else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::cameras::Camera::table(),
                photographed_with,
            ),
            conn,
        )
        .map(Some)
    }
    pub fn photographed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::camera_models::CameraModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::camera_models::CameraModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::camera_models::CameraModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::camera_models::CameraModel::table(),
                self.photographed_with_model,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_foreign_procedure_photographed_asset_fkey(
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
                            .eq(&self.photographed_asset),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn photograph_procedures_procedure_photographed_with_model_fkey<
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
                (self.procedure, self.photographed_with_model),
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_with_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(photographed_with) = self.photographed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(photographed_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .map(Some)
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePhotographProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) foreign_procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) photographed_asset: Option<::rosetta_uuid::Uuid>,
    pub(crate) photographed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) photographed_with_model: Option<i32>,
    pub(crate) procedure: Procedure,
}
/// Trait defining setters for attributes of an instance of
/// `PhotographProcedure` or descendant tables.
pub trait PhotographProcedureBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.photograph_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.photograph_procedures.procedure_template` column.
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
    /// `public.photograph_procedures.foreign_procedure_template` column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `public.photograph_procedures.foreign_procedure_template` column.
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
    /// Sets the value of the `public.photograph_procedures.foreign_procedure`
    /// column.
    ///
    /// # Arguments
    /// * `foreign_procedure`: The value to set for the
    ///   `public.photograph_procedures.foreign_procedure` column.
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
    /// Sets the value of the `public.photograph_procedures.photographed_asset`
    /// column.
    ///
    /// # Arguments
    /// * `photographed_asset`: The value to set for the
    ///   `public.photograph_procedures.photographed_asset` column.
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
    fn photographed_asset(
        self,
        photographed_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.photograph_procedures.photographed_with`
    /// column.
    ///
    /// # Arguments
    /// * `photographed_with`: The value to set for the
    ///   `public.photograph_procedures.photographed_with` column.
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
    fn photographed_with(
        self,
        photographed_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.photograph_procedures.photographed_with_model` column.
    ///
    /// # Arguments
    /// * `photographed_with_model`: The value to set for the
    ///   `public.photograph_procedures.photographed_with_model` column.
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
    fn photographed_with_model(
        self,
        photographed_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
        >,
> PhotographProcedureBuildable for InsertablePhotographProcedureBuilder<Procedure> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes;
    ///Sets the value of the `public.photograph_procedures.procedure_template` column.
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`photograph_procedure_templates`"]
    ///    v2@{shape: rounded, label: "procedure_template"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v5 ["`procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v1
    ///v0 --->|"`associated same as`"| v2
    ///v4 --->|"`extends`"| v5
    ///v4 ---o|"`associated with`"| v3
    ///```
    fn procedure_template(
        mut self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::procedure_template(
                self.procedure,
                procedure_template,
            )
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
                            crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::ForeignProcedureTemplate,
                        )
                })?;
        }
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.foreign_procedure_template` column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(procedure_template) = self.procedure_template {
            pgrx_validation::must_be_distinct_i32(
                    procedure_template,
                    foreign_procedure_template,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::ProcedureTemplate,
                            crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes::ForeignProcedureTemplate,
                        )
                })?;
        }
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.foreign_procedure` column.
    fn foreign_procedure(
        mut self,
        foreign_procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.foreign_procedure = Some(foreign_procedure);
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.photographed_asset` column.
    fn photographed_asset(
        mut self,
        photographed_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.photographed_asset = Some(photographed_asset);
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.photographed_with` column.
    fn photographed_with(
        mut self,
        photographed_with: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.photographed_with = photographed_with;
        Ok(self)
    }
    ///Sets the value of the `public.photograph_procedures.photographed_with_model` column.
    fn photographed_with_model(
        mut self,
        photographed_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.photographed_with_model = Some(photographed_with_model);
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable
for InsertablePhotographProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhotographProcedureBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureAttributes;
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`photograph_procedure_templates`"]
    ///    v2@{shape: rounded, label: "procedure_template"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`photograph_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v1 --->|"`associated same as`"| v2
    ///v4 --->|"`extends`"| v5
    ///v4 ---o|"`associated with`"| v3
    ///```
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as PhotographProcedureBuildable>::procedure_template(
            self,
            procedure_template,
        )
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
impl<Procedure> web_common_traits::database::MostConcreteTable
    for InsertablePhotographProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertablePhotographProcedureBuilder<Procedure>
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
for InsertablePhotographProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
        Error = web_common_traits::database::InsertError<
            InsertablePhotographProcedureAttributes,
        >,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Attributes = InsertablePhotographProcedureAttributes;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete() && self.procedure_template.is_some()
            && self.foreign_procedure_template.is_some()
            && self.foreign_procedure.is_some() && self.photographed_asset.is_some()
            && self.photographed_with_model.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
