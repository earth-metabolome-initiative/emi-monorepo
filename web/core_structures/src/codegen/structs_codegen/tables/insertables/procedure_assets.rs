#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProcedureAssetAttribute {
    Id,
    Procedure,
    ProcedureTemplate,
    AssetModel,
    Asset,
    ProcedureTemplateAssetModel,
    AncestorModel,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for ProcedureAssetAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "Procedure" => Ok(Self::Procedure),
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "AssetModel" => Ok(Self::AssetModel),
            "Asset" => Ok(Self::Asset),
            "ProcedureTemplateAssetModel" => Ok(Self::ProcedureTemplateAssetModel),
            "AncestorModel" => Ok(Self::AncestorModel),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "id" => Ok(Self::Id),
            "procedure" => Ok(Self::Procedure),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "asset_model" => Ok(Self::AssetModel),
            "asset" => Ok(Self::Asset),
            "procedure_template_asset_model" => Ok(Self::ProcedureTemplateAssetModel),
            "ancestor_model" => Ok(Self::AncestorModel),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for ProcedureAssetAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "procedure_assets.id"),
            Self::Procedure => write!(f, "procedure_assets.procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_assets.procedure_template"),
            Self::AssetModel => write!(f, "procedure_assets.asset_model"),
            Self::Asset => write!(f, "procedure_assets.asset"),
            Self::ProcedureTemplateAssetModel => {
                write!(f, "procedure_assets.procedure_template_asset_model")
            }
            Self::AncestorModel => write!(f, "procedure_assets.ancestor_model"),
            Self::CreatedBy => write!(f, "procedure_assets.created_by"),
            Self::CreatedAt => write!(f, "procedure_assets.created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureAsset {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) asset_model: i32,
    pub(crate) asset: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_asset_model: i32,
    pub(crate) ancestor_model: i32,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureAsset {
    pub fn ancestor_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::read(
            self.ancestor_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_assets_asset_asset_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::assets::Asset>, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(asset) = self.asset else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::assets::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id.eq(asset).and(
                    crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                        .eq(&self.asset_model),
                ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
            .optional()
    }
    pub fn asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::codegen::structs_codegen::tables::assets::Asset>, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::assets::Asset:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(asset) = self.asset else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::assets::Asset::read(asset, conn).optional()
    }
    pub fn procedure_assets_asset_model_ancestor_model_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor::read(
            (self.asset_model, self.ancestor_model),
            conn,
        )
    }
    pub fn asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::read(
            self.asset_model,
            conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
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
    #[cfg(feature = "postgres")]
    pub fn procedure_assets_procedure_procedure_template_fkey(
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
    #[cfg(feature = "postgres")]
    pub fn procedure_assets_procedure_template_asset_model_ancestor_m_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_asset_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.ancestor_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_asset_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_asset_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_assets_procedure_template_asset_model_procedure_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_asset_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureAssetBuilder {
    pub(crate) id: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template: Option<i32>,
    pub(crate) asset_model: Option<i32>,
    pub(crate) asset: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_asset_model: Option<i32>,
    pub(crate) ancestor_model: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl From<InsertableProcedureAssetBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableProcedureAssetBuilder,
    >
{
    fn from(builder: InsertableProcedureAssetBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableProcedureAssetBuilder {
    fn default() -> Self {
        Self {
            id: Some(rosetta_uuid::Uuid::new_v4()),
            procedure: Default::default(),
            procedure_template: Default::default(),
            asset_model: Default::default(),
            asset: Default::default(),
            procedure_template_asset_model: Default::default(),
            ancestor_model: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder
{
    fn is_complete(&self) -> bool {
        self.id.is_some()
            && self.procedure.is_some()
            && (self.procedure_template.is_some()
                || self.procedure.is_some()
                || self.procedure_template_asset_model.is_some())
            && (self.asset_model.is_some() || self.asset.is_some())
            && self.procedure_template_asset_model.is_some()
            && (self.ancestor_model.is_some() || self.procedure_template_asset_model.is_some())
            && self.created_by.is_some()
            && self.created_at.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `ProcedureAsset` or
/// descendant tables.
pub trait ProcedureAssetSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.procedure_assets.id` column.
    ///
    /// # Arguments
    /// * `id`: The value to set for the `public.procedure_assets.id` column.
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
    fn id<I>(
        self,
        id: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the `public.procedure_assets.procedure` column.
    ///
    /// # Arguments
    /// * `procedure`: The value to set for the
    ///   `public.procedure_assets.procedure` column.
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
    fn procedure<P>(
        self,
        procedure: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the `public.procedure_assets.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.procedure_assets.procedure_template` column.
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
    /// Sets the value of the `public.procedure_assets.asset_model` column.
    ///
    /// # Arguments
    /// * `asset_model`: The value to set for the
    ///   `public.procedure_assets.asset_model` column.
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
    fn asset_model<AM>(
        self,
        asset_model: AM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedure_assets.asset` column.
    ///
    /// # Arguments
    /// * `asset`: The value to set for the `public.procedure_assets.asset`
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
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn asset<A>(
        self,
        asset: A,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        A: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.procedure_assets.procedure_template_asset_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_asset_model`: The value to set for the
    ///   `public.procedure_assets.procedure_template_asset_model` column.
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
    fn procedure_template_asset_model<PTAM>(
        self,
        procedure_template_asset_model: PTAM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedure_assets.ancestor_model` column.
    ///
    /// # Arguments
    /// * `ancestor_model`: The value to set for the
    ///   `public.procedure_assets.ancestor_model` column.
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
    fn ancestor_model<AM>(
        self,
        ancestor_model: AM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedure_assets.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.procedure_assets.created_by` column.
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
    fn created_by<CB>(
        self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedure_assets.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.procedure_assets.created_at` column.
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
impl ProcedureAssetSettable for InsertableProcedureAssetBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute;
    /// Sets the value of the `public.procedure_assets.id` column.
    fn id<I>(
        mut self,
        id: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let id = <I as web_common_traits::database::PrimaryKeyLike>::primary_key(&id);
        self.id = Some(id);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.procedure` column.
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
    /// v0@{shape: rounded, label: "procedure"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "procedure_template"}
    /// class v1 directly-involved-column
    /// v0 -.->|"`foreign defines`"| v1
    /// ```
    fn procedure<P>(
        mut self,
        procedure: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let procedure = <P as web_common_traits::database::PrimaryKeyLike>::primary_key(&procedure);
        self.procedure = Some(procedure);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.procedure_template`
    /// column.
    fn procedure_template<PT>(
        mut self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template =
            <PT as web_common_traits::database::PrimaryKeyLike>::primary_key(&procedure_template);
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.asset_model` column.
    fn asset_model<AM>(
        mut self,
        asset_model: AM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let asset_model =
            <AM as web_common_traits::database::PrimaryKeyLike>::primary_key(&asset_model);
        self.asset_model = Some(asset_model);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.asset` column.
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
    /// v0@{shape: rounded, label: "asset"}
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "asset_model"}
    /// class v1 directly-involved-column
    /// v0 -.->|"`foreign defines`"| v1
    /// ```
    fn asset<A>(
        mut self,
        asset: A,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        A: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let asset =
            <A as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(&asset);
        self.asset = asset;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.procedure_assets.procedure_template_asset_model` column.
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
    /// v0@{shape: rounded, label: "ancestor_model"}
    /// class v0 directly-involved-column
    /// v1@{shape: rounded, label: "procedure_template"}
    /// class v1 directly-involved-column
    /// v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 column-of-interest
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// ```
    fn procedure_template_asset_model<PTAM>(
        mut self,
        procedure_template_asset_model: PTAM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_asset_model =
            <PTAM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_asset_model,
            );
        self.procedure_template_asset_model = Some(procedure_template_asset_model);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.ancestor_model` column.
    fn ancestor_model<AM>(
        mut self,
        ancestor_model: AM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let ancestor_model =
            <AM as web_common_traits::database::PrimaryKeyLike>::primary_key(&ancestor_model);
        self.ancestor_model = Some(ancestor_model);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.created_by` column.
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by =
            <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(&created_by);
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.created_at` column.
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
                .rename_field(ProcedureAssetAttribute::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableProcedureAssetBuilder {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = Some(primary_key);
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableProcedureAssetBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            Error = web_common_traits::database::InsertError<ProcedureAssetAttribute>,
        >,
{
    type Attributes = ProcedureAssetAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
