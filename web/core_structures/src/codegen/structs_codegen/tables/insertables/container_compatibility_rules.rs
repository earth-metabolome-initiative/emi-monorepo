#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContainerCompatibilityRuleAttribute {
    ContainerModel,
    ContainedAssetModel,
    Quantity,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for ContainerCompatibilityRuleAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ContainerModel" => Ok(Self::ContainerModel),
            "ContainedAssetModel" => Ok(Self::ContainedAssetModel),
            "Quantity" => Ok(Self::Quantity),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "container_model" => Ok(Self::ContainerModel),
            "contained_asset_model" => Ok(Self::ContainedAssetModel),
            "quantity" => Ok(Self::Quantity),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for ContainerCompatibilityRuleAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ContainerModel => write!(f, "container_model"),
            Self::ContainedAssetModel => write!(f, "contained_asset_model"),
            Self::Quantity => write!(f, "quantity"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainerCompatibilityRule {
    pub(crate) container_model: i32,
    pub(crate) contained_asset_model: i32,
    pub(crate) quantity: Option<i16>,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableContainerCompatibilityRule {
    pub fn container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ContainerModel, diesel::result::Error>
    where
        crate::ContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ContainerModel::read(self.container_model, conn)
    }
    pub fn contained_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::PhysicalAssetModel, diesel::result::Error>
    where
        crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::PhysicalAssetModel::read(self.contained_asset_model, conn)
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
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableContainerCompatibilityRuleBuilder {
    pub(crate) container_model: Option<i32>,
    pub(crate) contained_asset_model: Option<i32>,
    pub(crate) quantity: Option<i16>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableContainerCompatibilityRuleBuilder {
    fn default() -> Self {
        Self {
            container_model: Default::default(),
            contained_asset_model: Default::default(),
            quantity: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `ContainerCompatibilityRule` or descendant tables.
pub trait ContainerCompatibilityRuleSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.container_compatibility_rules.container_model` column.
    ///
    /// # Arguments
    /// * `container_model`: The value to set for the
    ///   `public.container_compatibility_rules.container_model` column.
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
    fn container_model(
        self,
        container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.container_compatibility_rules.contained_asset_model` column.
    ///
    /// # Arguments
    /// * `contained_asset_model`: The value to set for the
    ///   `public.container_compatibility_rules.contained_asset_model` column.
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
    fn contained_asset_model(
        self,
        contained_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.container_compatibility_rules.quantity`
    /// column.
    ///
    /// # Arguments
    /// * `quantity`: The value to set for the
    ///   `public.container_compatibility_rules.quantity` column.
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
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn quantity<Q>(
        self,
        quantity: Q,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        Q: TryInto<Option<i16>>,
        validation_errors::SingleFieldError: From<<Q as TryInto<Option<i16>>>::Error>;
    /// Sets the value of the `public.container_compatibility_rules.created_by`
    /// column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.container_compatibility_rules.created_by` column.
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
    /// Sets the value of the `public.container_compatibility_rules.created_at`
    /// column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.container_compatibility_rules.created_at` column.
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
impl ContainerCompatibilityRuleSettable for InsertableContainerCompatibilityRuleBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::ContainerCompatibilityRuleAttribute;
    /// Sets the value of the
    /// `public.container_compatibility_rules.container_model` column.
    fn container_model(
        mut self,
        container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(contained_asset_model) = self.contained_asset_model {
            pgrx_validation::must_be_distinct_i32(container_model, contained_asset_model)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ContainerCompatibilityRuleAttribute::ContainerModel,
                            crate::codegen::structs_codegen::tables::insertables::ContainerCompatibilityRuleAttribute::ContainedAssetModel,
                        )
                })?;
        }
        self.container_model = Some(container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.container_compatibility_rules.contained_asset_model` column.
    fn contained_asset_model(
        mut self,
        contained_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(container_model) = self.container_model {
            pgrx_validation::must_be_distinct_i32(container_model, contained_asset_model)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::ContainerCompatibilityRuleAttribute::ContainerModel,
                            crate::codegen::structs_codegen::tables::insertables::ContainerCompatibilityRuleAttribute::ContainedAssetModel,
                        )
                })?;
        }
        self.contained_asset_model = Some(contained_asset_model);
        Ok(self)
    }
    /// Sets the value of the `public.container_compatibility_rules.quantity`
    /// column.
    fn quantity<Q>(
        mut self,
        quantity: Q,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        Q: TryInto<Option<i16>>,
        validation_errors::SingleFieldError: From<<Q as TryInto<Option<i16>>>::Error>,
    {
        let quantity = quantity.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(ContainerCompatibilityRuleAttribute::Quantity)
        })?;
        if let Some(quantity) = quantity {
            pgrx_validation::must_be_strictly_positive_i16(quantity)
                .map_err(|e| {
                    e
                        .rename_field(
                            crate::codegen::structs_codegen::tables::insertables::ContainerCompatibilityRuleAttribute::Quantity,
                        )
                })?;
        }
        self.quantity = quantity;
        Ok(self)
    }
    /// Sets the value of the `public.container_compatibility_rules.created_by`
    /// column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.container_compatibility_rules.created_at`
    /// column.
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
                .rename_field(ContainerCompatibilityRuleAttribute::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableContainerCompatibilityRuleBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableContainerCompatibilityRuleBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::ContainerCompatibilityRule,
            Error = web_common_traits::database::InsertError<ContainerCompatibilityRuleAttribute>,
        >,
{
    type Attributes = ContainerCompatibilityRuleAttribute;
    fn is_complete(&self) -> bool {
        self.container_model.is_some()
            && self.contained_asset_model.is_some()
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
        let insertable: crate::ContainerCompatibilityRule = self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
