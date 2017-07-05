// This file was generated by gir (9e3f4cc) from gir-files (71d73f0)
// DO NOT EDIT

use Adjustment;
use Orientable;
use Orientation;
use Range;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Scrollbar(Object<ffi::GtkScrollbar>): Range, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_scrollbar_get_type(),
    }
}

impl Scrollbar {
    pub fn new<'a, P: Into<Option<&'a Adjustment>>>(orientation: Orientation, adjustment: P) -> Scrollbar {
        assert_initialized_main_thread!();
        let adjustment = adjustment.into();
        let adjustment = adjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrollbar_new(orientation.to_glib(), adjustment.0)).downcast_unchecked()
        }
    }
}
