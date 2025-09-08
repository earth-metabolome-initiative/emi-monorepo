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
            Self::Id => write!(f, "id"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::AssetModel => write!(f, "asset_model"),
            Self::Asset => write!(f, "asset"),
            Self::ProcedureTemplateAssetModel => {
                write!(f, "procedure_template_asset_model")
            }
            Self::AncestorModel => write!(f, "ancestor_model"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
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
    ) -> Result<crate::AssetModel, diesel::result::Error>
    where
        crate::AssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetModel::read(self.ancestor_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_assets_asset_asset_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::Asset>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(asset) = self.asset else {
            return Ok(None);
        };
        crate::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id.eq(asset).and(
                    crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                        .eq(&self.asset_model),
                ),
            )
            .first::<crate::Asset>(conn)
            .map(Some)
    }
    pub fn asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::Asset>, diesel::result::Error>
    where
        crate::Asset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(asset) = self.asset else {
            return Ok(None);
        };
        crate::Asset::read(asset, conn).map(Some)
    }
    pub fn procedure_assets_asset_model_ancestor_model_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetModelAncestor, diesel::result::Error>
    where
        crate::AssetModelAncestor: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetModelAncestor::read((self.asset_model, self.ancestor_model), conn)
    }
    pub fn asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetModel, diesel::result::Error>
    where
        crate::AssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetModel::read(self.asset_model, conn)
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::User, diesel::result::Error>
    where
        crate::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::User::read(self.created_by, conn)
    }
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
    #[cfg(feature = "postgres")]
    pub fn procedure_assets_procedure_procedure_template_fkey(
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
    pub fn procedure_assets_procedure_template_asset_model_ancestor_m_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_asset_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.ancestor_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
    pub fn procedure_template_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_asset_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_assets_procedure_template_asset_model_procedure_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_asset_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
    }
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
}
#[derive(Clone, Debug)]
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
    fn id(
        self,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn procedure(
        self,
        procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn asset_model(
        self,
        asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn asset(
        self,
        asset: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn procedure_template_asset_model(
        self,
        procedure_template_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn ancestor_model(
        self,
        ancestor_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn created_by(
        self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn id(
        mut self,
        id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let id = id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(ProcedureAssetAttribute::Id)
        })?;
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
    fn procedure(
        mut self,
        procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = Some(procedure);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.procedure_template`
    /// column.
    fn procedure_template(
        mut self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.asset_model` column.
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
    /// v1@{shape: rounded, label: "asset_model"}
    /// class v1 column-of-interest
    /// v1 -.->|"`foreign defines`"| v0
    /// v0 -.->|"`foreign defines`"| v1
    /// ```
    fn asset_model(
        mut self,
        asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
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
    fn asset(
        mut self,
        asset: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
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
    fn procedure_template_asset_model(
        mut self,
        procedure_template_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_asset_model = Some(procedure_template_asset_model);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.ancestor_model` column.
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
    /// class v0 column-of-interest
    /// v1@{shape: rounded, label: "asset_model"}
    /// class v1 directly-involved-column
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 -.->|"`foreign defines`"| v0
    /// ```
    fn ancestor_model(
        mut self,
        ancestor_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.ancestor_model = Some(ancestor_model);
        Ok(self)
    }
    /// Sets the value of the `public.procedure_assets.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
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
            Row = crate::ProcedureAsset,
            Error = web_common_traits::database::InsertError<ProcedureAssetAttribute>,
        >,
{
    type Attributes = ProcedureAssetAttribute;
    fn is_complete(&self) -> bool {
        self.id.is_some()
            && self.procedure.is_some()
            && self.procedure_template.is_some()
            && self.asset_model.is_some()
            && self.procedure_template_asset_model.is_some()
            && self.ancestor_model.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::ProcedureAsset = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
