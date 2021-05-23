// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::LevelBarMode;
use crate::Orientable;
use crate::Orientation;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkLevelBar")]
    pub struct LevelBar(Object<ffi::GtkLevelBar, ffi::GtkLevelBarClass>) @extends Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    #[doc(alias = "gtk_level_bar_new")]
    pub fn new() -> LevelBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_level_bar_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_level_bar_new_for_interval")]
    #[doc(alias = "new_for_interval")]
    pub fn for_interval(min_value: f64, max_value: f64) -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min_value, max_value))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`LevelBar`] objects.
    ///
    /// This method returns an instance of [`LevelBarBuilder`] which can be used to create [`LevelBar`] objects.
    pub fn builder() -> LevelBarBuilder {
        LevelBarBuilder::default()
    }
}

impl Default for LevelBar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`LevelBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct LevelBarBuilder {
    inverted: Option<bool>,
    max_value: Option<f64>,
    min_value: Option<f64>,
    mode: Option<LevelBarMode>,
    value: Option<f64>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl LevelBarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`LevelBarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`LevelBar`].
    pub fn build(self) -> LevelBar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        if let Some(ref max_value) = self.max_value {
            properties.push(("max-value", max_value));
        }
        if let Some(ref min_value) = self.min_value {
            properties.push(("min-value", min_value));
        }
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        if let Some(ref value) = self.value {
            properties.push(("value", value));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<LevelBar>(&properties)
            .expect("Failed to create an instance of LevelBar")
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    pub fn max_value(mut self, max_value: f64) -> Self {
        self.max_value = Some(max_value);
        self
    }

    pub fn min_value(mut self, min_value: f64) -> Self {
        self.min_value = Some(min_value);
        self
    }

    pub fn mode(mut self, mode: LevelBarMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &impl IsA<Container>) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_LEVEL_BAR: Option<&LevelBar> = None;

pub trait LevelBarExt: 'static {
    #[doc(alias = "gtk_level_bar_add_offset_value")]
    fn add_offset_value(&self, name: &str, value: f64);

    #[doc(alias = "gtk_level_bar_get_inverted")]
    #[doc(alias = "get_inverted")]
    fn is_inverted(&self) -> bool;

    #[doc(alias = "gtk_level_bar_get_max_value")]
    #[doc(alias = "get_max_value")]
    fn max_value(&self) -> f64;

    #[doc(alias = "gtk_level_bar_get_min_value")]
    #[doc(alias = "get_min_value")]
    fn min_value(&self) -> f64;

    #[doc(alias = "gtk_level_bar_get_mode")]
    #[doc(alias = "get_mode")]
    fn mode(&self) -> LevelBarMode;

    #[doc(alias = "gtk_level_bar_get_offset_value")]
    #[doc(alias = "get_offset_value")]
    fn offset_value(&self, name: Option<&str>) -> Option<f64>;

    #[doc(alias = "gtk_level_bar_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> f64;

    #[doc(alias = "gtk_level_bar_remove_offset_value")]
    fn remove_offset_value(&self, name: Option<&str>);

    #[doc(alias = "gtk_level_bar_set_inverted")]
    fn set_inverted(&self, inverted: bool);

    #[doc(alias = "gtk_level_bar_set_max_value")]
    fn set_max_value(&self, value: f64);

    #[doc(alias = "gtk_level_bar_set_min_value")]
    fn set_min_value(&self, value: f64);

    #[doc(alias = "gtk_level_bar_set_mode")]
    fn set_mode(&self, mode: LevelBarMode);

    #[doc(alias = "gtk_level_bar_set_value")]
    fn set_value(&self, value: f64);

    #[doc(alias = "offset-changed")]
    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "inverted")]
    fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "max-value")]
    fn connect_max_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "min-value")]
    fn connect_min_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mode")]
    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LevelBar>> LevelBarExt for O {
    fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value,
            );
        }
    }

    fn is_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_inverted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn max_value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_max_value(self.as_ref().to_glib_none().0) }
    }

    fn min_value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_min_value(self.as_ref().to_glib_none().0) }
    }

    fn mode(&self) -> LevelBarMode {
        unsafe { from_glib(ffi::gtk_level_bar_get_mode(self.as_ref().to_glib_none().0)) }
    }

    fn offset_value(&self, name: Option<&str>) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_level_bar_get_offset_value(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            let value = value.assume_init();
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    fn value(&self) -> f64 {
        unsafe { ffi::gtk_level_bar_get_value(self.as_ref().to_glib_none().0) }
    }

    fn remove_offset_value(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_level_bar_set_inverted(self.as_ref().to_glib_none().0, inverted.into_glib());
        }
    }

    fn set_max_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_max_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_min_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_min_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            ffi::gtk_level_bar_set_mode(self.as_ref().to_glib_none().0, mode.into_glib());
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn connect_offset_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn offset_changed_trampoline<
            P: IsA<LevelBar>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::GtkLevelBar,
            name: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                LevelBar::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("offset-changed::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"offset-changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    offset_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<P: IsA<LevelBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkLevelBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LevelBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_max_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_value_trampoline<P: IsA<LevelBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkLevelBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LevelBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_min_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_value_trampoline<P: IsA<LevelBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkLevelBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LevelBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P: IsA<LevelBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkLevelBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LevelBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P: IsA<LevelBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkLevelBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LevelBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for LevelBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LevelBar")
    }
}
