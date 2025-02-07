//! Submodule proving a trait defining a taxon entry.

use crate::traits::taxonomy::Taxonomy;
use crate::traits::TaxonEntry;

/// Trait defining a taxon entry.
pub trait Taxon<'a>: Sized
where
    Self: 'a,
{
    /// Type of the identifier for the taxon.
    type Taxonomy: super::Taxonomy<Taxon<'a> = Self>;

    /// Returns the identifier of the taxon.
    fn id(&self) -> &'a <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id {
        self.entry().id()
    }

    /// Returns the name of the taxon.
    fn name(&self) -> &'a str {
        self.entry().name()
    }

    /// Returns the "ltree path" of the taxon.
    fn ltree_path(&self) -> String {
        if let Some(parent) = self.parent() {
            // Here we recursively call the ltree_path function in order to return parents
            format!(
                "{}.{}", parent.ltree_path(), self.name() 
            )
        } else {
            // this is the base case. We have no parents.
            self.name().to_owned()
        }
    }

    /// Returns the rank of the taxon.
    fn rank(
        &self,
    ) -> &'a <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Rank {
        self.entry().rank()
    }

    /// Returns the identifier of the parent taxon.
    fn parent_id(
        &self,
    ) -> Option<&'a <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id>
    {
        self.entry().parent_id()
    }

    /// Returns the underlying taxon entry.
    fn entry(&self) -> &'a <Self::Taxonomy as super::Taxonomy>::TaxonEntry;

    /// Returns a reference to the underlying taxonomy.
    fn taxonomy(&self) -> &'a Self::Taxonomy;

    /// Returns the identifier of the parent taxon.
    fn parent(&self) -> Option<Self> {
        self.parent_id()
            .and_then(|id| self.taxonomy().taxon_by_id(id).ok())
    }

    /// Returns an iterator over the children of the taxon.
    fn children(&self) -> impl Iterator<Item = Self> {
        self.taxonomy()
            .taxons()
            .filter(move |taxon| taxon.parent_id() == Some(self.id()))
    }
}
