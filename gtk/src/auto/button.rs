// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{Actionable,Activatable,Align,Bin,Buildable,Container,IconSize,PositionType,ReliefStyle,ResizeMode,Widget};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkButton")]
    pub struct Button(Object<ffi::GtkButton, ffi::GtkButtonClass>) @extends Bin, Container, Widget, @implements Buildable, Actionable, Activatable;

    match fn {
        type_ => || ffi::gtk_button_get_type(),
    }
}

impl Button {
        pub const NONE: Option<&'static Button> = None;
    

    #[doc(alias = "gtk_button_new")]
    pub fn new() -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new()).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_button_new_from_icon_name")]
    #[doc(alias = "new_from_icon_name")]
    pub fn from_icon_name(icon_name: Option<&str>, size: IconSize) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_from_icon_name(icon_name.to_glib_none().0, size.into_glib())).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_button_new_with_label")]
    #[doc(alias = "new_with_label")]
    pub fn with_label(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_label(label.to_glib_none().0)).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_button_new_with_mnemonic")]
    #[doc(alias = "new_with_mnemonic")]
    pub fn with_mnemonic(label: &str) -> Button {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_new_with_mnemonic(label.to_glib_none().0)).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Button`] objects.
            ///
            /// This method returns an instance of [`ButtonBuilder`](crate::builders::ButtonBuilder) which can be used to create [`Button`] objects.
            pub fn builder() -> ButtonBuilder {
                ButtonBuilder::new()
            }
        
}

impl Default for Button {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Button`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ButtonBuilder {
            builder: glib::object::ObjectBuilder<'static, Button>,
        }

        impl ButtonBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn always_show_image(self, always_show_image: bool) -> Self {
                            Self { builder: self.builder.property("always-show-image", always_show_image), }
                        }

                            pub fn image(self, image: &impl IsA<Widget>) -> Self {
                            Self { builder: self.builder.property("image", image.clone().upcast()), }
                        }

                            pub fn image_position(self, image_position: PositionType) -> Self {
                            Self { builder: self.builder.property("image-position", image_position), }
                        }

                            pub fn label(self, label: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("label", label.into()), }
                        }

                            pub fn relief(self, relief: ReliefStyle) -> Self {
                            Self { builder: self.builder.property("relief", relief), }
                        }

                            pub fn use_underline(self, use_underline: bool) -> Self {
                            Self { builder: self.builder.property("use-underline", use_underline), }
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
    /// Build the [`Button`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Button {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Button>> Sealed for T {}
}

pub trait ButtonExt: IsA<Button> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_button_clicked")]
    fn clicked(&self) {
        unsafe {
            ffi::gtk_button_clicked(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_button_get_always_show_image")]
    #[doc(alias = "get_always_show_image")]
    fn must_always_show_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_always_show_image(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_button_get_event_window")]
    #[doc(alias = "get_event_window")]
    fn event_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_event_window(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_button_get_image")]
    #[doc(alias = "get_image")]
    fn image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_image(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_button_get_image_position")]
    #[doc(alias = "get_image_position")]
    fn image_position(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_button_get_image_position(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_button_get_label")]
    #[doc(alias = "get_label")]
    fn label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_button_get_label(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_button_get_relief")]
    #[doc(alias = "get_relief")]
    fn relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_button_get_relief(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_button_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    fn uses_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_get_use_underline(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_button_set_always_show_image")]
    fn set_always_show_image(&self, always_show: bool) {
        unsafe {
            ffi::gtk_button_set_always_show_image(self.as_ref().to_glib_none().0, always_show.into_glib());
        }
    }

    #[doc(alias = "gtk_button_set_image")]
    fn set_image(&self, image: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_button_set_image(self.as_ref().to_glib_none().0, image.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_button_set_image_position")]
    fn set_image_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_button_set_image_position(self.as_ref().to_glib_none().0, position.into_glib());
        }
    }

    #[doc(alias = "gtk_button_set_label")]
    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_button_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_button_set_relief")]
    fn set_relief(&self, relief: ReliefStyle) {
        unsafe {
            ffi::gtk_button_set_relief(self.as_ref().to_glib_none().0, relief.into_glib());
        }
    }

    #[doc(alias = "gtk_button_set_use_underline")]
    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_button_set_use_underline(self.as_ref().to_glib_none().0, use_underline.into_glib());
        }
    }

    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(activate_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "clicked")]
    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clicked_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
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

    #[doc(alias = "always-show-image")]
    fn connect_always_show_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_always_show_image_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::always-show-image\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_always_show_image_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "image")]
    fn connect_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_image_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::image\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_image_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "image-position")]
    fn connect_image_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_image_position_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::image-position\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_image_position_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "label")]
    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_label_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "relief")]
    fn connect_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_relief_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::relief\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_relief_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "use-underline")]
    fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<P: IsA<Button>, F: Fn(&P) + 'static>(this: *mut ffi::GtkButton, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Button::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-underline\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_use_underline_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Button>> ButtonExt for O {}
