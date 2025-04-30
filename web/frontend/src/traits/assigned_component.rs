//! Submodule defining a trait for a component which is mapped to a specific
//! database table.

use core_structures::tables::table_names::TableName;
use web_common_traits::prelude::{Row, StaticTabular, Tabular};
use yew::BaseComponent;

use crate::utils::AssignedConnectorProps;

/// This trait is used to define components that are associated with a specific
/// database table.
pub trait AssignedComponent:
    BaseComponent<Properties = AssignedConnectorProps<<Self as AssignedComponent>::Row>>
{
    /// The assigned row type for the component.
    type Row: Row + StaticTabular + Tabular<TableName = TableName>;
}
