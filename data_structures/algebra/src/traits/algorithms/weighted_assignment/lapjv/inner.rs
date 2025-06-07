//! Submodule providing the concrete implementation of the LAPJV algorithm.

use common_traits::prelude::TotalOrd;
use num_traits::ConstZero;
use numeric_common_traits::prelude::{Bounded, Finite, IntoUsize, Number, TryFromUsize};

use super::LAPJVError;
use crate::traits::{AssignmentState, DenseValuedMatrix2D};

/// Support struct for computing the weighted assignment using the LAPJV
/// algorithm.
pub(super) struct Inner<'matrix, M: DenseValuedMatrix2D + ?Sized> {
    /// The matrix to compute the assignment on.
    matrix: &'matrix M,
    /// The column costs of the matrix.
    column_costs: Vec<M::Value>,
    /// Vector of unassigned rows.
    unassigned_rows: Vec<M::RowIndex>,
    /// The maximum cost of the matrix.
    max_cost: M::Value,
    /// Vector of assigned rows.
    assigned_rows: Vec<AssignmentState<M::RowIndex>>,
    /// Vector of assigned columns.
    assigned_columns: Vec<AssignmentState<M::ColumnIndex>>,
}

impl<'matrix, M: DenseValuedMatrix2D + ?Sized> From<Inner<'matrix, M>>
    for Vec<(M::RowIndex, M::ColumnIndex)>
where
    M::Value: Number,
    M::ColumnIndex: TryFromUsize,
{
    fn from(inner: Inner<'matrix, M>) -> Self {
        let mut assignments: Vec<(M::RowIndex, M::ColumnIndex)> =
            Vec::with_capacity(inner.matrix.number_of_rows().into_usize());
        for (expected_column_index, maybe_row_index) in inner.assigned_rows.into_iter().enumerate()
        {
            let AssignmentState::Assigned(row_index) = maybe_row_index else {
                unreachable!("We expected the assigned row to be in the assigned state");
            };

            assignments
                .push((row_index, M::ColumnIndex::try_from_usize(expected_column_index).unwrap()));
        }
        assignments
    }
}

impl<'matrix, M: DenseValuedMatrix2D + ?Sized> Inner<'matrix, M>
where
    M::Value: Number,
{
    pub(super) fn new(matrix: &'matrix M, max_cost: M::Value) -> Result<Self, LAPJVError> {
        // Check if the matrix is square
        if matrix.number_of_rows().into_usize() != matrix.number_of_columns().into_usize() {
            return Err(LAPJVError::NonSquareMatrix);
        }

        let column_costs = vec![max_cost; matrix.number_of_columns().into_usize()];

        Ok(Inner {
            matrix,
            column_costs,
            unassigned_rows: Vec::new(),
            max_cost,
            assigned_rows: vec![AssignmentState::Unassigned; matrix.number_of_rows().into_usize()],
            assigned_columns: vec![
                AssignmentState::Unassigned;
                matrix.number_of_columns().into_usize()
            ],
        })
    }
}

impl<M: DenseValuedMatrix2D + ?Sized> Inner<'_, M>
where
    M::Value: Number,
{
    #[inline]
    pub(super) fn column_reduction(&mut self) -> Result<(), LAPJVError> {
        debug_assert!(
            self.column_costs.iter().all(|&cost| cost == self.max_cost),
            "We expected the column costs to be initialized to the maximum cost",
        );
        debug_assert!(
            self.assigned_rows.iter().all(AssignmentState::is_unassigned),
            "We expected all rows to be unassigned",
        );
        debug_assert!(
            self.assigned_columns.iter().all(AssignmentState::is_unassigned),
            "We expected all columns to be unassigned",
        );

        for row_index in self.matrix.row_indices() {
            // We retrieve the minimum value and its column on the row.
            for (column_index, value) in
                self.matrix.column_indices().zip(self.matrix.row_values(row_index))
            {
                if value >= self.max_cost {
                    return Err(LAPJVError::ValueTooLarge);
                }

                if !value.is_finite() {
                    return Err(LAPJVError::NonFiniteValues);
                }

                if value == M::Value::ZERO {
                    return Err(LAPJVError::ZeroValues);
                }

                if value < M::Value::ZERO {
                    return Err(LAPJVError::NegativeValues);
                }

                if value < self.column_costs[column_index.into_usize()] {
                    self.assigned_rows[column_index.into_usize()] =
                        AssignmentState::Assigned(row_index);
                    self.column_costs[column_index.into_usize()] = value;
                }
            }
        }

        debug_assert!(
            self.assigned_rows.iter().all(AssignmentState::is_assigned),
            "We expected all rows to be assigned",
        );

        for column_index in self.matrix.column_indices().rev() {
            let AssignmentState::Assigned(assigned_row) =
                self.assigned_rows[column_index.into_usize()]
            else {
                unreachable!("We expected the assigned row to be in the assigned state");
            };
            match self.assigned_columns.get(assigned_row.into_usize()) {
                Some(AssignmentState::Unassigned) => {
                    self.assigned_columns[assigned_row.into_usize()] =
                        AssignmentState::Assigned(column_index);
                }
                Some(
                    AssignmentState::Assigned(assigned_column_index)
                    | AssignmentState::Conflict(assigned_column_index),
                ) => {
                    self.assigned_columns[assigned_row.into_usize()] =
                        AssignmentState::Conflict(*assigned_column_index);
                    self.assigned_rows[column_index.into_usize()] = AssignmentState::Unassigned;
                }
                None => {
                    unreachable!("We expected the assigned column to be in the assigned state");
                }
            }
        }

        Ok(())
    }

    #[inline]
    pub(super) fn reduction_transfer(&mut self) {
        debug_assert!(
            self.unassigned_rows.is_empty(),
            "We expected the unassigned rows to be empty",
        );

        for row_index in self.matrix.row_indices() {
            match self.assigned_columns.get(row_index.into_usize()) {
                Some(AssignmentState::Unassigned) => {
                    self.unassigned_rows.push(row_index);
                }
                Some(AssignmentState::Conflict(conflicted_column)) => {
                    self.assigned_columns[row_index.into_usize()] =
                        AssignmentState::Assigned(*conflicted_column);
                }
                Some(AssignmentState::Assigned(assigned_column)) => {
                    let minimum_reduced_cost = self
                        .matrix
                        .column_indices()
                        .zip(self.matrix.row_values(row_index))
                        // We remove the columns that match the assigned column
                        .filter_map(|(column_index, value)| {
                            if column_index == *assigned_column {
                                None
                            } else {
                                Some(value - self.column_costs[column_index.into_usize()])
                            }
                        })
                        .min_by(|&a, &b| a.total_cmp(&b))
                        .unwrap_or(self.max_cost);
                    self.column_costs[assigned_column.into_usize()] -= minimum_reduced_cost;
                }
                None => {
                    unreachable!("We expected the assigned column to be in the assigned state");
                }
            }
        }
    }

    #[inline]
    #[allow(clippy::type_complexity)]
    /// Returns the first and second minimum reduced costs of the row.
    ///
    /// # Arguments
    ///
    /// * `row_index`: The row index.
    fn first_and_second_minimum_reduced_costs(
        &self,
        row_index: M::RowIndex,
    ) -> ((M::ColumnIndex, M::Value), (Option<M::ColumnIndex>, M::Value)) {
        let mut iterator = self.matrix.column_indices().zip(self.matrix.row_values(row_index)).map(
            |(column_index, cost)| {
                (column_index, cost - self.column_costs[column_index.into_usize()])
            },
        );

        let (mut first_minimum_index, mut first_minimum_reduced_cost) =
            iterator.next().expect("We expected the iterator to have at least one element");

        let mut second_minimum_column_index: Option<M::ColumnIndex> = None;
        let mut second_minimum_reduced_cost = self.max_cost;
        for (column_index, reduced_cost) in iterator {
            if reduced_cost < second_minimum_reduced_cost {
                if reduced_cost >= first_minimum_reduced_cost {
                    second_minimum_column_index = Some(column_index);
                    second_minimum_reduced_cost = reduced_cost;
                } else {
                    second_minimum_column_index = Some(first_minimum_index);
                    second_minimum_reduced_cost = first_minimum_reduced_cost;
                    first_minimum_index = column_index;
                    first_minimum_reduced_cost = reduced_cost;
                }
            }
        }

        debug_assert!(
            first_minimum_reduced_cost <= second_minimum_reduced_cost,
            "We expected the first minimum reduced cost to be less than or equal to the second minimum reduced cost"
        );

        (
            (first_minimum_index, first_minimum_reduced_cost),
            (second_minimum_column_index, second_minimum_reduced_cost),
        )
    }

    #[inline]
    pub(super) fn augmenting_row_reduction(&mut self) {
        if self.unassigned_rows.is_empty() {
            return;
        }

        let mut current_unassigned_row_index = 0;
        let mut updated_number_of_unassigned_rows = 0;
        let mut number_of_iterations = 0;
        let number_of_rows = self.matrix.number_of_rows().into_usize();
        let original_number_of_unassigned_rows = self.unassigned_rows.len();

        while current_unassigned_row_index < original_number_of_unassigned_rows {
            let unassigned_row_index = self.unassigned_rows[current_unassigned_row_index];
            current_unassigned_row_index += 1;
            number_of_iterations += 1;

            // We determine the first and second minimum reduced costs of the
            // row.
            let (
                (mut first_minimum_column_index, first_minimum_value),
                (second_minimum_column_index, second_minimum_value),
            ) = self.first_and_second_minimum_reduced_costs(unassigned_row_index);

            // We retrieve the row associated with the column of the first
            // minimum reduced cost.
            let mut row_index = self.assigned_rows[first_minimum_column_index.into_usize()];

            if number_of_iterations < current_unassigned_row_index * number_of_rows {
                if first_minimum_value < second_minimum_value {
                    self.column_costs[first_minimum_column_index.into_usize()] -=
                        second_minimum_value - first_minimum_value;
                } else if let (AssignmentState::Assigned(_), Some(second_minimum_column_index)) =
                    (row_index, second_minimum_column_index)
                {
                    first_minimum_column_index = second_minimum_column_index;
                    row_index = self.assigned_rows[first_minimum_column_index.into_usize()];
                }
                if let AssignmentState::Assigned(assigned_row) = row_index {
                    if first_minimum_value < second_minimum_value {
                        current_unassigned_row_index -= 1;
                        self.unassigned_rows[current_unassigned_row_index] = assigned_row;
                    } else {
                        self.unassigned_rows[updated_number_of_unassigned_rows] = assigned_row;
                        updated_number_of_unassigned_rows += 1;
                    }
                }
            } else if let AssignmentState::Assigned(assigned_row) = row_index {
                self.unassigned_rows[updated_number_of_unassigned_rows] = assigned_row;
                updated_number_of_unassigned_rows += 1;
            }

            // We update the assigned row with the new column index.
            self.assigned_rows[first_minimum_column_index.into_usize()] =
                AssignmentState::Assigned(unassigned_row_index);
            // We update the assigned column with the new row index.
            self.assigned_columns[unassigned_row_index.into_usize()] =
                AssignmentState::Assigned(first_minimum_column_index);
        }

        self.unassigned_rows.truncate(updated_number_of_unassigned_rows);

        debug_assert!(
            self.unassigned_rows.iter().all(|row_index| {
                !self.assigned_rows.contains(&AssignmentState::Assigned(*row_index))
            }),
            "We expected all unassigned rows to be unassigned",
        );
    }

    #[inline]
    fn find_minimum_distance(
        lower_bound: usize,
        distances: &[M::Value],
        to_scan: &mut [M::ColumnIndex],
    ) -> usize {
        debug_assert!(
            lower_bound < to_scan.len(),
            "We expected the lower bound to be less than the length of the to scan vector"
        );
        let mut upper_bound = lower_bound + 1;
        let column_index = to_scan[lower_bound];
        let mut minimum_distance = distances[column_index.into_usize()];

        for k in lower_bound + 1..to_scan.len() {
            let column_index = to_scan[k];
            let distance = distances[column_index.into_usize()];
            if distance <= minimum_distance {
                if distance < minimum_distance {
                    upper_bound = lower_bound;
                    minimum_distance = distance;
                }
                to_scan[k] = to_scan[upper_bound];
                to_scan[upper_bound] = column_index;
                upper_bound += 1;
            }
        }

        upper_bound
    }

    #[inline]
    fn scan(
        &self,
        lower_bound_ref: &mut usize,
        upper_bound_ref: &mut usize,
        to_scan: &mut [M::ColumnIndex],
        distances: &mut [M::Value],
        predecessors: &mut [M::RowIndex],
    ) -> Option<M::ColumnIndex> {
        let mut lower_bound = *lower_bound_ref;
        let mut upper_bound = *upper_bound_ref;
        let mut number_of_iterations = 0;
        while lower_bound != upper_bound {
            number_of_iterations += 1;
            debug_assert!(
                number_of_iterations < self.matrix.number_of_columns().into_usize(),
                "We expected the number of iterations to be less than the number of columns ({}), with max cost {}",
                self.matrix.number_of_columns(),
                self.max_cost
            );

            let column_index = to_scan[lower_bound];
            lower_bound += 1;
            let AssignmentState::Assigned(row_index) =
                self.assigned_rows[column_index.into_usize()]
            else {
                unreachable!("We expected the assigned row to be in the assigned state");
            };
            let minimum_distance = distances[column_index.into_usize()];
            let initial_reduced_cost = self.matrix.value((row_index, column_index))
                - self.column_costs[column_index.into_usize()]
                - minimum_distance;

            let current_upper_bound = upper_bound;
            for k in current_upper_bound..to_scan.len() {
                let column_index = to_scan[k];
                let reduced_cost = self.matrix.value((row_index, column_index))
                    - self.column_costs[column_index.into_usize()]
                    - initial_reduced_cost;
                if reduced_cost < distances[column_index.into_usize()] {
                    distances[column_index.into_usize()] = reduced_cost;
                    predecessors[column_index.into_usize()] = row_index;
                    if reduced_cost == minimum_distance {
                        if self.assigned_rows[column_index.into_usize()].is_unassigned() {
                            return Some(column_index);
                        }
                        to_scan[k] = to_scan[upper_bound];
                        to_scan[upper_bound] = column_index;
                        upper_bound += 1;
                    }
                }
            }
        }

        debug_assert!(
            lower_bound < to_scan.len(),
            "We expected the lower bound to be less than the length of the to scan vector"
        );

        *lower_bound_ref = lower_bound;
        *upper_bound_ref = upper_bound;

        None
    }

    #[inline]
    fn find_path(
        &mut self,
        start_row_index: M::RowIndex,
        to_scan: &mut [M::ColumnIndex],
        predecessors: &mut [M::RowIndex],
        distances: &mut [M::Value],
    ) -> M::ColumnIndex {
        let mut lower_bound = 0;
        let mut upper_bound = 0;
        let mut n_ready = 0;

        for (column_index, (column_index_to_scan, (predecessor, distance))) in self
            .matrix
            .column_indices()
            .zip(to_scan.iter_mut().zip(predecessors.iter_mut().zip(distances.iter_mut())))
        {
            *predecessor = start_row_index;
            *column_index_to_scan = column_index;
            *distance = self.matrix.value((start_row_index, column_index))
                - self.column_costs[column_index.into_usize()];
        }

        let mut number_of_iterations = 0;
        let column_index = 'outer: loop {
            number_of_iterations += 1;
            debug_assert!(
                number_of_iterations < self.matrix.number_of_columns().into_usize(),
                "We expected the number of iterations to be less than the number of columns ({}): with max cost {}",
                self.matrix.number_of_columns(),
                self.max_cost
            );

            if lower_bound == upper_bound {
                n_ready = lower_bound;
                upper_bound = Self::find_minimum_distance(lower_bound, distances, to_scan);

                for column_index in to_scan[lower_bound..upper_bound].iter().copied() {
                    if self.assigned_rows[column_index.into_usize()].is_unassigned() {
                        break 'outer column_index;
                    }
                }
            }

            if let Some(column_index) =
                self.scan(&mut lower_bound, &mut upper_bound, to_scan, distances, predecessors)
            {
                break 'outer column_index;
            }
        };

        let minimum_distance = distances[to_scan[lower_bound].into_usize()];
        for column_index in to_scan[0..n_ready].iter().copied() {
            self.column_costs[column_index.into_usize()] +=
                distances[column_index.into_usize()] - minimum_distance;
        }

        column_index
    }

    #[inline]
    pub(super) fn augmentation(&mut self) {
        if self.unassigned_rows.is_empty() {
            return;
        }

        let mut to_scan = vec![M::ColumnIndex::MAX; self.matrix.number_of_columns().into_usize()];
        let mut predecessors = vec![M::RowIndex::MAX; self.matrix.number_of_columns().into_usize()];
        let mut distances = vec![self.max_cost; self.matrix.number_of_columns().into_usize()];

        while let Some(unassigned_row_index) = self.unassigned_rows.pop() {
            let mut column_index = self.find_path(
                unassigned_row_index,
                &mut to_scan,
                &mut predecessors,
                &mut distances,
            );

            let mut number_of_iterations = 0;
            loop {
                debug_assert!(
                    number_of_iterations
                        < self.matrix.number_of_columns().into_usize()
                            * self.matrix.number_of_columns().into_usize(),
                    "We expected the number of iterations to be less than the number of columns",
                );

                let row_index = predecessors[column_index.into_usize()];

                self.assigned_rows[column_index.into_usize()] =
                    AssignmentState::Assigned(row_index);

                let AssignmentState::Assigned(old_column_index) =
                    self.assigned_columns[row_index.into_usize()]
                else {
                    unreachable!("We expected the assigned column to be in the assigned state");
                };
                self.assigned_columns[row_index.into_usize()] =
                    AssignmentState::Assigned(column_index);
                column_index = old_column_index;
                number_of_iterations += 1;

                if row_index == unassigned_row_index {
                    break;
                }
            }
        }
    }
}
