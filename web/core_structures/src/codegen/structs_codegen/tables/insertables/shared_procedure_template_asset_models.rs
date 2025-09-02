#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSharedProcedureTemplateAssetModelAttributes {
    Parent,
    ParentAssetModel,
    ParentProcedureTemplate,
    ChildId,
    ChildAssetModel,
    ChildProcedureTemplate,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for InsertableSharedProcedureTemplateAssetModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Parent" => Ok(Self::Parent),
            "ParentAssetModel" => Ok(Self::ParentAssetModel),
            "ParentProcedureTemplate" => Ok(Self::ParentProcedureTemplate),
            "ChildId" => Ok(Self::ChildId),
            "ChildAssetModel" => Ok(Self::ChildAssetModel),
            "ChildProcedureTemplate" => Ok(Self::ChildProcedureTemplate),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "parent" => Ok(Self::Parent),
            "parent_asset_model" => Ok(Self::ParentAssetModel),
            "parent_procedure_template" => Ok(Self::ParentProcedureTemplate),
            "child_id" => Ok(Self::ChildId),
            "child_asset_model" => Ok(Self::ChildAssetModel),
            "child_procedure_template" => Ok(Self::ChildProcedureTemplate),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableSharedProcedureTemplateAssetModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Parent => write!(f, "parent"),
            Self::ParentAssetModel => write!(f, "parent_asset_model"),
            Self::ParentProcedureTemplate => write!(f, "parent_procedure_template"),
            Self::ChildId => write!(f, "child_id"),
            Self::ChildAssetModel => write!(f, "child_asset_model"),
            Self::ChildProcedureTemplate => write!(f, "child_procedure_template"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::shared_procedure_template_asset_models::shared_procedure_template_asset_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSharedProcedureTemplateAssetModel {
    pub(crate) parent: i32,
    pub(crate) parent_asset_model: i32,
    pub(crate) parent_procedure_template: i32,
    pub(crate) child_id: i32,
    pub(crate) child_asset_model: i32,
    pub(crate) child_procedure_template: i32,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableSharedProcedureTemplateAssetModel {
    pub fn parent<C: diesel::connection::LoadConnection>(
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
                self.parent,
            ),
            conn,
        )
    }
    pub fn parent_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.parent_asset_model,
            ),
            conn,
        )
    }
    pub fn parent_procedure_template<C: diesel::connection::LoadConnection>(
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
                self.parent_procedure_template,
            ),
            conn,
        )
    }
    pub fn child<C: diesel::connection::LoadConnection>(
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
                self.child_id,
            ),
            conn,
        )
    }
    pub fn child_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.child_asset_model,
            ),
            conn,
        )
    }
    pub fn child_procedure_template<C: diesel::connection::LoadConnection>(
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
                self.child_procedure_template,
            ),
            conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn shared_procedure_template_ass_parent_procedure_template_ch_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::table(),
                (self.parent_procedure_template, self.child_procedure_template),
            ),
            conn,
        )
    }
    pub fn shared_procedure_template_ass_parent_asset_model_child_ass_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor::table(),
                (self.parent_asset_model, self.child_asset_model),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSharedProcedureTemplateAssetModelBuilder {
    pub(crate) parent: Option<i32>,
    pub(crate) parent_asset_model: Option<i32>,
    pub(crate) parent_procedure_template: Option<i32>,
    pub(crate) child_id: Option<i32>,
    pub(crate) child_asset_model: Option<i32>,
    pub(crate) child_procedure_template: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableSharedProcedureTemplateAssetModelBuilder {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            parent_asset_model: Default::default(),
            parent_procedure_template: Default::default(),
            child_id: Default::default(),
            child_asset_model: Default::default(),
            child_procedure_template: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `SharedProcedureTemplateAssetModel` or descendant tables.
pub trait SharedProcedureTemplateAssetModelBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.parent` column.
    ///
    /// # Arguments
    /// * `parent`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.parent` column.
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
    fn parent(
        self,
        parent: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.parent_asset_model`
    /// column.
    ///
    /// # Arguments
    /// * `parent_asset_model`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.parent_asset_model`
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
    fn parent_asset_model(
        self,
        parent_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.
    /// parent_procedure_template` column.
    ///
    /// # Arguments
    /// * `parent_procedure_template`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.
    ///   parent_procedure_template` column.
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
    fn parent_procedure_template(
        self,
        parent_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.child_id` column.
    ///
    /// # Arguments
    /// * `child_id`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.child_id` column.
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
    fn child(
        self,
        child_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.child_asset_model`
    /// column.
    ///
    /// # Arguments
    /// * `child_asset_model`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.child_asset_model`
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
    fn child_asset_model(
        self,
        child_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.child_procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `child_procedure_template`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.
    ///   child_procedure_template` column.
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
    fn child_procedure_template(
        self,
        child_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.created_by` column.
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
    fn created_by(
        self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.shared_procedure_template_asset_models.created_at` column.
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
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_at<CA>(
        self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
}
impl SharedProcedureTemplateAssetModelBuildable
    for InsertableSharedProcedureTemplateAssetModelBuilder
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureTemplateAssetModelAttributes;
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.parent` column.
    fn parent(
        mut self,
        parent: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let parent = parent.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableSharedProcedureTemplateAssetModelAttributes::Parent)
        })?;
        if let Some(child_id) = self.child_id {
            pgrx_validation::must_be_distinct_i32(parent, child_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureTemplateAssetModelAttributes::Parent,
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureTemplateAssetModelAttributes::ChildId,
                        )
                })?;
        }
        self.parent = Some(parent);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.parent_asset_model`
    /// column.
    fn parent_asset_model(
        mut self,
        parent_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let parent_asset_model = parent_asset_model.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(
                InsertableSharedProcedureTemplateAssetModelAttributes::ParentAssetModel,
            )
        })?;
        self.parent_asset_model = Some(parent_asset_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.
    /// parent_procedure_template` column.
    fn parent_procedure_template(
        mut self,
        parent_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let parent_procedure_template = parent_procedure_template.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(
                InsertableSharedProcedureTemplateAssetModelAttributes::ParentProcedureTemplate,
            )
        })?;
        self.parent_procedure_template = Some(parent_procedure_template);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.child_id` column.
    fn child(
        mut self,
        child_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let child_id = child_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableSharedProcedureTemplateAssetModelAttributes::ChildId)
        })?;
        if let Some(parent) = self.parent {
            pgrx_validation::must_be_distinct_i32(parent, child_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureTemplateAssetModelAttributes::Parent,
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureTemplateAssetModelAttributes::ChildId,
                        )
                })?;
        }
        self.child_id = Some(child_id);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.child_asset_model`
    /// column.
    fn child_asset_model(
        mut self,
        child_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let child_asset_model = child_asset_model.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(
                InsertableSharedProcedureTemplateAssetModelAttributes::ChildAssetModel,
            )
        })?;
        self.child_asset_model = Some(child_asset_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.child_procedure_template`
    /// column.
    fn child_procedure_template(
        mut self,
        child_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let child_procedure_template = child_procedure_template.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(
                InsertableSharedProcedureTemplateAssetModelAttributes::ChildProcedureTemplate,
            )
        })?;
        self.child_procedure_template = Some(child_procedure_template);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let created_by = created_by.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableSharedProcedureTemplateAssetModelAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.shared_procedure_template_asset_models.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableSharedProcedureTemplateAssetModelAttributes::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey
    for InsertableSharedProcedureTemplateAssetModelBuilder
{
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableSharedProcedureTemplateAssetModelBuilder
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel,
        Error = web_common_traits::database::InsertError<
            InsertableSharedProcedureTemplateAssetModelAttributes,
        >,
    >,
{
    type Attributes = InsertableSharedProcedureTemplateAssetModelAttributes;
    fn is_complete(&self) -> bool {
        self.parent.is_some() && self.parent_asset_model.is_some()
            && self.parent_procedure_template.is_some() && self.child_id.is_some()
            && self.child_asset_model.is_some()
            && self.child_procedure_template.is_some() && self.created_by.is_some()
            && self.created_at.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
