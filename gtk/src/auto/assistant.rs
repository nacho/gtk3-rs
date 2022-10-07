// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Application;
use crate::AssistantPageType;
use crate::Bin;
use crate::Buildable;
use crate::Container;
use crate::ResizeMode;
use crate::Widget;
use crate::Window;
use crate::WindowPosition;
use crate::WindowType;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkAssistant")]
    pub struct Assistant(Object<ffi::GtkAssistant, ffi::GtkAssistantClass>) @extends Window, Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_assistant_get_type(),
    }
}

impl Assistant {
    pub const NONE: Option<&'static Assistant> = None;

    #[doc(alias = "gtk_assistant_new")]
    pub fn new() -> Assistant {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_assistant_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Assistant`] objects.
    ///
    /// This method returns an instance of [`AssistantBuilder`](crate::builders::AssistantBuilder) which can be used to create [`Assistant`] objects.
    pub fn builder() -> AssistantBuilder {
        AssistantBuilder::default()
    }
}

impl Default for Assistant {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Assistant`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AssistantBuilder {
    use_header_bar: Option<i32>,
    accept_focus: Option<bool>,
    application: Option<Application>,
    attached_to: Option<Widget>,
    decorated: Option<bool>,
    default_height: Option<i32>,
    default_width: Option<i32>,
    deletable: Option<bool>,
    destroy_with_parent: Option<bool>,
    focus_on_map: Option<bool>,
    focus_visible: Option<bool>,
    gravity: Option<gdk::Gravity>,
    hide_titlebar_when_maximized: Option<bool>,
    icon: Option<gdk_pixbuf::Pixbuf>,
    icon_name: Option<String>,
    mnemonics_visible: Option<bool>,
    modal: Option<bool>,
    resizable: Option<bool>,
    role: Option<String>,
    screen: Option<gdk::Screen>,
    skip_pager_hint: Option<bool>,
    skip_taskbar_hint: Option<bool>,
    startup_id: Option<String>,
    title: Option<String>,
    transient_for: Option<Window>,
    type_: Option<WindowType>,
    type_hint: Option<gdk::WindowTypeHint>,
    urgency_hint: Option<bool>,
    window_position: Option<WindowPosition>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
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
}

impl AssistantBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`AssistantBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Assistant`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Assistant {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref use_header_bar) = self.use_header_bar {
            properties.push(("use-header-bar", use_header_bar));
        }
        if let Some(ref accept_focus) = self.accept_focus {
            properties.push(("accept-focus", accept_focus));
        }
        if let Some(ref application) = self.application {
            properties.push(("application", application));
        }
        if let Some(ref attached_to) = self.attached_to {
            properties.push(("attached-to", attached_to));
        }
        if let Some(ref decorated) = self.decorated {
            properties.push(("decorated", decorated));
        }
        if let Some(ref default_height) = self.default_height {
            properties.push(("default-height", default_height));
        }
        if let Some(ref default_width) = self.default_width {
            properties.push(("default-width", default_width));
        }
        if let Some(ref deletable) = self.deletable {
            properties.push(("deletable", deletable));
        }
        if let Some(ref destroy_with_parent) = self.destroy_with_parent {
            properties.push(("destroy-with-parent", destroy_with_parent));
        }
        if let Some(ref focus_on_map) = self.focus_on_map {
            properties.push(("focus-on-map", focus_on_map));
        }
        if let Some(ref focus_visible) = self.focus_visible {
            properties.push(("focus-visible", focus_visible));
        }
        if let Some(ref gravity) = self.gravity {
            properties.push(("gravity", gravity));
        }
        if let Some(ref hide_titlebar_when_maximized) = self.hide_titlebar_when_maximized {
            properties.push(("hide-titlebar-when-maximized", hide_titlebar_when_maximized));
        }
        if let Some(ref icon) = self.icon {
            properties.push(("icon", icon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref mnemonics_visible) = self.mnemonics_visible {
            properties.push(("mnemonics-visible", mnemonics_visible));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref role) = self.role {
            properties.push(("role", role));
        }
        if let Some(ref screen) = self.screen {
            properties.push(("screen", screen));
        }
        if let Some(ref skip_pager_hint) = self.skip_pager_hint {
            properties.push(("skip-pager-hint", skip_pager_hint));
        }
        if let Some(ref skip_taskbar_hint) = self.skip_taskbar_hint {
            properties.push(("skip-taskbar-hint", skip_taskbar_hint));
        }
        if let Some(ref startup_id) = self.startup_id {
            properties.push(("startup-id", startup_id));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
        }
        if let Some(ref type_) = self.type_ {
            properties.push(("type", type_));
        }
        if let Some(ref type_hint) = self.type_hint {
            properties.push(("type-hint", type_hint));
        }
        if let Some(ref urgency_hint) = self.urgency_hint {
            properties.push(("urgency-hint", urgency_hint));
        }
        if let Some(ref window_position) = self.window_position {
            properties.push(("window-position", window_position));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
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
        glib::Object::new::<Assistant>(&properties)
    }

    pub fn use_header_bar(mut self, use_header_bar: i32) -> Self {
        self.use_header_bar = Some(use_header_bar);
        self
    }

    pub fn accept_focus(mut self, accept_focus: bool) -> Self {
        self.accept_focus = Some(accept_focus);
        self
    }

    pub fn application(mut self, application: &impl IsA<Application>) -> Self {
        self.application = Some(application.clone().upcast());
        self
    }

    pub fn attached_to(mut self, attached_to: &impl IsA<Widget>) -> Self {
        self.attached_to = Some(attached_to.clone().upcast());
        self
    }

    pub fn decorated(mut self, decorated: bool) -> Self {
        self.decorated = Some(decorated);
        self
    }

    pub fn default_height(mut self, default_height: i32) -> Self {
        self.default_height = Some(default_height);
        self
    }

    pub fn default_width(mut self, default_width: i32) -> Self {
        self.default_width = Some(default_width);
        self
    }

    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = Some(deletable);
        self
    }

    pub fn destroy_with_parent(mut self, destroy_with_parent: bool) -> Self {
        self.destroy_with_parent = Some(destroy_with_parent);
        self
    }

    pub fn focus_on_map(mut self, focus_on_map: bool) -> Self {
        self.focus_on_map = Some(focus_on_map);
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.focus_visible = Some(focus_visible);
        self
    }

    pub fn gravity(mut self, gravity: gdk::Gravity) -> Self {
        self.gravity = Some(gravity);
        self
    }

    pub fn hide_titlebar_when_maximized(mut self, hide_titlebar_when_maximized: bool) -> Self {
        self.hide_titlebar_when_maximized = Some(hide_titlebar_when_maximized);
        self
    }

    pub fn icon(mut self, icon: &gdk_pixbuf::Pixbuf) -> Self {
        self.icon = Some(icon.clone());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn mnemonics_visible(mut self, mnemonics_visible: bool) -> Self {
        self.mnemonics_visible = Some(mnemonics_visible);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    pub fn screen(mut self, screen: &gdk::Screen) -> Self {
        self.screen = Some(screen.clone());
        self
    }

    pub fn skip_pager_hint(mut self, skip_pager_hint: bool) -> Self {
        self.skip_pager_hint = Some(skip_pager_hint);
        self
    }

    pub fn skip_taskbar_hint(mut self, skip_taskbar_hint: bool) -> Self {
        self.skip_taskbar_hint = Some(skip_taskbar_hint);
        self
    }

    pub fn startup_id(mut self, startup_id: &str) -> Self {
        self.startup_id = Some(startup_id.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn transient_for(mut self, transient_for: &impl IsA<Window>) -> Self {
        self.transient_for = Some(transient_for.clone().upcast());
        self
    }

    pub fn type_(mut self, type_: WindowType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn type_hint(mut self, type_hint: gdk::WindowTypeHint) -> Self {
        self.type_hint = Some(type_hint);
        self
    }

    pub fn urgency_hint(mut self, urgency_hint: bool) -> Self {
        self.urgency_hint = Some(urgency_hint);
        self
    }

    pub fn window_position(mut self, window_position: WindowPosition) -> Self {
        self.window_position = Some(window_position);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &impl IsA<Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
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
}

pub trait AssistantExt: 'static {
    #[doc(alias = "gtk_assistant_add_action_widget")]
    fn add_action_widget(&self, child: &impl IsA<Widget>);

    #[doc(alias = "gtk_assistant_append_page")]
    fn append_page(&self, page: &impl IsA<Widget>) -> i32;

    #[doc(alias = "gtk_assistant_commit")]
    fn commit(&self);

    #[doc(alias = "gtk_assistant_get_current_page")]
    #[doc(alias = "get_current_page")]
    fn current_page(&self) -> i32;

    #[doc(alias = "gtk_assistant_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    fn n_pages(&self) -> i32;

    #[doc(alias = "gtk_assistant_get_nth_page")]
    #[doc(alias = "get_nth_page")]
    fn nth_page(&self, page_num: i32) -> Option<Widget>;

    #[doc(alias = "gtk_assistant_get_page_complete")]
    #[doc(alias = "get_page_complete")]
    fn page_is_complete(&self, page: &impl IsA<Widget>) -> bool;

    #[doc(alias = "gtk_assistant_get_page_has_padding")]
    #[doc(alias = "get_page_has_padding")]
    fn page_has_padding(&self, page: &impl IsA<Widget>) -> bool;

    #[doc(alias = "gtk_assistant_get_page_title")]
    #[doc(alias = "get_page_title")]
    fn page_title(&self, page: &impl IsA<Widget>) -> Option<glib::GString>;

    #[doc(alias = "gtk_assistant_get_page_type")]
    #[doc(alias = "get_page_type")]
    fn page_type(&self, page: &impl IsA<Widget>) -> AssistantPageType;

    #[doc(alias = "gtk_assistant_insert_page")]
    fn insert_page(&self, page: &impl IsA<Widget>, position: i32) -> i32;

    #[doc(alias = "gtk_assistant_next_page")]
    fn next_page(&self);

    #[doc(alias = "gtk_assistant_prepend_page")]
    fn prepend_page(&self, page: &impl IsA<Widget>) -> i32;

    #[doc(alias = "gtk_assistant_previous_page")]
    fn previous_page(&self);

    #[doc(alias = "gtk_assistant_remove_action_widget")]
    fn remove_action_widget(&self, child: &impl IsA<Widget>);

    #[doc(alias = "gtk_assistant_remove_page")]
    fn remove_page(&self, page_num: i32);

    #[doc(alias = "gtk_assistant_set_current_page")]
    fn set_current_page(&self, page_num: i32);

    #[doc(alias = "gtk_assistant_set_forward_page_func")]
    fn set_forward_page_func(&self, page_func: Option<Box_<dyn Fn(i32) -> i32 + 'static>>);

    #[doc(alias = "gtk_assistant_set_page_complete")]
    fn set_page_complete(&self, page: &impl IsA<Widget>, complete: bool);

    #[doc(alias = "gtk_assistant_set_page_has_padding")]
    fn set_page_has_padding(&self, page: &impl IsA<Widget>, has_padding: bool);

    #[doc(alias = "gtk_assistant_set_page_title")]
    fn set_page_title(&self, page: &impl IsA<Widget>, title: &str);

    #[doc(alias = "gtk_assistant_set_page_type")]
    fn set_page_type(&self, page: &impl IsA<Widget>, type_: AssistantPageType);

    #[doc(alias = "gtk_assistant_update_buttons_state")]
    fn update_buttons_state(&self);

    #[doc(alias = "use-header-bar")]
    fn use_header_bar(&self) -> i32;

    fn child_is_complete<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    fn set_child_complete<T: IsA<crate::Widget>>(&self, item: &T, complete: bool);

    #[doc(alias = "child.has-padding")]
    fn child_has_padding<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    #[doc(alias = "child.has-padding")]
    fn set_child_has_padding<T: IsA<crate::Widget>>(&self, item: &T, has_padding: bool);

    #[doc(alias = "child.page-type")]
    fn child_page_type<T: IsA<crate::Widget>>(&self, item: &T) -> AssistantPageType;

    #[doc(alias = "child.page-type")]
    fn set_child_page_type<T: IsA<crate::Widget>>(&self, item: &T, page_type: AssistantPageType);

    fn child_title<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString>;

    fn set_child_title<T: IsA<crate::Widget>>(&self, item: &T, title: Option<&str>);

    #[doc(alias = "apply")]
    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "cancel")]
    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "close")]
    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "escape")]
    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_escape(&self);

    #[doc(alias = "prepare")]
    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Assistant>> AssistantExt for O {
    fn add_action_widget(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_assistant_add_action_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn append_page(&self, page: &impl IsA<Widget>) -> i32 {
        unsafe {
            ffi::gtk_assistant_append_page(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            )
        }
    }

    fn commit(&self) {
        unsafe {
            ffi::gtk_assistant_commit(self.as_ref().to_glib_none().0);
        }
    }

    fn current_page(&self) -> i32 {
        unsafe { ffi::gtk_assistant_get_current_page(self.as_ref().to_glib_none().0) }
    }

    fn n_pages(&self) -> i32 {
        unsafe { ffi::gtk_assistant_get_n_pages(self.as_ref().to_glib_none().0) }
    }

    fn nth_page(&self, page_num: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_nth_page(
                self.as_ref().to_glib_none().0,
                page_num,
            ))
        }
    }

    fn page_is_complete(&self, page: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_complete(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    fn page_has_padding(&self, page: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_has_padding(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    fn page_title(&self, page: &impl IsA<Widget>) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_assistant_get_page_title(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    fn page_type(&self, page: &impl IsA<Widget>) -> AssistantPageType {
        unsafe {
            from_glib(ffi::gtk_assistant_get_page_type(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert_page(&self, page: &impl IsA<Widget>, position: i32) -> i32 {
        unsafe {
            ffi::gtk_assistant_insert_page(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                position,
            )
        }
    }

    fn next_page(&self) {
        unsafe {
            ffi::gtk_assistant_next_page(self.as_ref().to_glib_none().0);
        }
    }

    fn prepend_page(&self, page: &impl IsA<Widget>) -> i32 {
        unsafe {
            ffi::gtk_assistant_prepend_page(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
            )
        }
    }

    fn previous_page(&self) {
        unsafe {
            ffi::gtk_assistant_previous_page(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_action_widget(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_assistant_remove_action_widget(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_remove_page(self.as_ref().to_glib_none().0, page_num);
        }
    }

    fn set_current_page(&self, page_num: i32) {
        unsafe {
            ffi::gtk_assistant_set_current_page(self.as_ref().to_glib_none().0, page_num);
        }
    }

    fn set_forward_page_func(&self, page_func: Option<Box_<dyn Fn(i32) -> i32 + 'static>>) {
        let page_func_data: Box_<Option<Box_<dyn Fn(i32) -> i32 + 'static>>> = Box_::new(page_func);
        unsafe extern "C" fn page_func_func(
            current_page: libc::c_int,
            data: glib::ffi::gpointer,
        ) -> libc::c_int {
            let callback: &Option<Box_<dyn Fn(i32) -> i32 + 'static>> = &*(data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(current_page)
            } else {
                panic!("cannot get closure...")
            };
            res
        }
        let page_func = if page_func_data.is_some() {
            Some(page_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib::ffi::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(i32) -> i32 + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(i32) -> i32 + 'static>>> = page_func_data;
        unsafe {
            ffi::gtk_assistant_set_forward_page_func(
                self.as_ref().to_glib_none().0,
                page_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_page_complete(&self, page: &impl IsA<Widget>, complete: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_complete(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                complete.into_glib(),
            );
        }
    }

    fn set_page_has_padding(&self, page: &impl IsA<Widget>, has_padding: bool) {
        unsafe {
            ffi::gtk_assistant_set_page_has_padding(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                has_padding.into_glib(),
            );
        }
    }

    fn set_page_title(&self, page: &impl IsA<Widget>, title: &str) {
        unsafe {
            ffi::gtk_assistant_set_page_title(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn set_page_type(&self, page: &impl IsA<Widget>, type_: AssistantPageType) {
        unsafe {
            ffi::gtk_assistant_set_page_type(
                self.as_ref().to_glib_none().0,
                page.as_ref().to_glib_none().0,
                type_.into_glib(),
            );
        }
    }

    fn update_buttons_state(&self) {
        unsafe {
            ffi::gtk_assistant_update_buttons_state(self.as_ref().to_glib_none().0);
        }
    }

    fn use_header_bar(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "use-header-bar")
    }

    fn child_is_complete<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "complete",
        )
    }

    fn set_child_complete<T: IsA<crate::Widget>>(&self, item: &T, complete: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "complete",
            &complete,
        )
    }

    fn child_has_padding<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "has-padding",
        )
    }

    fn set_child_has_padding<T: IsA<crate::Widget>>(&self, item: &T, has_padding: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "has-padding",
            &has_padding,
        )
    }

    fn child_page_type<T: IsA<crate::Widget>>(&self, item: &T) -> AssistantPageType {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "page-type",
        )
    }

    fn set_child_page_type<T: IsA<crate::Widget>>(&self, item: &T, page_type: AssistantPageType) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "page-type",
            &page_type,
        )
    }

    fn child_title<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString> {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "title",
        )
    }

    fn set_child_title<T: IsA<crate::Widget>>(&self, item: &T, title: Option<&str>) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "title",
            &title,
        )
    }

    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn apply_trampoline<P: IsA<Assistant>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAssistant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"apply\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    apply_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<P: IsA<Assistant>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAssistant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<P: IsA<Assistant>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAssistant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    close_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_escape<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn escape_trampoline<P: IsA<Assistant>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAssistant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Assistant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"escape\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    escape_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_escape(&self) {
        self.emit_by_name::<()>("escape", &[]);
    }

    fn connect_prepare<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prepare_trampoline<P: IsA<Assistant>, F: Fn(&P, &Widget) + 'static>(
            this: *mut ffi::GtkAssistant,
            page: *mut ffi::GtkWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Assistant::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(page),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prepare\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prepare_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Assistant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Assistant")
    }
}
