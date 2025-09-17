#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetModelAncestorAttribute {
    DescendantModel,
    AncestorModel,
}
impl core::str::FromStr for AssetModelAncestorAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DescendantModel" => Ok(Self::DescendantModel),
            "AncestorModel" => Ok(Self::AncestorModel),
            "descendant_model" => Ok(Self::DescendantModel),
            "ancestor_model" => Ok(Self::AncestorModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder
{
    type Attribute = AssetModelAncestorAttribute;
}
impl core::fmt::Display for AssetModelAncestorAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::DescendantModel => write!(f, "asset_model_ancestors.descendant_model"),
            Self::AncestorModel => write!(f, "asset_model_ancestors.ancestor_model"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAssetModelAncestor {
    pub(crate) descendant_model: i32,
    pub(crate) ancestor_model: i32,
}
impl InsertableAssetModelAncestor {
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
    pub fn descendant_model<C: diesel::connection::LoadConnection>(
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
            self.descendant_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`AssetModelAncestor`](crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::AssetModelAncestor;
/// use core_structures::tables::insertables::AssetModelAncestorSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let asset_model_ancestor = AssetModelAncestor::new()
///    // Set mandatory fields
///    .ancestor_model(ancestor_model)?
///    .descendant_model(descendant_model)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableAssetModelAncestorBuilder {
    pub(crate) descendant_model: Option<i32>,
    pub(crate) ancestor_model: Option<i32>,
}
impl diesel::associations::HasTable for InsertableAssetModelAncestorBuilder {
    type Table =
        crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::table
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder
{
    fn is_complete(&self) -> bool {
        self.descendant_model.is_some() && self.ancestor_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `AssetModelAncestor`
/// or descendant tables.
pub trait AssetModelAncestorSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.asset_model_ancestors.descendant_model`
    /// column.
    ///
    /// # Arguments
    /// * `descendant_model`: The value to set for the
    ///   `public.asset_model_ancestors.descendant_model` column.
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
    fn descendant_model<DM>(self, descendant_model: DM) -> Result<Self, Self::Error>
    where
        DM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.asset_model_ancestors.ancestor_model`
    /// column.
    ///
    /// # Arguments
    /// * `ancestor_model`: The value to set for the
    ///   `public.asset_model_ancestors.ancestor_model` column.
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
    fn ancestor_model<AM>(self, ancestor_model: AM) -> Result<Self, Self::Error>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl AssetModelAncestorSettable for InsertableAssetModelAncestorBuilder
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::AssetModelAncestorAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.asset_model_ancestors.descendant_model` column.
    fn descendant_model<DM>(mut self, descendant_model: DM) -> Result<Self, Self::Error>
    where
        DM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let descendant_model = <DM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &descendant_model,
        );
        self.descendant_model = Some(descendant_model);
        Ok(self)
    }
    ///Sets the value of the `public.asset_model_ancestors.ancestor_model` column.
    fn ancestor_model<AM>(mut self, ancestor_model: AM) -> Result<Self, Self::Error>
    where
        AM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let ancestor_model = <AM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &ancestor_model,
        );
        self.ancestor_model = Some(ancestor_model);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableAssetModelAncestorBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableAssetModelAncestorBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        C,
        Row = crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
        Error = web_common_traits::database::InsertError<AssetModelAncestorAttribute>,
    >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<AssetModelAncestorAttribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
