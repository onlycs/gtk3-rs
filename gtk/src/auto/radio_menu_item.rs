// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{Actionable,Activatable,Align,Bin,Buildable,CheckMenuItem,Container,Menu,MenuItem,ResizeMode,Widget};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkRadioMenuItem")]
    pub struct RadioMenuItem(Object<ffi::GtkRadioMenuItem, ffi::GtkRadioMenuItemClass>) @extends CheckMenuItem, MenuItem, Bin, Container, Widget, @implements Buildable, Actionable, Activatable;

    match fn {
        type_ => || ffi::gtk_radio_menu_item_get_type(),
    }
}

impl RadioMenuItem {
        pub const NONE: Option<&'static RadioMenuItem> = None;
    

    #[doc(alias = "gtk_radio_menu_item_new_from_widget")]
    #[doc(alias = "new_from_widget")]
    pub fn from_widget(group: &impl IsA<RadioMenuItem>) -> RadioMenuItem {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_from_widget(group.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_radio_menu_item_new_with_label_from_widget")]
    #[doc(alias = "new_with_label_from_widget")]
    pub fn with_label_from_widget(group: &impl IsA<RadioMenuItem>, label: Option<&str>) -> RadioMenuItem {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_label_from_widget(group.as_ref().to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_radio_menu_item_new_with_mnemonic_from_widget")]
    #[doc(alias = "new_with_mnemonic_from_widget")]
    pub fn with_mnemonic_from_widget(group: &impl IsA<RadioMenuItem>, label: Option<&str>) -> RadioMenuItem {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_mnemonic_from_widget(group.as_ref().to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`RadioMenuItem`] objects.
            ///
            /// This method returns an instance of [`RadioMenuItemBuilder`](crate::builders::RadioMenuItemBuilder) which can be used to create [`RadioMenuItem`] objects.
            pub fn builder() -> RadioMenuItemBuilder {
                RadioMenuItemBuilder::new()
            }
        
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`RadioMenuItem`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct RadioMenuItemBuilder {
            builder: glib::object::ObjectBuilder<'static, RadioMenuItem>,
        }

        impl RadioMenuItemBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn active(self, active: bool) -> Self {
                            Self { builder: self.builder.property("active", active), }
                        }

                            pub fn draw_as_radio(self, draw_as_radio: bool) -> Self {
                            Self { builder: self.builder.property("draw-as-radio", draw_as_radio), }
                        }

                            pub fn inconsistent(self, inconsistent: bool) -> Self {
                            Self { builder: self.builder.property("inconsistent", inconsistent), }
                        }

                            pub fn accel_path(self, accel_path: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("accel-path", accel_path.into()), }
                        }

                            pub fn label(self, label: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("label", label.into()), }
                        }

                            pub fn right_justified(self, right_justified: bool) -> Self {
                            Self { builder: self.builder.property("right-justified", right_justified), }
                        }

                            pub fn submenu(self, submenu: &impl IsA<Menu>) -> Self {
                            Self { builder: self.builder.property("submenu", submenu.clone().upcast()), }
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
    /// Build the [`RadioMenuItem`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> RadioMenuItem {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RadioMenuItem>> Sealed for T {}
}

pub trait RadioMenuItemExt: IsA<RadioMenuItem> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_radio_menu_item_get_group")]
    #[doc(alias = "get_group")]
    fn group(&self) -> Vec<RadioMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_menu_item_get_group(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_radio_menu_item_join_group")]
    fn join_group(&self, group_source: Option<&impl IsA<RadioMenuItem>>) {
        unsafe {
            ffi::gtk_radio_menu_item_join_group(self.as_ref().to_glib_none().0, group_source.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "group-changed")]
    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn group_changed_trampoline<P: IsA<RadioMenuItem>, F: Fn(&P) + 'static>(this: *mut ffi::GtkRadioMenuItem, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(RadioMenuItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"group-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(group_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<RadioMenuItem>> RadioMenuItemExt for O {}
