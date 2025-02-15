// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, BaselinePosition, Buildable, Container, Orientable, Orientation, PackType, ResizeMode,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkBox")]
    pub struct Box(Object<ffi::GtkBox, ffi::GtkBoxClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_box_get_type(),
    }
}

impl Box {
    pub const NONE: Option<&'static Box> = None;

    #[doc(alias = "gtk_box_new")]
    pub fn new(orientation: Orientation, spacing: i32) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_box_new(orientation.into_glib(), spacing)).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Box`] objects.
    ///
    /// This method returns an instance of [`BoxBuilder`](crate::builders::BoxBuilder) which can be used to create [`Box`] objects.
    pub fn builder() -> BoxBuilder {
        BoxBuilder::new()
    }
}

impl Default for Box {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Box`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BoxBuilder {
    builder: glib::object::ObjectBuilder<'static, Box>,
}

impl BoxBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn baseline_position(self, baseline_position: BaselinePosition) -> Self {
        Self {
            builder: self
                .builder
                .property("baseline-position", baseline_position),
        }
    }

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
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
    /// Build the [`Box`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Box {
        self.builder.build()
    }
}

pub trait BoxExt: 'static {
    #[doc(alias = "gtk_box_get_baseline_position")]
    #[doc(alias = "get_baseline_position")]
    fn baseline_position(&self) -> BaselinePosition;

    #[doc(alias = "gtk_box_get_center_widget")]
    #[doc(alias = "get_center_widget")]
    fn center_widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_box_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    fn is_homogeneous(&self) -> bool;

    #[doc(alias = "gtk_box_get_spacing")]
    #[doc(alias = "get_spacing")]
    fn spacing(&self) -> i32;

    #[doc(alias = "gtk_box_pack_end")]
    fn pack_end(&self, child: &impl IsA<Widget>, expand: bool, fill: bool, padding: u32);

    #[doc(alias = "gtk_box_pack_start")]
    fn pack_start(&self, child: &impl IsA<Widget>, expand: bool, fill: bool, padding: u32);

    #[doc(alias = "gtk_box_query_child_packing")]
    fn query_child_packing(&self, child: &impl IsA<Widget>) -> (bool, bool, u32, PackType);

    #[doc(alias = "gtk_box_reorder_child")]
    fn reorder_child(&self, child: &impl IsA<Widget>, position: i32);

    #[doc(alias = "gtk_box_set_baseline_position")]
    fn set_baseline_position(&self, position: BaselinePosition);

    #[doc(alias = "gtk_box_set_center_widget")]
    fn set_center_widget(&self, widget: Option<&impl IsA<Widget>>);

    #[doc(alias = "gtk_box_set_child_packing")]
    fn set_child_packing(
        &self,
        child: &impl IsA<Widget>,
        expand: bool,
        fill: bool,
        padding: u32,
        pack_type: PackType,
    );

    #[doc(alias = "gtk_box_set_homogeneous")]
    fn set_homogeneous(&self, homogeneous: bool);

    #[doc(alias = "gtk_box_set_spacing")]
    fn set_spacing(&self, spacing: i32);

    fn child_expands<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    fn set_child_expand<T: IsA<crate::Widget>>(&self, item: &T, expand: bool);

    fn child_fills<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    fn set_child_fill<T: IsA<crate::Widget>>(&self, item: &T, fill: bool);

    #[doc(alias = "child.pack-type")]
    fn child_pack_type<T: IsA<crate::Widget>>(&self, item: &T) -> PackType;

    #[doc(alias = "child.pack-type")]
    fn set_child_pack_type<T: IsA<crate::Widget>>(&self, item: &T, pack_type: PackType);

    fn child_padding<T: IsA<crate::Widget>>(&self, item: &T) -> u32;

    fn set_child_padding<T: IsA<crate::Widget>>(&self, item: &T, padding: u32);

    fn child_position<T: IsA<crate::Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<crate::Widget>>(&self, item: &T, position: i32);

    #[doc(alias = "baseline-position")]
    fn connect_baseline_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "homogeneous")]
    fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "spacing")]
    fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Box>> BoxExt for O {
    fn baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_box_get_baseline_position(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_box_get_center_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_homogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_box_get_homogeneous(self.as_ref().to_glib_none().0)) }
    }

    fn spacing(&self) -> i32 {
        unsafe { ffi::gtk_box_get_spacing(self.as_ref().to_glib_none().0) }
    }

    fn pack_end(&self, child: &impl IsA<Widget>, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_end(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                expand.into_glib(),
                fill.into_glib(),
                padding,
            );
        }
    }

    fn pack_start(&self, child: &impl IsA<Widget>, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_start(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                expand.into_glib(),
                fill.into_glib(),
                padding,
            );
        }
    }

    fn query_child_packing(&self, child: &impl IsA<Widget>) -> (bool, bool, u32, PackType) {
        unsafe {
            let mut expand = mem::MaybeUninit::uninit();
            let mut fill = mem::MaybeUninit::uninit();
            let mut padding = mem::MaybeUninit::uninit();
            let mut pack_type = mem::MaybeUninit::uninit();
            ffi::gtk_box_query_child_packing(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                expand.as_mut_ptr(),
                fill.as_mut_ptr(),
                padding.as_mut_ptr(),
                pack_type.as_mut_ptr(),
            );
            (
                from_glib(expand.assume_init()),
                from_glib(fill.assume_init()),
                padding.assume_init(),
                from_glib(pack_type.assume_init()),
            )
        }
    }

    fn reorder_child(&self, child: &impl IsA<Widget>, position: i32) {
        unsafe {
            ffi::gtk_box_reorder_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_set_baseline_position(
                self.as_ref().to_glib_none().0,
                position.into_glib(),
            );
        }
    }

    fn set_center_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_box_set_center_widget(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_child_packing(
        &self,
        child: &impl IsA<Widget>,
        expand: bool,
        fill: bool,
        padding: u32,
        pack_type: PackType,
    ) {
        unsafe {
            ffi::gtk_box_set_child_packing(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                expand.into_glib(),
                fill.into_glib(),
                padding,
                pack_type.into_glib(),
            );
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_set_homogeneous(self.as_ref().to_glib_none().0, homogeneous.into_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_box_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn child_expands<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "expand",
        )
    }

    fn set_child_expand<T: IsA<crate::Widget>>(&self, item: &T, expand: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "expand",
            &expand,
        )
    }

    fn child_fills<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "fill",
        )
    }

    fn set_child_fill<T: IsA<crate::Widget>>(&self, item: &T, fill: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "fill",
            &fill,
        )
    }

    fn child_pack_type<T: IsA<crate::Widget>>(&self, item: &T) -> PackType {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "pack-type",
        )
    }

    fn set_child_pack_type<T: IsA<crate::Widget>>(&self, item: &T, pack_type: PackType) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "pack-type",
            &pack_type,
        )
    }

    fn child_padding<T: IsA<crate::Widget>>(&self, item: &T) -> u32 {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "padding",
        )
    }

    fn set_child_padding<T: IsA<crate::Widget>>(&self, item: &T, padding: u32) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "padding",
            &padding,
        )
    }

    fn child_position<T: IsA<crate::Widget>>(&self, item: &T) -> i32 {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "position",
        )
    }

    fn set_child_position<T: IsA<crate::Widget>>(&self, item: &T, position: i32) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "position",
            &position,
        )
    }

    fn connect_baseline_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_baseline_position_trampoline<
            P: IsA<Box>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Box::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::baseline-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_baseline_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<P: IsA<Box>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Box::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<P: IsA<Box>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Box::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Box")
    }
}
