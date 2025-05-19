#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod as_ref;
pub mod diesel_impls;
mod display;
pub mod errors;
mod try_from;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(
	feature = "diesel",
	diesel(sql_type = crate::diesel_impls::NameplateCategory)
)]
/// Enum representing the tool categories.
pub enum NameplateCategory {
    /// An Aluminium nameplate with a brushed finish and adhesive backing.
    /// It is used for labeling specimens such as trees.
    /// Placed at human height and nailed to the trunk.
    Permanent,
}

impl NameplateCategory {
    /// Returns the name of the nameplate category.
    pub fn name(&self) -> &'static str {
        match self {
            NameplateCategory::Permanent => "Permanent",
        }
    }

    /// Returns the description of the nameplate category.
    pub fn description(&self) -> &'static str {
        match self {
            NameplateCategory::Permanent => {
                "An Aluminium nameplate with a brushed finish and adhesive backing. \
                It is used for labeling specimens such as trees. Placed at human height \
                and nailed to the trunk."
            }
        }
    }

    /// Returns the icon of the nameplate category.
    pub fn icon(&self) -> &'static str {
        match self {
            NameplateCategory::Permanent => "tag",
        }
    }
}
