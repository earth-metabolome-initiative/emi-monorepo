#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProcedureTemplateAssetModelAttribute {
    Id,
    Name,
    ProcedureTemplate,
    BasedOn,
    AssetModel,
}
impl core::str::FromStr for ProcedureTemplateAssetModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Name" => Ok(Self::Name),
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "BasedOn" => Ok(Self::BasedOn),
            "AssetModel" => Ok(Self::AssetModel),
            "name" => Ok(Self::Name),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "based_on" => Ok(Self::BasedOn),
            "asset_model" => Ok(Self::AssetModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder {
    type Attribute = ProcedureTemplateAssetModelAttribute;
}
impl web_common_traits::database::TableField for ProcedureTemplateAssetModelAttribute {}
impl web_common_traits::database::HasTableType for ProcedureTemplateAssetModelAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl core::fmt::Display for ProcedureTemplateAssetModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "procedure_template_asset_models.id"),
            Self::Name => write!(f, "procedure_template_asset_models.name"),
            Self::ProcedureTemplate => {
                write!(f, "procedure_template_asset_models.procedure_template")
            }
            Self::BasedOn => write!(f, "procedure_template_asset_models.based_on"),
            Self::AssetModel => write!(f, "procedure_template_asset_models.asset_model"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureTemplateAssetModel {
    pub(crate) name: String,
    pub(crate) procedure_template: i32,
    pub(crate) based_on: Option<i32>,
    pub(crate) asset_model: i32,
}
impl InsertableProcedureTemplateAssetModel {
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
    #[cfg(feature = "postgres")]
    pub fn procedure_template_asset_models_based_on_asset_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(based_on) = self.based_on else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(based_on)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
            .optional()
    }
    pub fn based_on<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(based_on) = self.based_on else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                based_on,
                conn,
            )
            .optional()
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
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`ProcedureTemplateAssetModel`](crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::ProcedureTemplateAssetModel;
/// use core_structures::tables::insertables::ProcedureTemplateAssetModelSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let procedure_template_asset_model = ProcedureTemplateAssetModel::new()
///    // Set mandatory fields
///    .name(name)?
///    .procedure_template(procedure_template)?
///    // Optionally set optional fields
///    .based_on(based_on)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableProcedureTemplateAssetModelBuilder {
    pub(crate) name: Option<String>,
    pub(crate) procedure_template: Option<i32>,
    pub(crate) based_on: Option<i32>,
    pub(crate) asset_model: Option<i32>,
}
impl diesel::associations::HasTable for InsertableProcedureTemplateAssetModelBuilder {
    type Table = crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::table
    }
}
impl From<InsertableProcedureTemplateAssetModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableProcedureTemplateAssetModelBuilder>
{
    fn from(builder: InsertableProcedureTemplateAssetModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder {
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.procedure_template.is_some()
            && (self.asset_model.is_some() || self.based_on.is_some())
    }
}
/// Trait defining setters for attributes of an instance of
/// `ProcedureTemplateAssetModel` or descendant tables.
pub trait ProcedureTemplateAssetModelSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.procedure_template_asset_models.name`
    /// column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the
    ///   `public.procedure_template_asset_models.name` column.
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
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the
    /// `public.procedure_template_asset_models.procedure_template` column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.procedure_template_asset_models.procedure_template` column.
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
    fn procedure_template<PT>(self, procedure_template: PT) -> Result<Self, Self::Error>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.procedure_template_asset_models.based_on`
    /// column.
    ///
    /// # Arguments
    /// * `based_on`: The value to set for the
    ///   `public.procedure_template_asset_models.based_on` column.
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
    fn based_on<BO>(self, based_on: BO) -> Result<Self, Self::Error>
    where
        BO: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.procedure_template_asset_models.asset_model` column.
    ///
    /// # Arguments
    /// * `asset_model`: The value to set for the
    ///   `public.procedure_template_asset_models.asset_model` column.
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
    fn asset_model<AM>(self, asset_model: AM) -> Result<Self, Self::Error>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl ProcedureTemplateAssetModelSettable for InsertableProcedureTemplateAssetModelBuilder
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.procedure_template_asset_models.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(ProcedureTemplateAssetModelAttribute::Name)
            })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Name,
                    )
            })?;
        self.name = Some(name);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_template_asset_models.procedure_template` column.
    fn procedure_template<PT>(
        mut self,
        procedure_template: PT,
    ) -> Result<Self, Self::Error>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template = <PT as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &procedure_template,
        );
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    ///Sets the value of the `public.procedure_template_asset_models.based_on` column.
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
    ///v0@{shape: rounded, label: "asset_model"}
    ///class v0 directly-involved-column
    ///v1@{shape: rounded, label: "based_on"}
    ///class v1 column-of-interest
    ///v1 -.->|"`foreign defines`"| v0
    ///```
    fn based_on<BO>(mut self, based_on: BO) -> Result<Self, Self::Error>
    where
        BO: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        let based_on = <BO as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
            &based_on,
        );
        self.based_on = based_on;
        Ok(self)
    }
    ///Sets the value of the `public.procedure_template_asset_models.asset_model` column.
    fn asset_model<AM>(mut self, asset_model: AM) -> Result<Self, Self::Error>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let asset_model = <AM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &asset_model,
        );
        self.asset_model = Some(asset_model);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableProcedureTemplateAssetModelBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableProcedureTemplateAssetModelBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            Error = web_common_traits::database::InsertError<
                ProcedureTemplateAssetModelAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        ProcedureTemplateAssetModelAttribute,
    >;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
