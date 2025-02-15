// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Align, Bin, Buildable, Container, Entry, ResizeMode, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSearchBar")]
    pub struct SearchBar(Object<ffi::GtkSearchBar, ffi::GtkSearchBarClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_search_bar_get_type(),
    }
}

impl SearchBar {
    pub const NONE: Option<&'static SearchBar> = None;

    #[doc(alias = "gtk_search_bar_new")]
    pub fn new() -> SearchBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_search_bar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SearchBar`] objects.
    ///
    /// This method returns an instance of [`SearchBarBuilder`](crate::builders::SearchBarBuilder) which can be used to create [`SearchBar`] objects.
    pub fn builder() -> SearchBarBuilder {
        SearchBarBuilder::new()
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SearchBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SearchBarBuilder {
    builder: glib::object::ObjectBuilder<'static, SearchBar>,
}

impl SearchBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn search_mode_enabled(self, search_mode_enabled: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("search-mode-enabled", search_mode_enabled),
        }
    }

    pub fn show_close_button(self, show_close_button: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-close-button", show_close_button),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`SearchBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SearchBar {
        self.builder.build()
    }
}

pub trait SearchBarExt: 'static {
    #[doc(alias = "gtk_search_bar_connect_entry")]
    fn connect_entry(&self, entry: &impl IsA<Entry>);

    #[doc(alias = "gtk_search_bar_get_search_mode")]
    #[doc(alias = "get_search_mode")]
    fn is_search_mode(&self) -> bool;

    #[doc(alias = "gtk_search_bar_get_show_close_button")]
    #[doc(alias = "get_show_close_button")]
    fn shows_close_button(&self) -> bool;

    #[doc(alias = "gtk_search_bar_handle_event")]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[doc(alias = "gtk_search_bar_set_search_mode")]
    fn set_search_mode(&self, search_mode: bool);

    #[doc(alias = "gtk_search_bar_set_show_close_button")]
    fn set_show_close_button(&self, visible: bool);

    #[doc(alias = "search-mode-enabled")]
    fn is_search_mode_enabled(&self) -> bool;

    #[doc(alias = "search-mode-enabled")]
    fn set_search_mode_enabled(&self, search_mode_enabled: bool);

    #[doc(alias = "search-mode-enabled")]
    fn connect_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-close-button")]
    fn connect_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchBar>> SearchBarExt for O {
    fn connect_entry(&self, entry: &impl IsA<Entry>) {
        unsafe {
            ffi::gtk_search_bar_connect_entry(
                self.as_ref().to_glib_none().0,
                entry.as_ref().to_glib_none().0,
            );
        }
    }

    fn is_search_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_search_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_show_close_button(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_handle_event(
                self.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
            ))
        }
    }

    fn set_search_mode(&self, search_mode: bool) {
        unsafe {
            ffi::gtk_search_bar_set_search_mode(
                self.as_ref().to_glib_none().0,
                search_mode.into_glib(),
            );
        }
    }

    fn set_show_close_button(&self, visible: bool) {
        unsafe {
            ffi::gtk_search_bar_set_show_close_button(
                self.as_ref().to_glib_none().0,
                visible.into_glib(),
            );
        }
    }

    fn is_search_mode_enabled(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "search-mode-enabled")
    }

    fn set_search_mode_enabled(&self, search_mode_enabled: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "search-mode-enabled", &search_mode_enabled)
    }

    fn connect_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_mode_enabled_trampoline<
            P: IsA<SearchBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSearchBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SearchBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-mode-enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_mode_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_close_button_trampoline<
            P: IsA<SearchBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSearchBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SearchBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_close_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SearchBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SearchBar")
    }
}
