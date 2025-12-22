#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetCompatibilityRuleAttribute {
    LeftAssetModel,
    RightAssetModel,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for AssetCompatibilityRuleAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LeftAssetModel" => Ok(Self::LeftAssetModel),
            "RightAssetModel" => Ok(Self::RightAssetModel),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "left_asset_model" => Ok(Self::LeftAssetModel),
            "right_asset_model" => Ok(Self::RightAssetModel),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableAssetCompatibilityRuleBuilder {
    type Attribute = AssetCompatibilityRuleAttribute;
}
impl web_common_traits::database::TableField for AssetCompatibilityRuleAttribute {}
impl web_common_traits::database::HasTableType for AssetCompatibilityRuleAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl core::fmt::Display for AssetCompatibilityRuleAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::LeftAssetModel => {
                write!(f, "asset_compatibility_rules.left_asset_model")
            }
            Self::RightAssetModel => {
                write!(f, "asset_compatibility_rules.right_asset_model")
            }
            Self::CreatedBy => write!(f, "asset_compatibility_rules.created_by"),
            Self::CreatedAt => write!(f, "asset_compatibility_rules.created_at"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAssetCompatibilityRule {
    pub(crate) left_asset_model: i32,
    pub(crate) right_asset_model: i32,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableAssetCompatibilityRule {
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
    pub fn left_asset_model<C: diesel::connection::LoadConnection>(
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
            self.left_asset_model_id,
            conn,
        )
    }
    pub fn right_asset_model<C: diesel::connection::LoadConnection>(
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
            self.right_asset_model_id,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`AssetCompatibilityRule`](crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::AssetCompatibilityRule;
/// use core_structures::tables::insertables::AssetCompatibilityRuleSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let asset_compatibility_rule = AssetCompatibilityRule::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .left_asset_model(left_asset_model_id)?
///    .right_asset_model(right_asset_model_id)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableAssetCompatibilityRuleBuilder {
    pub(crate) left_asset_model: Option<i32>,
    pub(crate) right_asset_model: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl diesel::associations::HasTable for InsertableAssetCompatibilityRuleBuilder {
    type Table = crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::table
    }
}
impl Default for InsertableAssetCompatibilityRuleBuilder {
    fn default() -> Self {
        Self {
            left_asset_model: Default::default(),
            right_asset_model: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableAssetCompatibilityRuleBuilder {
    fn is_complete(&self) -> bool {
        self.left_asset_model.is_some() && self.right_asset_model.is_some()
            && self.created_by.is_some() && self.created_at.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `AssetCompatibilityRule` or descendant tables.
pub trait AssetCompatibilityRuleSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the
    /// `public.asset_compatibility_rules.left_asset_model` column.
    ///
    /// # Arguments
    /// * `left_asset_model`: The value to set for the
    ///   `public.asset_compatibility_rules.left_asset_model` column.
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
    fn left_asset_model<LAM>(self, left_asset_model: LAM) -> Result<Self, Self::Error>
    where
        LAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.asset_compatibility_rules.right_asset_model` column.
    ///
    /// # Arguments
    /// * `right_asset_model`: The value to set for the
    ///   `public.asset_compatibility_rules.right_asset_model` column.
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
    fn right_asset_model<RAM>(self, right_asset_model: RAM) -> Result<Self, Self::Error>
    where
        RAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.asset_compatibility_rules.created_by`
    /// column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.asset_compatibility_rules.created_by` column.
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
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.asset_compatibility_rules.created_at`
    /// column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.asset_compatibility_rules.created_at` column.
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
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
}
impl AssetCompatibilityRuleSettable for InsertableAssetCompatibilityRuleBuilder
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.asset_compatibility_rules.left_asset_model` column.
    fn left_asset_model<LAM>(
        mut self,
        left_asset_model: LAM,
    ) -> Result<Self, Self::Error>
    where
        LAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let left_asset_model_id = <LAM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &left_asset_model_id,
        );
        if let Some(right_asset_model_id) = self.right_asset_model_id {
            pgrx_validation::must_be_distinct_i32(left_asset_model_id, right_asset_model_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::LeftAssetModel,
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::RightAssetModel,
                        )
                })?;
        }
        self.left_asset_model_id = Some(left_asset_model_id);
        Ok(self)
    }
    ///Sets the value of the `public.asset_compatibility_rules.right_asset_model` column.
    fn right_asset_model<RAM>(
        mut self,
        right_asset_model: RAM,
    ) -> Result<Self, Self::Error>
    where
        RAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let right_asset_model_id = <RAM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &right_asset_model_id,
        );
        if let Some(left_asset_model_id) = self.left_asset_model_id {
            pgrx_validation::must_be_distinct_i32(left_asset_model_id, right_asset_model_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::LeftAssetModel,
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::RightAssetModel,
                        )
                })?;
        }
        self.right_asset_model_id = Some(right_asset_model_id);
        Ok(self)
    }
    ///Sets the value of the `public.asset_compatibility_rules.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by = <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &created_by,
        );
        self.created_by = Some(created_by);
        Ok(self)
    }
    ///Sets the value of the `public.asset_compatibility_rules.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        let created_at = created_at
            .try_into()
            .map_err(|err| {
                validation_errors::prelude::SingleFieldError::from(err)
                    .rename_field(AssetCompatibilityRuleAttribute::CreatedAt)
            })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableAssetCompatibilityRuleBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableAssetCompatibilityRuleBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
            Error = web_common_traits::database::InsertError<
                AssetCompatibilityRuleAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = (i32, i32)>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        AssetCompatibilityRuleAttribute,
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
