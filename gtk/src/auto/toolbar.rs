// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::IconSize;
use crate::Orientable;
use crate::Orientation;
use crate::ResizeMode;
use crate::ToolItem;
use crate::ToolShell;
use crate::ToolbarStyle;
use crate::Widget;
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
    #[doc(alias = "GtkToolbar")]
    pub struct Toolbar(Object<ffi::GtkToolbar, ffi::GtkToolbarClass>) @extends Container, Widget, @implements Buildable, Orientable, ToolShell;

    match fn {
        type_ => || ffi::gtk_toolbar_get_type(),
    }
}

impl Toolbar {
    pub const NONE: Option<&'static Toolbar> = None;

    #[doc(alias = "gtk_toolbar_new")]
    pub fn new() -> Toolbar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_toolbar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Toolbar`] objects.
    ///
    /// This method returns an instance of [`ToolbarBuilder`](crate::builders::ToolbarBuilder) which can be used to create [`Toolbar`] objects.
    pub fn builder() -> ToolbarBuilder {
        ToolbarBuilder::default()
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Toolbar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToolbarBuilder {
    icon_size: Option<IconSize>,
    icon_size_set: Option<bool>,
    show_arrow: Option<bool>,
    toolbar_style: Option<ToolbarStyle>,
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
    orientation: Option<Orientation>,
}

impl ToolbarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ToolbarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Toolbar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Toolbar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref icon_size) = self.icon_size {
            properties.push(("icon-size", icon_size));
        }
        if let Some(ref icon_size_set) = self.icon_size_set {
            properties.push(("icon-size-set", icon_size_set));
        }
        if let Some(ref show_arrow) = self.show_arrow {
            properties.push(("show-arrow", show_arrow));
        }
        if let Some(ref toolbar_style) = self.toolbar_style {
            properties.push(("toolbar-style", toolbar_style));
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
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<Toolbar>(&properties).expect("Failed to create an instance of Toolbar")
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn icon_size_set(mut self, icon_size_set: bool) -> Self {
        self.icon_size_set = Some(icon_size_set);
        self
    }

    pub fn show_arrow(mut self, show_arrow: bool) -> Self {
        self.show_arrow = Some(show_arrow);
        self
    }

    pub fn toolbar_style(mut self, toolbar_style: ToolbarStyle) -> Self {
        self.toolbar_style = Some(toolbar_style);
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

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub trait ToolbarExt: 'static {
    #[doc(alias = "gtk_toolbar_get_drop_index")]
    #[doc(alias = "get_drop_index")]
    fn drop_index(&self, x: i32, y: i32) -> i32;

    #[doc(alias = "gtk_toolbar_get_item_index")]
    #[doc(alias = "get_item_index")]
    fn item_index(&self, item: &impl IsA<ToolItem>) -> i32;

    #[doc(alias = "gtk_toolbar_get_n_items")]
    #[doc(alias = "get_n_items")]
    fn n_items(&self) -> i32;

    #[doc(alias = "gtk_toolbar_get_nth_item")]
    #[doc(alias = "get_nth_item")]
    fn nth_item(&self, n: i32) -> Option<ToolItem>;

    #[doc(alias = "gtk_toolbar_get_show_arrow")]
    #[doc(alias = "get_show_arrow")]
    fn shows_arrow(&self) -> bool;

    #[doc(alias = "gtk_toolbar_insert")]
    fn insert(&self, item: &impl IsA<ToolItem>, pos: i32);

    #[doc(alias = "gtk_toolbar_set_drop_highlight_item")]
    fn set_drop_highlight_item(&self, tool_item: Option<&impl IsA<ToolItem>>, index_: i32);

    #[doc(alias = "gtk_toolbar_set_icon_size")]
    fn set_icon_size(&self, icon_size: IconSize);

    #[doc(alias = "gtk_toolbar_set_show_arrow")]
    fn set_show_arrow(&self, show_arrow: bool);

    #[doc(alias = "gtk_toolbar_set_style")]
    fn set_style(&self, style: ToolbarStyle);

    #[doc(alias = "gtk_toolbar_unset_icon_size")]
    fn unset_icon_size(&self);

    #[doc(alias = "gtk_toolbar_unset_style")]
    fn unset_style(&self);

    #[doc(alias = "icon-size-set")]
    fn is_icon_size_set(&self) -> bool;

    #[doc(alias = "icon-size-set")]
    fn set_icon_size_set(&self, icon_size_set: bool);

    #[doc(alias = "toolbar-style")]
    fn toolbar_style(&self) -> ToolbarStyle;

    #[doc(alias = "toolbar-style")]
    fn set_toolbar_style(&self, toolbar_style: ToolbarStyle);

    fn item_expands<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    fn set_item_expand<T: IsA<crate::Widget>>(&self, item: &T, expand: bool);

    fn item_is_homogeneous<T: IsA<crate::Widget>>(&self, item: &T) -> bool;

    fn set_item_homogeneous<T: IsA<crate::Widget>>(&self, item: &T, homogeneous: bool);

    #[doc(alias = "focus-home-or-end")]
    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_focus_home_or_end(&self, focus_home: bool) -> bool;

    #[doc(alias = "orientation-changed")]
    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "popup-context-menu")]
    fn connect_popup_context_menu<F: Fn(&Self, i32, i32, i32) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "style-changed")]
    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon-size")]
    fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon-size-set")]
    fn connect_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-arrow")]
    fn connect_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "toolbar-style")]
    fn connect_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Toolbar>> ToolbarExt for O {
    fn drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe { ffi::gtk_toolbar_get_drop_index(self.as_ref().to_glib_none().0, x, y) }
    }

    fn item_index(&self, item: &impl IsA<ToolItem>) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_item_index(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            )
        }
    }

    fn n_items(&self) -> i32 {
        unsafe { ffi::gtk_toolbar_get_n_items(self.as_ref().to_glib_none().0) }
    }

    fn nth_item(&self, n: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_toolbar_get_nth_item(
                self.as_ref().to_glib_none().0,
                n,
            ))
        }
    }

    fn shows_arrow(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_show_arrow(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert(&self, item: &impl IsA<ToolItem>, pos: i32) {
        unsafe {
            ffi::gtk_toolbar_insert(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                pos,
            );
        }
    }

    fn set_drop_highlight_item(&self, tool_item: Option<&impl IsA<ToolItem>>, index_: i32) {
        unsafe {
            ffi::gtk_toolbar_set_drop_highlight_item(
                self.as_ref().to_glib_none().0,
                tool_item.map(|p| p.as_ref()).to_glib_none().0,
                index_,
            );
        }
    }

    fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_toolbar_set_icon_size(self.as_ref().to_glib_none().0, icon_size.into_glib());
        }
    }

    fn set_show_arrow(&self, show_arrow: bool) {
        unsafe {
            ffi::gtk_toolbar_set_show_arrow(self.as_ref().to_glib_none().0, show_arrow.into_glib());
        }
    }

    fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_toolbar_set_style(self.as_ref().to_glib_none().0, style.into_glib());
        }
    }

    fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_icon_size(self.as_ref().to_glib_none().0);
        }
    }

    fn unset_style(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_style(self.as_ref().to_glib_none().0);
        }
    }

    fn is_icon_size_set(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "icon-size-set")
    }

    fn set_icon_size_set(&self, icon_size_set: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "icon-size-set", &icon_size_set)
    }

    fn toolbar_style(&self) -> ToolbarStyle {
        glib::ObjectExt::property(self.as_ref(), "toolbar-style")
    }

    fn set_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        glib::ObjectExt::set_property(self.as_ref(), "toolbar-style", &toolbar_style)
    }

    fn item_expands<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "expand",
        )
    }

    fn set_item_expand<T: IsA<crate::Widget>>(&self, item: &T, expand: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "expand",
            &expand,
        )
    }

    fn item_is_homogeneous<T: IsA<crate::Widget>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "homogeneous",
        )
    }

    fn set_item_homogeneous<T: IsA<crate::Widget>>(&self, item: &T, homogeneous: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "homogeneous",
            &homogeneous,
        )
    }

    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn focus_home_or_end_trampoline<
            P: IsA<Toolbar>,
            F: Fn(&P, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkToolbar,
            focus_home: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Toolbar::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(focus_home),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"focus-home-or-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    focus_home_or_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_focus_home_or_end(&self, focus_home: bool) -> bool {
        self.emit_by_name("focus-home-or-end", &[&focus_home])
    }

    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn orientation_changed_trampoline<
            P: IsA<Toolbar>,
            F: Fn(&P, Orientation) + 'static,
        >(
            this: *mut ffi::GtkToolbar,
            orientation: ffi::GtkOrientation,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Toolbar::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(orientation),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"orientation-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    orientation_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_popup_context_menu<
        F: Fn(&Self, i32, i32, i32) -> glib::signal::Inhibit + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn popup_context_menu_trampoline<
            P: IsA<Toolbar>,
            F: Fn(&P, i32, i32, i32) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkToolbar,
            x: libc::c_int,
            y: libc::c_int,
            button: libc::c_int,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Toolbar::from_glib_borrow(this).unsafe_cast_ref(),
                x,
                y,
                button,
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popup-context-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    popup_context_menu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn style_changed_trampoline<
            P: IsA<Toolbar>,
            F: Fn(&P, ToolbarStyle) + 'static,
        >(
            this: *mut ffi::GtkToolbar,
            style: ffi::GtkToolbarStyle,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Toolbar::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(style),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"style-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    style_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<P: IsA<Toolbar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkToolbar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toolbar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_set_trampoline<
            P: IsA<Toolbar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolbar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toolbar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-size-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_size_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_arrow_trampoline<P: IsA<Toolbar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkToolbar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toolbar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-arrow\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_arrow_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_toolbar_style_trampoline<
            P: IsA<Toolbar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolbar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toolbar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::toolbar-style\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_toolbar_style_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Toolbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Toolbar")
    }
}
