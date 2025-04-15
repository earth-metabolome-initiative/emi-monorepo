//! Trait defining a plate format and the vial positions.
// example: a Thermo 54 vials plate is labelled from A1 to F9.

use data-str
trait RackFormat {
    /// Number of rows in the plate.
    fn rows(&self) -> usize;
    /// Number of columns in the plate.
    fn columns(&self) -> usize;
    /// Number of vials in the plate.
    fn vials(&self) -> usize {
        self.rows() * self.columns()
    }
    /// Get the position of a vial in the plate.
    fn position(&self, row: usize, column: usize) -> usize {
        row * self.columns() + column
    }
    /// Get the row and column of a vial in the plate.
    fn row_column(&self, position: usize) -> (usize, usize) {
        (position / self.columns(), position % self.columns())
    }
    /// Return the Alpha-numeric label of a vial in the plate.
    fn label(&self, position: usize) -> String {
        let (row, column) = self.row_column(position);
        let row_label = (b'A' + row as u8) as char;
        format!("{}{}", row_label, column + 1)
    }
}