//! Submodule providing the support dual matrix to be used in the context of the
//! `Dual` struct of the Hungarian algorithm.

use algebra::prelude::{IntoUsize, Number, ValuedMatrix, ValuedSparseMatrix2D, Zero};

mod matrix_traits;

pub struct DualMatrix<'matrix, M: ValuedMatrix + ?Sized> {
    /// The reference to the underlying matrix.
    pub(super) matrix: &'matrix M,
    /// The dual weights associated with the source nodes.
    /// This is often referred to as the `u` vector.
    /// We represent with `None` the maximum value of the weight.
    left_node_weights: Vec<Option<M::Value>>,
    /// The dual weights associated with the destination nodes.
    /// This is often referred to as the `v` vector.
    /// We represent with `None` the maximum value of the weight.
    right_node_weights: Vec<Option<M::Value>>,
}

impl<M: ValuedSparseMatrix2D + ?Sized> DualMatrix<'_, M>
where
    M::Value: Number + Zero,
{
    /// Increases the weight of the left node by the provided value.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The identifier of the left node.
    /// * `value`: The value to increase the weight by.
    pub(super) fn increase_left_node_weight(&mut self, left_node_id: M::RowIndex, value: M::Value) {
        if let Some(weight) = self.left_node_weights[left_node_id.into_usize()].as_mut() {
            *weight += value;
        }
    }

    /// Reduces the weight of the right node by the provided value.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The identifier of the right node.
    /// * `value`: The value to reduce the weight by.
    pub(super) fn decrease_right_node_weight(&mut self, right_node_id: M::ColumnIndex, value: M::Value) {
        if let Some(weight) = self.right_node_weights[right_node_id.into_usize()].as_mut() {
            *weight -= value;
        }
    }

    /// Returns the weight of the left node.
    ///
    /// # Arguments
    ///
    /// * `left_node_id`: The identifier of the left node.
    pub(super) fn left_node_weight(&self, left_node_id: M::RowIndex) -> Option<M::Value> {
        self.left_node_weights[left_node_id.into_usize()]
    }

    /// Returns the weight of the right node.
    ///
    /// # Arguments
    ///
    /// * `right_node_id`: The identifier of the right node.
    pub(super) fn right_node_weight(&self, right_node_id: M::ColumnIndex) -> Option<M::Value> {
        self.right_node_weights[right_node_id.into_usize()]
    }

    /// Returns an iterator over the columns of a given row
    /// which are characterized by having a zero weight.
    /// 
    /// # Arguments
    /// 
    /// * `row_id`: The identifier of the row.
    /// 
    pub(super) fn zero_weight_columns(
        &self,
        row_id: M::RowIndex,
    ) -> impl Iterator<Item = M::ColumnIndex> + '_ {
        self.matrix
            .sparse_row_values(row_id)
            .zip(self.matrix.sparse_row(row_id))
            .filter_map(move |(weight, column_id)| {
                if weight == M::Value::ZERO {
                    Some(column_id)
                } else {
                    None
                }
            })
    }
}

impl<'matrix, M: ValuedSparseMatrix2D + ?Sized> From<&'matrix M> for DualMatrix<'matrix, M>
where
    M::Value: Number + Zero,
{
    fn from(matrix: &'matrix M) -> Self {
        let left_node_weights: Vec<Option<M::Value>> = matrix.sparse_row_min_values().collect();
        let mut right_node_weights: Vec<Option<M::Value>> =
            vec![None; matrix.number_of_columns().into_usize()];

        matrix.sparse_values().zip(matrix.sparse_coordinates()).for_each(
            |(mut weight, (source_id, destination_id))| {
                let left_weight = left_node_weights[source_id.into_usize()].expect(
                    "Since we are solely iterating on defined values, the weight of the source node should be defined.",
                );
                // We subtract the minimum weight of the row we have determined earlier.
                weight -= left_weight;
                if let Some(current_weight) =
                    right_node_weights[destination_id.into_usize()].as_mut()
                {
                    if *current_weight > weight {
                        *current_weight = weight;
                    }
                } else {
                    right_node_weights[destination_id.into_usize()] = Some(weight);
                }
            },
        );

        DualMatrix { matrix, left_node_weights, right_node_weights }
    }
}
