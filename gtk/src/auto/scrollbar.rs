// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::Orientable;
use crate::Orientation;
use crate::Range;
use crate::SensitivityType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkScrollbar")]
    pub struct Scrollbar(Object<ffi::GtkScrollbar, ffi::GtkScrollbarClass>) @extends Range, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_scrollbar_get_type(),
    }
}

impl Scrollbar {
    #[doc(alias = "gtk_scrollbar_new")]
    pub fn new(orientation: Orientation, adjustment: Option<&impl IsA<Adjustment>>) -> Scrollbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrollbar_new(
                orientation.into_glib(),
                adjustment.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Scrollbar`] objects.
    ///
    /// This method returns an instance of [`ScrollbarBuilder`] which can be used to create [`Scrollbar`] objects.
    pub fn builder() -> ScrollbarBuilder {
        ScrollbarBuilder::default()
    }
}

impl Default for Scrollbar {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct Scrollbar object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Scrollbar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ScrollbarBuilder {
    adjustment: Option<Adjustment>,
    fill_level: Option<f64>,
    inverted: Option<bool>,
    lower_stepper_sensitivity: Option<SensitivityType>,
    restrict_to_fill_level: Option<bool>,
    round_digits: Option<i32>,
    show_fill_level: Option<bool>,
    upper_stepper_sensitivity: Option<SensitivityType>,
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

impl ScrollbarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ScrollbarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Scrollbar`].
    pub fn build(self) -> Scrollbar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref adjustment) = self.adjustment {
            properties.push(("adjustment", adjustment));
        }
        if let Some(ref fill_level) = self.fill_level {
            properties.push(("fill-level", fill_level));
        }
        if let Some(ref inverted) = self.inverted {
            properties.push(("inverted", inverted));
        }
        if let Some(ref lower_stepper_sensitivity) = self.lower_stepper_sensitivity {
            properties.push(("lower-stepper-sensitivity", lower_stepper_sensitivity));
        }
        if let Some(ref restrict_to_fill_level) = self.restrict_to_fill_level {
            properties.push(("restrict-to-fill-level", restrict_to_fill_level));
        }
        if let Some(ref round_digits) = self.round_digits {
            properties.push(("round-digits", round_digits));
        }
        if let Some(ref show_fill_level) = self.show_fill_level {
            properties.push(("show-fill-level", show_fill_level));
        }
        if let Some(ref upper_stepper_sensitivity) = self.upper_stepper_sensitivity {
            properties.push(("upper-stepper-sensitivity", upper_stepper_sensitivity));
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
        glib::Object::new::<Scrollbar>(&properties)
            .expect("Failed to create an instance of Scrollbar")
    }

    pub fn adjustment(mut self, adjustment: &impl IsA<Adjustment>) -> Self {
        self.adjustment = Some(adjustment.clone().upcast());
        self
    }

    pub fn fill_level(mut self, fill_level: f64) -> Self {
        self.fill_level = Some(fill_level);
        self
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    pub fn lower_stepper_sensitivity(mut self, lower_stepper_sensitivity: SensitivityType) -> Self {
        self.lower_stepper_sensitivity = Some(lower_stepper_sensitivity);
        self
    }

    pub fn restrict_to_fill_level(mut self, restrict_to_fill_level: bool) -> Self {
        self.restrict_to_fill_level = Some(restrict_to_fill_level);
        self
    }

    pub fn round_digits(mut self, round_digits: i32) -> Self {
        self.round_digits = Some(round_digits);
        self
    }

    pub fn show_fill_level(mut self, show_fill_level: bool) -> Self {
        self.show_fill_level = Some(show_fill_level);
        self
    }

    pub fn upper_stepper_sensitivity(mut self, upper_stepper_sensitivity: SensitivityType) -> Self {
        self.upper_stepper_sensitivity = Some(upper_stepper_sensitivity);
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

pub const NONE_SCROLLBAR: Option<&Scrollbar> = None;

impl fmt::Display for Scrollbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Scrollbar")
    }
}
