// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Align, Buildable, Container, Orientable, Orientation, ResizeMode, ScrollType, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkPaned")]
    pub struct Paned(Object<ffi::GtkPaned, ffi::GtkPanedClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_paned_get_type(),
    }
}

impl Paned {
    pub const NONE: Option<&'static Paned> = None;

    #[doc(alias = "gtk_paned_new")]
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_paned_new(orientation.into_glib())).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Paned`] objects.
    ///
    /// This method returns an instance of [`PanedBuilder`](crate::builders::PanedBuilder) which can be used to create [`Paned`] objects.
    pub fn builder() -> PanedBuilder {
        PanedBuilder::new()
    }
}

impl Default for Paned {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Paned`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PanedBuilder {
    builder: glib::object::ObjectBuilder<'static, Paned>,
}

impl PanedBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn position(self, position: i32) -> Self {
        Self {
            builder: self.builder.property("position", position),
        }
    }

    pub fn position_set(self, position_set: bool) -> Self {
        Self {
            builder: self.builder.property("position-set", position_set),
        }
    }

    pub fn wide_handle(self, wide_handle: bool) -> Self {
        Self {
            builder: self.builder.property("wide-handle", wide_handle),
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Paned`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Paned {
        self.builder.build()
    }
}

pub trait PanedExt: 'static {
    #[doc(alias = "gtk_paned_add1")]
    fn add1(&self, child: &impl IsA<Widget>);

    #[doc(alias = "gtk_paned_add2")]
    fn add2(&self, child: &impl IsA<Widget>);

    #[doc(alias = "gtk_paned_get_child1")]
    #[doc(alias = "get_child1")]
    fn child1(&self) -> Option<Widget>;

    #[doc(alias = "gtk_paned_get_child2")]
    #[doc(alias = "get_child2")]
    fn child2(&self) -> Option<Widget>;

    #[doc(alias = "gtk_paned_get_handle_window")]
    #[doc(alias = "get_handle_window")]
    fn handle_window(&self) -> Option<gdk::Window>;

    #[doc(alias = "gtk_paned_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> i32;

    #[doc(alias = "gtk_paned_get_wide_handle")]
    #[doc(alias = "get_wide_handle")]
    fn is_wide_handle(&self) -> bool;

    #[doc(alias = "gtk_paned_pack1")]
    fn pack1(&self, child: &impl IsA<Widget>, resize: bool, shrink: bool);

    #[doc(alias = "gtk_paned_pack2")]
    fn pack2(&self, child: &impl IsA<Widget>, resize: bool, shrink: bool);

    #[doc(alias = "gtk_paned_set_position")]
    fn set_position(&self, position: i32);

    #[doc(alias = "gtk_paned_set_wide_handle")]
    fn set_wide_handle(&self, wide: bool);

    #[doc(alias = "max-position")]
    fn max_position(&self) -> i32;

    #[doc(alias = "min-position")]
    fn min_position(&self) -> i32;

    #[doc(alias = "position-set")]
    fn is_position_set(&self) -> bool;

    #[doc(alias = "position-set")]
    fn set_position_set(&self, position_set: bool);

    fn child_resizes<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    fn set_child_resize<T: IsA<crate::Widget>>(&self, item: &T, resize: bool);

    fn child_shrinks<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    fn set_child_shrink<T: IsA<crate::Widget>>(&self, item: &T, shrink: bool);

    #[doc(alias = "accept-position")]
    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_accept_position(&self) -> bool;

    #[doc(alias = "cancel-position")]
    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancel_position(&self) -> bool;

    #[doc(alias = "cycle-child-focus")]
    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool;

    #[doc(alias = "cycle-handle-focus")]
    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool;

    #[doc(alias = "move-handle")]
    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool;

    #[doc(alias = "toggle-handle-focus")]
    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_handle_focus(&self) -> bool;

    #[doc(alias = "max-position")]
    fn connect_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "min-position")]
    fn connect_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "position")]
    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "position-set")]
    fn connect_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "wide-handle")]
    fn connect_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Paned>> PanedExt for O {
    fn add1(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_paned_add1(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn add2(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_paned_add2(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn child1(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_child1(self.as_ref().to_glib_none().0)) }
    }

    fn child2(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_child2(self.as_ref().to_glib_none().0)) }
    }

    fn handle_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_handle_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn position(&self) -> i32 {
        unsafe { ffi::gtk_paned_get_position(self.as_ref().to_glib_none().0) }
    }

    fn is_wide_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paned_get_wide_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pack1(&self, child: &impl IsA<Widget>, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack1(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.into_glib(),
                shrink.into_glib(),
            );
        }
    }

    fn pack2(&self, child: &impl IsA<Widget>, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack2(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.into_glib(),
                shrink.into_glib(),
            );
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_paned_set_position(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_wide_handle(&self, wide: bool) {
        unsafe {
            ffi::gtk_paned_set_wide_handle(self.as_ref().to_glib_none().0, wide.into_glib());
        }
    }

    fn max_position(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "max-position")
    }

    fn min_position(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "min-position")
    }

    fn is_position_set(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "position-set")
    }

    fn set_position_set(&self, position_set: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "position-set", &position_set)
    }

    fn child_resizes<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "resize",
        )
    }

    fn set_child_resize<T: IsA<crate::Widget>>(&self, item: &T, resize: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "resize",
            &resize,
        )
    }

    fn child_shrinks<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "shrink",
        )
    }

    fn set_child_shrink<T: IsA<crate::Widget>>(&self, item: &T, shrink: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "shrink",
            &shrink,
        )
    }

    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn accept_position_trampoline<
            P: IsA<Paned>,
            F: Fn(&P) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_accept_position(&self) -> bool {
        self.emit_by_name("accept-position", &[])
    }

    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_position_trampoline<
            P: IsA<Paned>,
            F: Fn(&P) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cancel_position(&self) -> bool {
        self.emit_by_name("cancel-position", &[])
    }

    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_child_focus_trampoline<
            P: IsA<Paned>,
            F: Fn(&P, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Paned::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(reversed),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-child-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_child_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool {
        self.emit_by_name("cycle-child-focus", &[&reversed])
    }

    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_handle_focus_trampoline<
            P: IsA<Paned>,
            F: Fn(&P, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Paned::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(reversed),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_handle_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool {
        self.emit_by_name("cycle-handle-focus", &[&reversed])
    }

    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_handle_trampoline<
            P: IsA<Paned>,
            F: Fn(&P, ScrollType) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            scroll_type: ffi::GtkScrollType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Paned::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(scroll_type),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_handle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool {
        self.emit_by_name("move-handle", &[&scroll_type])
    }

    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_handle_focus_trampoline<
            P: IsA<Paned>,
            F: Fn(&P) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggle_handle_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_toggle_handle_focus(&self) -> bool {
        self.emit_by_name("toggle-handle-focus", &[])
    }

    fn connect_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_position_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_position_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_set_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wide_handle_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wide-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wide_handle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Paned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Paned")
    }
}
