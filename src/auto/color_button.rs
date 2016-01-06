// This file was generated by gir (d4a8bb5) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ColorButton(Object<ffi::GtkColorButton>): Widget, Container, Bin, Button, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_color_button_get_type(),
    }
}

impl ColorButton {
    pub fn new() -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_color(color: /*Ignored*/&gdk::Color) -> ColorButton {
    //    unsafe { TODO: call ffi::gtk_color_button_new_with_color() }
    //}

    //pub fn new_with_rgba(rgba: /*Ignored*/&gdk::RGBA) -> ColorButton {
    //    unsafe { TODO: call ffi::gtk_color_button_new_with_rgba() }
    //}

    pub fn get_alpha(&self) -> u16 {
        unsafe {
            ffi::gtk_color_button_get_alpha(self.to_glib_none().0)
        }
    }

    //pub fn get_color(&self, color: /*Ignored*/gdk::Color) {
    //    unsafe { TODO: call ffi::gtk_color_button_get_color() }
    //}

    //pub fn get_rgba(&self, rgba: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_color_button_get_rgba() }
    //}

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_color_button_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_use_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_color_button_get_use_alpha(self.to_glib_none().0))
        }
    }

    pub fn set_alpha(&self, alpha: u16) {
        unsafe {
            ffi::gtk_color_button_set_alpha(self.to_glib_none().0, alpha);
        }
    }

    //pub fn set_color(&self, color: /*Ignored*/&gdk::Color) {
    //    unsafe { TODO: call ffi::gtk_color_button_set_color() }
    //}

    //pub fn set_rgba(&self, rgba: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_color_button_set_rgba() }
    //}

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_color_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_button_set_use_alpha(self.to_glib_none().0, use_alpha.to_glib());
        }
    }

}
