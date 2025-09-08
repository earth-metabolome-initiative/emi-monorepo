#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StorageProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for StorageProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for StorageProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StorageProcedureAttribute {
    Extension(StorageProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    StoredAsset,
    StoredAssetModel,
    ProcedureTemplateStoredAssetModel,
    ProcedureStoredAsset(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    StoredInto,
    StoredIntoModel,
    ProcedureTemplateStoredIntoModel,
    ProcedureStoredInto(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for StorageProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "StoredAsset" => Ok(Self::StoredAsset),
            "StoredAssetModel" => Ok(Self::StoredAssetModel),
            "ProcedureTemplateStoredAssetModel" => Ok(Self::ProcedureTemplateStoredAssetModel),
            "ProcedureStoredAsset" => Ok(Self::ProcedureStoredAsset(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "StoredInto" => Ok(Self::StoredInto),
            "StoredIntoModel" => Ok(Self::StoredIntoModel),
            "ProcedureTemplateStoredIntoModel" => Ok(Self::ProcedureTemplateStoredIntoModel),
            "ProcedureStoredInto" => Ok(Self::ProcedureStoredInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "stored_asset" => Ok(Self::StoredAsset),
            "stored_asset_model" => Ok(Self::StoredAssetModel),
            "procedure_template_stored_asset_model" => Ok(Self::ProcedureTemplateStoredAssetModel),
            "procedure_stored_asset" => Ok(Self::ProcedureStoredAsset(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "stored_into" => Ok(Self::StoredInto),
            "stored_into_model" => Ok(Self::StoredIntoModel),
            "procedure_template_stored_into_model" => Ok(Self::ProcedureTemplateStoredIntoModel),
            "procedure_stored_into" => Ok(Self::ProcedureStoredInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for StorageProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "procedure"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::StoredAsset => write!(f, "stored_asset"),
            Self::StoredAssetModel => write!(f, "stored_asset_model"),
            Self::ProcedureTemplateStoredAssetModel => {
                write!(f, "procedure_template_stored_asset_model")
            }
            Self::ProcedureStoredAsset(e) => write!(f, "{e}"),
            Self::StoredInto => write!(f, "stored_into"),
            Self::StoredIntoModel => write!(f, "stored_into_model"),
            Self::ProcedureTemplateStoredIntoModel => {
                write!(f, "procedure_template_stored_into_model")
            }
            Self::ProcedureStoredInto(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStorageProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) stored_asset: ::rosetta_uuid::Uuid,
    pub(crate) stored_asset_model: i32,
    pub(crate) procedure_template_stored_asset_model: i32,
    pub(crate) procedure_stored_asset: ::rosetta_uuid::Uuid,
    pub(crate) stored_into: ::rosetta_uuid::Uuid,
    pub(crate) stored_into_model: i32,
    pub(crate) procedure_template_stored_into_model: i32,
    pub(crate) procedure_stored_into: ::rosetta_uuid::Uuid,
}
impl InsertableStorageProcedure {
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
    pub fn storage_procedures_procedure_procedure_template_fkey(
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
    pub fn procedure_stored_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_stored_asset, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_procedure_templa_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_stored_asset_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_stored_asset_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.stored_asset),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_asset_stored_asset_mod_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.stored_asset_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_stored_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_stored_into, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_stored_into_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_stored_into_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.stored_into),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_stored_into_stored_into_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_stored_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.stored_into_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::StorageProcedureTemplate, diesel::result::Error>
    where
        crate::StorageProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::StorageProcedureTemplate::read(self.procedure_template, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_template_procedure_template_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::StorageProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::StorageProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template_stored_into_model
                            .eq(&self.procedure_template_stored_into_model),
                    ),
            )
            .first::<crate::StorageProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn storage_procedures_procedure_template_procedure_template_s_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::StorageProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::StorageProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::storage_procedure_templates::storage_procedure_templates::dsl::procedure_template_stored_asset_model
                            .eq(&self.procedure_template_stored_asset_model),
                    ),
            )
            .first::<crate::StorageProcedureTemplate>(conn)
    }
    pub fn procedure_template_stored_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_stored_asset_model, conn)
    }
    pub fn procedure_template_stored_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_stored_into_model, conn)
    }
    pub fn stored_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::PhysicalAsset, diesel::result::Error>
    where
        crate::PhysicalAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::PhysicalAsset::read(self.stored_asset, conn)
    }
    pub fn stored_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::PhysicalAssetModel, diesel::result::Error>
    where
        crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::PhysicalAssetModel::read(self.stored_asset_model, conn)
    }
    pub fn stored_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Container, diesel::result::Error>
    where
        crate::Container: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Container::read(self.stored_into, conn)
    }
    pub fn stored_into_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ContainerModel, diesel::result::Error>
    where
        crate::ContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ContainerModel::read(self.stored_into_model, conn)
    }
    pub fn storage_procedures_stored_into_model_stored_asset_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::ContainerCompatibilityRule, diesel::result::Error>
    where
        crate::ContainerCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ContainerCompatibilityRule::read(
            (self.stored_into_model, self.stored_asset_model),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStorageProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) stored_asset: Option<::rosetta_uuid::Uuid>,
    pub(crate) stored_asset_model: Option<i32>,
    pub(crate) procedure_template_stored_asset_model: Option<i32>,
    pub(crate) procedure_stored_asset: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) stored_into: Option<::rosetta_uuid::Uuid>,
    pub(crate) stored_into_model: Option<i32>,
    pub(crate) procedure_template_stored_into_model: Option<i32>,
    pub(crate) procedure_stored_into: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl From<InsertableStorageProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableStorageProcedureBuilder,
    >
{
    fn from(builder: InsertableStorageProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
/// Trait defining setters for attributes of an instance of `StorageProcedure`
/// or descendant tables.
pub trait StorageProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.storage_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.storage_procedures.procedure_template` column.
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
    /// Sets the value of the `public.storage_procedures.stored_asset` column.
    ///
    /// # Arguments
    /// * `stored_asset`: The value to set for the
    ///   `public.storage_procedures.stored_asset` column.
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
    fn stored_asset(
        self,
        stored_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.storage_procedures.stored_asset_model`
    /// column.
    ///
    /// # Arguments
    /// * `stored_asset_model`: The value to set for the
    ///   `public.storage_procedures.stored_asset_model` column.
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
    fn stored_asset_model(
        self,
        stored_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.storage_procedures.procedure_template_stored_asset_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_stored_asset_model`: The value to set for the
    ///   `public.storage_procedures.procedure_template_stored_asset_model`
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
    fn procedure_template_stored_asset_model(
        self,
        procedure_template_stored_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.storage_procedures.procedure_stored_asset`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_stored_asset`: The value to set for the
    ///   `public.storage_procedures.procedure_stored_asset` column.
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
    fn procedure_stored_asset<PSA>(
        self,
        procedure_stored_asset: PSA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.storage_procedures.stored_into` column.
    ///
    /// # Arguments
    /// * `stored_into`: The value to set for the
    ///   `public.storage_procedures.stored_into` column.
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
    fn stored_into(
        self,
        stored_into: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.storage_procedures.stored_into_model`
    /// column.
    ///
    /// # Arguments
    /// * `stored_into_model`: The value to set for the
    ///   `public.storage_procedures.stored_into_model` column.
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
    fn stored_into_model(
        self,
        stored_into_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.storage_procedures.procedure_template_stored_into_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_stored_into_model`: The value to set for the
    ///   `public.storage_procedures.procedure_template_stored_into_model`
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
    fn procedure_template_stored_into_model(
        self,
        procedure_template_stored_into_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.storage_procedures.procedure_stored_into`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_stored_into`: The value to set for the
    ///   `public.storage_procedures.procedure_stored_into` column.
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
    fn procedure_stored_into<PSI>(
        self,
        procedure_stored_into: PSI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSI: Into<
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
> StorageProcedureSettable for InsertableStorageProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute;
    /// Sets the value of the `public.storage_procedures.procedure_template`
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
    /// subgraph v5 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 undirectly-involved-column
    /// end
    /// subgraph v6 ["`procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v7 ["`storage_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_stored_asset_model"}
    /// class v2 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template"}
    /// class v1 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_template_stored_into_model"}
    /// class v3 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v4
    /// v1 --->|"`ancestral same as`"| v0
    /// v1 -.->|"`foreign defines`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v3 --->|"`associated same as`"| v4
    /// v7 --->|"`extends`"| v6
    /// v7 ---o|"`associated with`"| v5
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
    /// Sets the value of the `public.storage_procedures.stored_asset` column.
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
    /// subgraph v4 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`storage_procedures`"]
    ///    v2@{shape: rounded, label: "stored_asset"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_stored_asset"}
    /// class v1 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v0
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn stored_asset(
        mut self,
        stored_asset: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stored_asset) =
            self.procedure_stored_asset
        {
            self.procedure_stored_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_stored_asset,
                    Some(stored_asset),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStoredAsset(attribute)
                    })
                })?
                .into();
        }
        self.stored_asset = Some(stored_asset);
        Ok(self)
    }
    /// Sets the value of the `public.storage_procedures.stored_asset_model`
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
    /// subgraph v5 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "id"}
    /// class v4 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v6 ["`storage_procedures`"]
    ///    v3@{shape: rounded, label: "stored_into_model"}
    /// class v3 directly-involved-column
    ///    v2@{shape: rounded, label: "stored_asset_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_stored_asset"}
    /// class v1 directly-involved-column
    /// end
    /// v3 --->|"`associated same as`"| v0
    /// v3 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v2 -.->|"`foreign defines`"| v3
    /// v1 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v2
    /// v6 ---o|"`associated with`"| v5
    /// ```
    fn stored_asset_model(
        mut self,
        stored_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stored_asset) =
            self.procedure_stored_asset
        {
            self.procedure_stored_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_stored_asset,
                    stored_asset_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStoredAsset(attribute)
                    })
                })?
                .into();
        }
        self.stored_asset_model = Some(stored_asset_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.storage_procedures.procedure_template_stored_asset_model`
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
    /// subgraph v4 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`storage_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_stored_asset_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_stored_asset"}
    /// class v1 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v0
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_stored_asset_model(
        mut self,
        procedure_template_stored_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stored_asset) =
            self.procedure_stored_asset
        {
            self.procedure_stored_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_stored_asset,
                    procedure_template_stored_asset_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStoredAsset(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_stored_asset_model = Some(procedure_template_stored_asset_model);
        Ok(self)
    }
    /// Sets the value of the `public.storage_procedures.procedure_stored_asset`
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
    /// subgraph v8 ["`procedure_assets`"]
    ///    v7@{shape: rounded, label: "id"}
    /// class v7 undirectly-involved-column
    ///    v1@{shape: rounded, label: "asset_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v9 ["`storage_procedures`"]
    ///    v3@{shape: rounded, label: "procedure_stored_asset"}
    /// class v3 column-of-interest
    ///    v5@{shape: rounded, label: "stored_asset"}
    /// class v5 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_stored_asset_model"}
    /// class v4 directly-involved-column
    ///    v6@{shape: rounded, label: "stored_asset_model"}
    /// class v6 directly-involved-column
    /// end
    /// v3 --->|"`associated same as`"| v7
    /// v3 --->|"`associated same as`"| v7
    /// v3 --->|"`associated same as`"| v7
    /// v3 --->|"`associated same as`"| v7
    /// v3 -.->|"`foreign defines`"| v4
    /// v3 -.->|"`foreign defines`"| v5
    /// v3 -.->|"`foreign defines`"| v6
    /// v5 --->|"`associated same as`"| v0
    /// v0 -.->|"`foreign defines`"| v1
    /// v4 --->|"`associated same as`"| v2
    /// v6 --->|"`associated same as`"| v1
    /// v9 ---o|"`associated with`"| v8
    /// ```
    fn procedure_stored_asset<PSA>(
        mut self,
        procedure_stored_asset: PSA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSA: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_stored_asset = procedure_stored_asset.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_stored_asset {
            procedure_stored_asset = if let (
                Some(procedure_template_stored_asset_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_stored_asset_model, builder.procedure_template_asset_model)
            {
                if procedure_template_stored_asset_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateStoredAssetModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_stored_asset_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_stored_asset_model) =
                self.procedure_template_stored_asset_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_stored_asset_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStoredAsset(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_stored_asset {
            procedure_stored_asset = if let (Some(stored_asset), Some(asset)) =
                (self.stored_asset, builder.asset)
            {
                if stored_asset != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::StoredAsset,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.stored_asset = Some(asset);
                builder.into()
            } else if let Some(stored_asset) = self.stored_asset {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(stored_asset),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStoredAsset(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_stored_asset {
            procedure_stored_asset = if let (Some(stored_asset_model), Some(asset_model)) =
                (self.stored_asset_model, builder.asset_model)
            {
                if stored_asset_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::StoredAssetModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.stored_asset_model = Some(asset_model);
                builder.into()
            } else if let Some(stored_asset_model) = self.stored_asset_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        stored_asset_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStoredAsset(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_stored_asset = procedure_stored_asset;
        Ok(self)
    }
    /// Sets the value of the `public.storage_procedures.stored_into` column.
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
    /// subgraph v4 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`storage_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_stored_into"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "stored_into"}
    /// class v2 column-of-interest
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn stored_into(
        mut self,
        stored_into: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stored_into) =
            self.procedure_stored_into
        {
            self.procedure_stored_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_stored_into,
                    Some(stored_into),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStoredInto(attribute)
                    })
                })?
                .into();
        }
        self.stored_into = Some(stored_into);
        Ok(self)
    }
    /// Sets the value of the `public.storage_procedures.stored_into_model`
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
    /// subgraph v5 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "id"}
    /// class v4 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v6 ["`storage_procedures`"]
    ///    v3@{shape: rounded, label: "stored_into_model"}
    /// class v3 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_stored_into"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "stored_asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v3 --->|"`associated same as`"| v0
    /// v3 -.->|"`foreign defines`"| v2
    /// v1 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v4
    /// v1 -.->|"`foreign defines`"| v3
    /// v2 --->|"`associated same as`"| v0
    /// v2 -.->|"`foreign defines`"| v3
    /// v6 ---o|"`associated with`"| v5
    /// ```
    fn stored_into_model(
        mut self,
        stored_into_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stored_into) =
            self.procedure_stored_into
        {
            self.procedure_stored_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_stored_into,
                    stored_into_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStoredInto(attribute)
                    })
                })?
                .into();
        }
        self.stored_into_model = Some(stored_into_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.storage_procedures.procedure_template_stored_into_model` column.
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
    /// subgraph v4 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`storage_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_stored_into_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_stored_into"}
    /// class v1 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v0
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_stored_into_model(
        mut self,
        procedure_template_stored_into_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stored_into) =
            self.procedure_stored_into
        {
            self.procedure_stored_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_stored_into,
                    procedure_template_stored_into_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStoredInto(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_stored_into_model = Some(procedure_template_stored_into_model);
        Ok(self)
    }
    /// Sets the value of the `public.storage_procedures.procedure_stored_into`
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
    /// subgraph v8 ["`procedure_assets`"]
    ///    v7@{shape: rounded, label: "id"}
    /// class v7 undirectly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "asset_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v9 ["`storage_procedures`"]
    ///    v6@{shape: rounded, label: "stored_into_model"}
    /// class v6 directly-involved-column
    ///    v5@{shape: rounded, label: "stored_into"}
    /// class v5 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_stored_into"}
    /// class v3 column-of-interest
    ///    v4@{shape: rounded, label: "procedure_template_stored_into_model"}
    /// class v4 directly-involved-column
    /// end
    /// v6 --->|"`associated same as`"| v1
    /// v5 --->|"`associated same as`"| v0
    /// v3 --->|"`associated same as`"| v7
    /// v3 --->|"`associated same as`"| v7
    /// v3 --->|"`associated same as`"| v7
    /// v3 --->|"`associated same as`"| v7
    /// v3 -.->|"`foreign defines`"| v4
    /// v3 -.->|"`foreign defines`"| v5
    /// v3 -.->|"`foreign defines`"| v6
    /// v0 -.->|"`foreign defines`"| v1
    /// v4 --->|"`associated same as`"| v2
    /// v9 ---o|"`associated with`"| v8
    /// ```
    fn procedure_stored_into<PSI>(
        mut self,
        procedure_stored_into: PSI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSI: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_stored_into = procedure_stored_into.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_stored_into {
            procedure_stored_into = if let (
                Some(procedure_template_stored_into_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_stored_into_model, builder.procedure_template_asset_model)
            {
                if procedure_template_stored_into_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateStoredIntoModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_stored_into_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_stored_into_model) =
                self.procedure_template_stored_into_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_stored_into_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStoredInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_stored_into {
            procedure_stored_into = if let (Some(stored_into), Some(asset)) =
                (self.stored_into, builder.asset)
            {
                if stored_into != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::StoredInto,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.stored_into = Some(asset);
                builder.into()
            } else if let Some(stored_into) = self.stored_into {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(stored_into),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStoredInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_stored_into {
            procedure_stored_into = if let (Some(stored_into_model), Some(asset_model)) =
                (self.stored_into_model, builder.asset_model)
            {
                if stored_into_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::StoredIntoModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.stored_into_model = Some(asset_model);
                builder.into()
            } else if let Some(stored_into_model) = self.stored_into_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        stored_into_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStoredInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_stored_into = procedure_stored_into;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableStorageProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute;
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
    ///subgraph v2 ["`procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`storage_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as StorageProcedureSettable>::procedure_template(self, procedure_template)
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
    for InsertableStorageProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableStorageProcedureBuilder<Procedure>
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
    for InsertableStorageProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::StorageProcedure,
            Error = web_common_traits::database::InsertError<StorageProcedureAttribute>,
        >,
    Procedure: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        web_common_traits::database::TryInsertGeneric<C>,
{
    type Attributes = StorageProcedureAttribute;
    fn is_complete(&self) -> bool {
        self.procedure.is_complete()
            && self.procedure_template.is_some()
            && self.stored_asset.is_some()
            && self.stored_asset_model.is_some()
            && self.procedure_template_stored_asset_model.is_some()
            && self.procedure_stored_asset.is_complete()
            && self.stored_into.is_some()
            && self.stored_into_model.is_some()
            && self.procedure_template_stored_into_model.is_some()
            && self.procedure_stored_into.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::StorageProcedure = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
