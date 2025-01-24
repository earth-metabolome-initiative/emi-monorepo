//! Submodule providing a trait to iterate across the taxons in a topological order.

/// Trait to iterate across the taxons in a topological order.
pub trait TaxonTopologicalIterator<'a>:
    Iterator<Item = <<Self as TaxonTopologicalIterator<'a>>::Taxonomy as super::Taxonomy>::Taxon<'a>>
    + Sized
where
    Self: 'a,
{
    /// Type of the taxon.
    type Taxonomy: super::Taxonomy;
}
