// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{Actionable,Activatable,Align,Bin,Buildable,Container,ResizeMode,ToolItem,Widget};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkToolButton")]
    pub struct ToolButton(Object<ffi::GtkToolButton, ffi::GtkToolButtonClass>) @extends ToolItem, Bin, Container, Widget, @implements Buildable, Activatable, Actionable;

    match fn {
        type_ => || ffi::gtk_tool_button_get_type(),
    }
}

impl ToolButton {
        pub const NONE: Option<&'static ToolButton> = None;
    

    #[doc(alias = "gtk_tool_button_new")]
    pub fn new(icon_widget: Option<&impl IsA<Widget>>, label: Option<&str>) -> ToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new(icon_widget.map(|p| p.as_ref()).to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`ToolButton`] objects.
            ///
            /// This method returns an instance of [`ToolButtonBuilder`](crate::builders::ToolButtonBuilder) which can be used to create [`ToolButton`] objects.
            pub fn builder() -> ToolButtonBuilder {
                ToolButtonBuilder::new()
            }
        
}

impl Default for ToolButton {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`ToolButton`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToolButtonBuilder {
            builder: glib::object::ObjectBuilder<'static, ToolButton>,
        }

        impl ToolButtonBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("icon-name", icon_name.into()), }
                        }

                            pub fn icon_widget(self, icon_widget: &impl IsA<Widget>) -> Self {
                            Self { builder: self.builder.property("icon-widget", icon_widget.clone().upcast()), }
                        }

                            pub fn label(self, label: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("label", label.into()), }
                        }

                            pub fn label_widget(self, label_widget: &impl IsA<Widget>) -> Self {
                            Self { builder: self.builder.property("label-widget", label_widget.clone().upcast()), }
                        }

                            pub fn use_underline(self, use_underline: bool) -> Self {
                            Self { builder: self.builder.property("use-underline", use_underline), }
                        }

                            pub fn is_important(self, is_important: bool) -> Self {
                            Self { builder: self.builder.property("is-important", is_important), }
                        }

                            pub fn visible_horizontal(self, visible_horizontal: bool) -> Self {
                            Self { builder: self.builder.property("visible-horizontal", visible_horizontal), }
                        }

                            pub fn visible_vertical(self, visible_vertical: bool) -> Self {
                            Self { builder: self.builder.property("visible-vertical", visible_vertical), }
                        }

                            pub fn border_width(self, border_width: u32) -> Self {
                            Self { builder: self.builder.property("border-width", border_width), }
                        }

                            pub fn child(self, child: &impl IsA<Widget>) -> Self {
                            Self { builder: self.builder.property("child", child.clone().upcast()), }
                        }

                            pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
                            Self { builder: self.builder.property("resize-mode", resize_mode), }
                        }

                            pub fn app_paintable(self, app_paintable: bool) -> Self {
                            Self { builder: self.builder.property("app-paintable", app_paintable), }
                        }

                            pub fn can_default(self, can_default: bool) -> Self {
                            Self { builder: self.builder.property("can-default", can_default), }
                        }

                            pub fn can_focus(self, can_focus: bool) -> Self {
                            Self { builder: self.builder.property("can-focus", can_focus), }
                        }

                            pub fn events(self, events: gdk::EventMask) -> Self {
                            Self { builder: self.builder.property("events", events), }
                        }

                            pub fn expand(self, expand: bool) -> Self {
                            Self { builder: self.builder.property("expand", expand), }
                        }

                            pub fn focus_on_click(self, focus_on_click: bool) -> Self {
                            Self { builder: self.builder.property("focus-on-click", focus_on_click), }
                        }

                            pub fn halign(self, halign: Align) -> Self {
                            Self { builder: self.builder.property("halign", halign), }
                        }

                            pub fn has_default(self, has_default: bool) -> Self {
                            Self { builder: self.builder.property("has-default", has_default), }
                        }

                            pub fn has_focus(self, has_focus: bool) -> Self {
                            Self { builder: self.builder.property("has-focus", has_focus), }
                        }

                            pub fn has_tooltip(self, has_tooltip: bool) -> Self {
                            Self { builder: self.builder.property("has-tooltip", has_tooltip), }
                        }

                            pub fn height_request(self, height_request: i32) -> Self {
                            Self { builder: self.builder.property("height-request", height_request), }
                        }

                            pub fn hexpand(self, hexpand: bool) -> Self {
                            Self { builder: self.builder.property("hexpand", hexpand), }
                        }

                            pub fn hexpand_set(self, hexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("hexpand-set", hexpand_set), }
                        }

                            pub fn is_focus(self, is_focus: bool) -> Self {
                            Self { builder: self.builder.property("is-focus", is_focus), }
                        }

                            pub fn margin(self, margin: i32) -> Self {
                            Self { builder: self.builder.property("margin", margin), }
                        }

                            pub fn margin_bottom(self, margin_bottom: i32) -> Self {
                            Self { builder: self.builder.property("margin-bottom", margin_bottom), }
                        }

                            pub fn margin_end(self, margin_end: i32) -> Self {
                            Self { builder: self.builder.property("margin-end", margin_end), }
                        }

                            pub fn margin_start(self, margin_start: i32) -> Self {
                            Self { builder: self.builder.property("margin-start", margin_start), }
                        }

                            pub fn margin_top(self, margin_top: i32) -> Self {
                            Self { builder: self.builder.property("margin-top", margin_top), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn no_show_all(self, no_show_all: bool) -> Self {
                            Self { builder: self.builder.property("no-show-all", no_show_all), }
                        }

                            pub fn opacity(self, opacity: f64) -> Self {
                            Self { builder: self.builder.property("opacity", opacity), }
                        }

                            pub fn parent(self, parent: &impl IsA<Container>) -> Self {
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

                            pub fn receives_default(self, receives_default: bool) -> Self {
                            Self { builder: self.builder.property("receives-default", receives_default), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-markup", tooltip_markup.into()), }
                        }

                            pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-text", tooltip_text.into()), }
                        }

                            pub fn valign(self, valign: Align) -> Self {
                            Self { builder: self.builder.property("valign", valign), }
                        }

                            pub fn vexpand(self, vexpand: bool) -> Self {
                            Self { builder: self.builder.property("vexpand", vexpand), }
                        }

                            pub fn vexpand_set(self, vexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("vexpand-set", vexpand_set), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width_request(self, width_request: i32) -> Self {
                            Self { builder: self.builder.property("width-request", width_request), }
                        }

                            pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("action-name", action_name.into()), }
                        }

                            pub fn action_target(self, action_target: &glib::Variant) -> Self {
                            Self { builder: self.builder.property("action-target", action_target.clone()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`ToolButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ToolButton {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ToolButton>> Sealed for T {}
}

pub trait ToolButtonExt: IsA<ToolButton> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_tool_button_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_tool_button_get_icon_widget")]
    #[doc(alias = "get_icon_widget")]
    fn icon_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_widget(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_tool_button_get_label")]
    #[doc(alias = "get_label")]
    fn label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_tool_button_get_label_widget")]
    #[doc(alias = "get_label_widget")]
    fn label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label_widget(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_tool_button_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    fn uses_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_button_get_use_underline(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_tool_button_set_icon_name")]
    fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_icon_name(self.as_ref().to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tool_button_set_icon_widget")]
    fn set_icon_widget(&self, icon_widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_tool_button_set_icon_widget(self.as_ref().to_glib_none().0, icon_widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tool_button_set_label")]
    fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_tool_button_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tool_button_set_label_widget")]
    fn set_label_widget(&self, label_widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_tool_button_set_label_widget(self.as_ref().to_glib_none().0, label_widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tool_button_set_use_underline")]
    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_tool_button_set_use_underline(self.as_ref().to_glib_none().0, use_underline.into_glib());
        }
    }

    #[doc(alias = "clicked")]
    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clicked_trampoline<P: IsA<ToolButton>, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolButton, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"clicked\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(clicked_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_clicked(&self) {
        self.emit_by_name::<()>("clicked", &[]);
    }

    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P: IsA<ToolButton>, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_icon_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon-widget")]
    fn connect_icon_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_widget_trampoline<P: IsA<ToolButton>, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_icon_widget_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "label")]
    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P: IsA<ToolButton>, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_label_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "label-widget")]
    fn connect_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_widget_trampoline<P: IsA<ToolButton>, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_label_widget_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "use-underline")]
    fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<P: IsA<ToolButton>, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-underline\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_use_underline_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<ToolButton>> ToolButtonExt for O {}
