use super::column::Column;
use std::cell::RefCell;

pub struct Table<'t> {
    // The name of the table
    name: &'t str,
    // Keeps track of created `Column`s (via indexing), so they are properly dropped
    col_ptrs: RefCell<Vec<*mut Column<'t>>>,
}

impl<'t> std::ops::Index<&'t str> for Table<'t> {
    type Output = Column<'t>;

    /// Returns a `Column` of the given table
    /// # Safety
    /// This operation does not check the validity of the column. That is left to the `Checker`.
    fn index(&self, col_name: &'t str) -> &Self::Output {
        let col_ref = Box::leak(Box::new(Column::new(col_name)));
        // Keep track of the "leaked" references
        self.col_ptrs.borrow_mut().push(col_ref as *mut _);
        col_ref
    }
}

impl<'t> std::ops::Drop for Table<'t> {
    fn drop(&mut self) {
        // Drop all of the allocated "leaked" references to `Column`s
        // Safety:
        // - The pointers are valid as they were created by `Box::leak`.
        // - The values have not already been dropped, as there is no other way to do so.
        // - No other pointers exist that point to the same locations, as they can't be made.
        // - No double drop due to how the pointers are created.
        self.col_ptrs
            .borrow()
            .iter()
            .for_each(|&ptr| drop(unsafe { Box::from_raw(ptr) }));
    }
}
