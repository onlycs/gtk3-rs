// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::ResizeMode;
use crate::StackTransitionType;
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
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkStack")]
    pub struct Stack(Object<ffi::GtkStack, ffi::GtkStackClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_stack_get_type(),
    }
}

impl Stack {
    pub const NONE: Option<&'static Stack> = None;

    #[doc(alias = "gtk_stack_new")]
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_stack_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Stack`] objects.
    ///
    /// This method returns an instance of [`StackBuilder`](crate::builders::StackBuilder) which can be used to create [`Stack`] objects.
    pub fn builder() -> StackBuilder {
        StackBuilder::default()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Stack`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct StackBuilder {
    hhomogeneous: Option<bool>,
    homogeneous: Option<bool>,
    interpolate_size: Option<bool>,
    transition_duration: Option<u32>,
    transition_type: Option<StackTransitionType>,
    vhomogeneous: Option<bool>,
    visible_child: Option<Widget>,
    visible_child_name: Option<String>,
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

impl StackBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StackBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Stack`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Stack {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref hhomogeneous) = self.hhomogeneous {
            properties.push(("hhomogeneous", hhomogeneous));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref interpolate_size) = self.interpolate_size {
            properties.push(("interpolate-size", interpolate_size));
        }
        if let Some(ref transition_duration) = self.transition_duration {
            properties.push(("transition-duration", transition_duration));
        }
        if let Some(ref transition_type) = self.transition_type {
            properties.push(("transition-type", transition_type));
        }
        if let Some(ref vhomogeneous) = self.vhomogeneous {
            properties.push(("vhomogeneous", vhomogeneous));
        }
        if let Some(ref visible_child) = self.visible_child {
            properties.push(("visible-child", visible_child));
        }
        if let Some(ref visible_child_name) = self.visible_child_name {
            properties.push(("visible-child-name", visible_child_name));
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
        glib::Object::new::<Stack>(&properties).expect("Failed to create an instance of Stack")
    }

    pub fn hhomogeneous(mut self, hhomogeneous: bool) -> Self {
        self.hhomogeneous = Some(hhomogeneous);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn interpolate_size(mut self, interpolate_size: bool) -> Self {
        self.interpolate_size = Some(interpolate_size);
        self
    }

    pub fn transition_duration(mut self, transition_duration: u32) -> Self {
        self.transition_duration = Some(transition_duration);
        self
    }

    pub fn transition_type(mut self, transition_type: StackTransitionType) -> Self {
        self.transition_type = Some(transition_type);
        self
    }

    pub fn vhomogeneous(mut self, vhomogeneous: bool) -> Self {
        self.vhomogeneous = Some(vhomogeneous);
        self
    }

    pub fn visible_child(mut self, visible_child: &impl IsA<Widget>) -> Self {
        self.visible_child = Some(visible_child.clone().upcast());
        self
    }

    pub fn visible_child_name(mut self, visible_child_name: &str) -> Self {
        self.visible_child_name = Some(visible_child_name.to_string());
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

pub trait StackExt: 'static {
    #[doc(alias = "gtk_stack_add_named")]
    fn add_named(&self, child: &impl IsA<Widget>, name: &str);

    #[doc(alias = "gtk_stack_add_titled")]
    fn add_titled(&self, child: &impl IsA<Widget>, name: &str, title: &str);

    #[doc(alias = "gtk_stack_get_child_by_name")]
    #[doc(alias = "get_child_by_name")]
    fn child_by_name(&self, name: &str) -> Option<Widget>;

    #[doc(alias = "gtk_stack_get_hhomogeneous")]
    #[doc(alias = "get_hhomogeneous")]
    fn is_hhomogeneous(&self) -> bool;

    #[doc(alias = "gtk_stack_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    fn is_homogeneous(&self) -> bool;

    #[doc(alias = "gtk_stack_get_interpolate_size")]
    #[doc(alias = "get_interpolate_size")]
    fn interpolates_size(&self) -> bool;

    #[doc(alias = "gtk_stack_get_transition_duration")]
    #[doc(alias = "get_transition_duration")]
    fn transition_duration(&self) -> u32;

    #[doc(alias = "gtk_stack_get_transition_running")]
    #[doc(alias = "get_transition_running")]
    fn is_transition_running(&self) -> bool;

    #[doc(alias = "gtk_stack_get_transition_type")]
    #[doc(alias = "get_transition_type")]
    fn transition_type(&self) -> StackTransitionType;

    #[doc(alias = "gtk_stack_get_vhomogeneous")]
    #[doc(alias = "get_vhomogeneous")]
    fn is_vhomogeneous(&self) -> bool;

    #[doc(alias = "gtk_stack_get_visible_child")]
    #[doc(alias = "get_visible_child")]
    fn visible_child(&self) -> Option<Widget>;

    #[doc(alias = "gtk_stack_get_visible_child_name")]
    #[doc(alias = "get_visible_child_name")]
    fn visible_child_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_stack_set_hhomogeneous")]
    fn set_hhomogeneous(&self, hhomogeneous: bool);

    #[doc(alias = "gtk_stack_set_homogeneous")]
    fn set_homogeneous(&self, homogeneous: bool);

    #[doc(alias = "gtk_stack_set_interpolate_size")]
    fn set_interpolate_size(&self, interpolate_size: bool);

    #[doc(alias = "gtk_stack_set_transition_duration")]
    fn set_transition_duration(&self, duration: u32);

    #[doc(alias = "gtk_stack_set_transition_type")]
    fn set_transition_type(&self, transition: StackTransitionType);

    #[doc(alias = "gtk_stack_set_vhomogeneous")]
    fn set_vhomogeneous(&self, vhomogeneous: bool);

    #[doc(alias = "gtk_stack_set_visible_child")]
    fn set_visible_child(&self, child: &impl IsA<Widget>);

    #[doc(alias = "gtk_stack_set_visible_child_full")]
    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType);

    #[doc(alias = "gtk_stack_set_visible_child_name")]
    fn set_visible_child_name(&self, name: &str);

    #[doc(alias = "child.icon-name")]
    fn child_icon_name<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString>;

    #[doc(alias = "child.icon-name")]
    fn set_child_icon_name<T: IsA<crate::Widget>>(&self, item: &T, icon_name: Option<&str>);

    fn child_name<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString>;

    fn set_child_name<T: IsA<crate::Widget>>(&self, item: &T, name: Option<&str>);

    #[doc(alias = "child.needs-attention")]
    fn child_needs_attention<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    #[doc(alias = "child.needs-attention")]
    fn set_child_needs_attention<T: IsA<crate::Widget>>(&self, item: &T, needs_attention: bool);

    fn child_position<T: IsA<crate::Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<crate::Widget>>(&self, item: &T, position: i32);

    fn child_title<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString>;

    fn set_child_title<T: IsA<crate::Widget>>(&self, item: &T, title: Option<&str>);

    #[doc(alias = "hhomogeneous")]
    fn connect_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "homogeneous")]
    fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "interpolate-size")]
    fn connect_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "transition-duration")]
    fn connect_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "transition-running")]
    fn connect_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "transition-type")]
    fn connect_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "vhomogeneous")]
    fn connect_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-child")]
    fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-child-name")]
    fn connect_visible_child_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Stack>> StackExt for O {
    fn add_named(&self, child: &impl IsA<Widget>, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn add_titled(&self, child: &impl IsA<Widget>, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_child_by_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn is_hhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_hhomogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn interpolates_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_interpolate_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn transition_duration(&self) -> u32 {
        unsafe { ffi::gtk_stack_get_transition_duration(self.as_ref().to_glib_none().0) }
    }

    fn is_transition_running(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_running(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn transition_type(&self) -> StackTransitionType {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_vhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_vhomogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn visible_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn visible_child_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_hhomogeneous(
                self.as_ref().to_glib_none().0,
                hhomogeneous.into_glib(),
            );
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(self.as_ref().to_glib_none().0, homogeneous.into_glib());
        }
    }

    fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            ffi::gtk_stack_set_interpolate_size(
                self.as_ref().to_glib_none().0,
                interpolate_size.into_glib(),
            );
        }
    }

    fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(self.as_ref().to_glib_none().0, duration);
        }
    }

    fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(
                self.as_ref().to_glib_none().0,
                transition.into_glib(),
            );
        }
    }

    fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_vhomogeneous(
                self.as_ref().to_glib_none().0,
                vhomogeneous.into_glib(),
            );
        }
    }

    fn set_visible_child(&self, child: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_stack_set_visible_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                transition.into_glib(),
            );
        }
    }

    fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn child_icon_name<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString> {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "icon-name",
        )
    }

    fn set_child_icon_name<T: IsA<crate::Widget>>(&self, item: &T, icon_name: Option<&str>) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "icon-name",
            &icon_name,
        )
    }

    fn child_name<T: IsA<crate::Widget>>(&self, item: &T) -> Option<glib::GString> {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "name",
        )
    }

    fn set_child_name<T: IsA<crate::Widget>>(&self, item: &T, name: Option<&str>) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "name",
            &name,
        )
    }

    fn child_needs_attention<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "needs-attention",
        )
    }

    fn set_child_needs_attention<T: IsA<crate::Widget>>(&self, item: &T, needs_attention: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "needs-attention",
            &needs_attention,
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

    fn connect_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hhomogeneous_trampoline<P: IsA<Stack>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hhomogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hhomogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<P: IsA<Stack>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interpolate_size_trampoline<
            P: IsA<Stack>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::interpolate-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interpolate_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_duration_trampoline<
            P: IsA<Stack>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transition-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_running_trampoline<
            P: IsA<Stack>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transition-running\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_running_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_type_trampoline<
            P: IsA<Stack>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transition-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vhomogeneous_trampoline<P: IsA<Stack>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vhomogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vhomogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<P: IsA<Stack>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_visible_child_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_name_trampoline<
            P: IsA<Stack>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkStack,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-child-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_child_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Stack")
    }
}
