// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Actionable, Align, Bin, Buildable, Container, Menu, MenuItem, ResizeMode, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCheckMenuItem")]
    pub struct CheckMenuItem(Object<ffi::GtkCheckMenuItem, ffi::GtkCheckMenuItemClass>) @extends MenuItem, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        type_ => || ffi::gtk_check_menu_item_get_type(),
    }
}

impl CheckMenuItem {
    pub const NONE: Option<&'static CheckMenuItem> = None;

    #[doc(alias = "gtk_check_menu_item_new")]
    pub fn new() -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_check_menu_item_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_check_menu_item_new_with_label")]
    #[doc(alias = "new_with_label")]
    pub fn with_label(label: &str) -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new_with_label(
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_check_menu_item_new_with_mnemonic")]
    #[doc(alias = "new_with_mnemonic")]
    pub fn with_mnemonic(label: &str) -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new_with_mnemonic(
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CheckMenuItem`] objects.
    ///
    /// This method returns an instance of [`CheckMenuItemBuilder`](crate::builders::CheckMenuItemBuilder) which can be used to create [`CheckMenuItem`] objects.
    pub fn builder() -> CheckMenuItemBuilder {
        CheckMenuItemBuilder::new()
    }
}

impl Default for CheckMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CheckMenuItem`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CheckMenuItemBuilder {
    builder: glib::object::ObjectBuilder<'static, CheckMenuItem>,
}

impl CheckMenuItemBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn active(self, active: bool) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn draw_as_radio(self, draw_as_radio: bool) -> Self {
        Self {
            builder: self.builder.property("draw-as-radio", draw_as_radio),
        }
    }

    pub fn inconsistent(self, inconsistent: bool) -> Self {
        Self {
            builder: self.builder.property("inconsistent", inconsistent),
        }
    }

    pub fn accel_path(self, accel_path: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("accel-path", accel_path.into()),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn right_justified(self, right_justified: bool) -> Self {
        Self {
            builder: self.builder.property("right-justified", right_justified),
        }
    }

    pub fn submenu(self, submenu: &impl IsA<Menu>) -> Self {
        Self {
            builder: self.builder.property("submenu", submenu.clone().upcast()),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CheckMenuItem`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CheckMenuItem {
        self.builder.build()
    }
}

pub trait CheckMenuItemExt: 'static {
    #[doc(alias = "gtk_check_menu_item_get_active")]
    #[doc(alias = "get_active")]
    fn is_active(&self) -> bool;

    #[doc(alias = "gtk_check_menu_item_get_draw_as_radio")]
    #[doc(alias = "get_draw_as_radio")]
    fn draws_as_radio(&self) -> bool;

    #[doc(alias = "gtk_check_menu_item_get_inconsistent")]
    #[doc(alias = "get_inconsistent")]
    fn is_inconsistent(&self) -> bool;

    #[doc(alias = "gtk_check_menu_item_set_active")]
    fn set_active(&self, is_active: bool);

    #[doc(alias = "gtk_check_menu_item_set_draw_as_radio")]
    fn set_draw_as_radio(&self, draw_as_radio: bool);

    #[doc(alias = "gtk_check_menu_item_set_inconsistent")]
    fn set_inconsistent(&self, setting: bool);

    #[doc(alias = "gtk_check_menu_item_toggled")]
    fn toggled(&self);

    #[doc(alias = "toggled")]
    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "draw-as-radio")]
    fn connect_draw_as_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "inconsistent")]
    fn connect_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CheckMenuItem>> CheckMenuItemExt for O {
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn draws_as_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_draw_as_radio(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_inconsistent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_inconsistent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_active(
                self.as_ref().to_glib_none().0,
                is_active.into_glib(),
            );
        }
    }

    fn set_draw_as_radio(&self, draw_as_radio: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_draw_as_radio(
                self.as_ref().to_glib_none().0,
                draw_as_radio.into_glib(),
            );
        }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_inconsistent(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn toggled(&self) {
        unsafe {
            ffi::gtk_check_menu_item_toggled(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggled_trampoline<P: IsA<CheckMenuItem>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCheckMenuItem,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckMenuItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<
            P: IsA<CheckMenuItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCheckMenuItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckMenuItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_draw_as_radio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_as_radio_trampoline<
            P: IsA<CheckMenuItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCheckMenuItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckMenuItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::draw-as-radio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_as_radio_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_inconsistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inconsistent_trampoline<
            P: IsA<CheckMenuItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCheckMenuItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CheckMenuItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inconsistent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inconsistent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CheckMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CheckMenuItem")
    }
}
