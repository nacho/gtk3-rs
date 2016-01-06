// This file was generated by gir (d4a8bb5) from gir-files (11e0e6d)
// DO NOT EDIT

use CellArea;
use CellRenderer;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellLayout(Object<ffi::GtkCellLayout>);

    match fn {
        get_type => || ffi::gtk_cell_layout_get_type(),
    }
}

pub trait CellLayoutExt {
    fn add_attribute<T: Upcast<CellRenderer>>(&self, cell: &T, attribute: &str, column: i32);
    fn clear(&self);
    fn clear_attributes<T: Upcast<CellRenderer>>(&self, cell: &T);
    fn get_area(&self) -> Option<CellArea>;
    fn get_cells(&self) -> Vec<CellRenderer>;
    fn pack_end<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool);
    fn pack_start<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool);
    fn reorder<T: Upcast<CellRenderer>>(&self, cell: &T, position: i32);
    //fn set_attributes<T: Upcast<CellRenderer>>(&self, cell: &T, : /*Unknown conversion*/Fundamental: VarArgs);
    //fn set_cell_data_func<T: Upcast<CellRenderer>>(&self, cell: &T, func: /*Unknown conversion*/Unknown rust type: "CellLayoutDataFunc", func_data: Fundamental: Pointer, destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify");
}

impl<O: Upcast<CellLayout>> CellLayoutExt for O {
    fn add_attribute<T: Upcast<CellRenderer>>(&self, cell: &T, attribute: &str, column: i32) {
        unsafe {
            ffi::gtk_cell_layout_add_attribute(self.to_glib_none().0, cell.to_glib_none().0, attribute.to_glib_none().0, column);
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gtk_cell_layout_clear(self.to_glib_none().0);
        }
    }

    fn clear_attributes<T: Upcast<CellRenderer>>(&self, cell: &T) {
        unsafe {
            ffi::gtk_cell_layout_clear_attributes(self.to_glib_none().0, cell.to_glib_none().0);
        }
    }

    fn get_area(&self) -> Option<CellArea> {
        unsafe {
            from_glib_none(ffi::gtk_cell_layout_get_area(self.to_glib_none().0))
        }
    }

    fn get_cells(&self) -> Vec<CellRenderer> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_cell_layout_get_cells(self.to_glib_none().0))
        }
    }

    fn pack_end<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_end(self.to_glib_none().0, cell.to_glib_none().0, expand.to_glib());
        }
    }

    fn pack_start<T: Upcast<CellRenderer>>(&self, cell: &T, expand: bool) {
        unsafe {
            ffi::gtk_cell_layout_pack_start(self.to_glib_none().0, cell.to_glib_none().0, expand.to_glib());
        }
    }

    fn reorder<T: Upcast<CellRenderer>>(&self, cell: &T, position: i32) {
        unsafe {
            ffi::gtk_cell_layout_reorder(self.to_glib_none().0, cell.to_glib_none().0, position);
        }
    }

    //fn set_attributes<T: Upcast<CellRenderer>>(&self, cell: &T, : /*Unknown conversion*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_cell_layout_set_attributes() }
    //}

    //fn set_cell_data_func<T: Upcast<CellRenderer>>(&self, cell: &T, func: /*Unknown conversion*/Unknown rust type: "CellLayoutDataFunc", func_data: Fundamental: Pointer, destroy: /*Unknown conversion*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi::gtk_cell_layout_set_cell_data_func() }
    //}

}
