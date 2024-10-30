// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{CellRenderer,CellRendererMode,Orientable,Orientation};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkCellRendererProgress")]
    pub struct CellRendererProgress(Object<ffi::GtkCellRendererProgress, ffi::GtkCellRendererProgressClass>) @extends CellRenderer, @implements Orientable;

    match fn {
        type_ => || ffi::gtk_cell_renderer_progress_get_type(),
    }
}

impl CellRendererProgress {
        pub const NONE: Option<&'static CellRendererProgress> = None;
    

    #[doc(alias = "gtk_cell_renderer_progress_new")]
    pub fn new() -> CellRendererProgress {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_progress_new()).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`CellRendererProgress`] objects.
            ///
            /// This method returns an instance of [`CellRendererProgressBuilder`](crate::builders::CellRendererProgressBuilder) which can be used to create [`CellRendererProgress`] objects.
            pub fn builder() -> CellRendererProgressBuilder {
                CellRendererProgressBuilder::new()
            }
        
}

impl Default for CellRendererProgress {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`CellRendererProgress`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererProgressBuilder {
            builder: glib::object::ObjectBuilder<'static, CellRendererProgress>,
        }

        impl CellRendererProgressBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn inverted(self, inverted: bool) -> Self {
                            Self { builder: self.builder.property("inverted", inverted), }
                        }

                            pub fn pulse(self, pulse: i32) -> Self {
                            Self { builder: self.builder.property("pulse", pulse), }
                        }

                            pub fn text(self, text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("text", text.into()), }
                        }

                            pub fn text_xalign(self, text_xalign: f32) -> Self {
                            Self { builder: self.builder.property("text-xalign", text_xalign), }
                        }

                            pub fn text_yalign(self, text_yalign: f32) -> Self {
                            Self { builder: self.builder.property("text-yalign", text_yalign), }
                        }

                            pub fn value(self, value: i32) -> Self {
                            Self { builder: self.builder.property("value", value), }
                        }

                            pub fn cell_background(self, cell_background: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("cell-background", cell_background.into()), }
                        }

                            pub fn cell_background_rgba(self, cell_background_rgba: &gdk::RGBA) -> Self {
                            Self { builder: self.builder.property("cell-background-rgba", cell_background_rgba), }
                        }

                            pub fn cell_background_set(self, cell_background_set: bool) -> Self {
                            Self { builder: self.builder.property("cell-background-set", cell_background_set), }
                        }

                            pub fn height(self, height: i32) -> Self {
                            Self { builder: self.builder.property("height", height), }
                        }

                            pub fn is_expanded(self, is_expanded: bool) -> Self {
                            Self { builder: self.builder.property("is-expanded", is_expanded), }
                        }

                            pub fn is_expander(self, is_expander: bool) -> Self {
                            Self { builder: self.builder.property("is-expander", is_expander), }
                        }

                            pub fn mode(self, mode: CellRendererMode) -> Self {
                            Self { builder: self.builder.property("mode", mode), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width(self, width: i32) -> Self {
                            Self { builder: self.builder.property("width", width), }
                        }

                            pub fn xalign(self, xalign: f32) -> Self {
                            Self { builder: self.builder.property("xalign", xalign), }
                        }

                            pub fn xpad(self, xpad: u32) -> Self {
                            Self { builder: self.builder.property("xpad", xpad), }
                        }

                            pub fn yalign(self, yalign: f32) -> Self {
                            Self { builder: self.builder.property("yalign", yalign), }
                        }

                            pub fn ypad(self, ypad: u32) -> Self {
                            Self { builder: self.builder.property("ypad", ypad), }
                        }

                            pub fn orientation(self, orientation: Orientation) -> Self {
                            Self { builder: self.builder.property("orientation", orientation), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererProgress`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererProgress {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::CellRendererProgress>> Sealed for T {}
}

pub trait CellRendererProgressExt: IsA<CellRendererProgress> + sealed::Sealed + 'static {
    fn is_inverted(&self) -> bool {
        ObjectExt::property(self.as_ref(), "inverted")
    }

    fn set_inverted(&self, inverted: bool) {
        ObjectExt::set_property(self.as_ref(),"inverted", inverted)
    }

    fn pulse(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "pulse")
    }

    fn set_pulse(&self, pulse: i32) {
        ObjectExt::set_property(self.as_ref(),"pulse", pulse)
    }

    fn text(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "text")
    }

    fn set_text(&self, text: Option<&str>) {
        ObjectExt::set_property(self.as_ref(),"text", text)
    }

    #[doc(alias = "text-xalign")]
    fn text_xalign(&self) -> f32 {
        ObjectExt::property(self.as_ref(), "text-xalign")
    }

    #[doc(alias = "text-xalign")]
    fn set_text_xalign(&self, text_xalign: f32) {
        ObjectExt::set_property(self.as_ref(),"text-xalign", text_xalign)
    }

    #[doc(alias = "text-yalign")]
    fn text_yalign(&self) -> f32 {
        ObjectExt::property(self.as_ref(), "text-yalign")
    }

    #[doc(alias = "text-yalign")]
    fn set_text_yalign(&self, text_yalign: f32) {
        ObjectExt::set_property(self.as_ref(),"text-yalign", text_yalign)
    }

    fn value(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "value")
    }

    fn set_value(&self, value: i32) {
        ObjectExt::set_property(self.as_ref(),"value", value)
    }

    #[doc(alias = "inverted")]
    fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<P: IsA<CellRendererProgress>, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CellRendererProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inverted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_inverted_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "pulse")]
    fn connect_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pulse_trampoline<P: IsA<CellRendererProgress>, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CellRendererProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pulse\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_pulse_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P: IsA<CellRendererProgress>, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CellRendererProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_text_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "text-xalign")]
    fn connect_text_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_xalign_trampoline<P: IsA<CellRendererProgress>, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CellRendererProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-xalign\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_text_xalign_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "text-yalign")]
    fn connect_text_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_yalign_trampoline<P: IsA<CellRendererProgress>, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CellRendererProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-yalign\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_text_yalign_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P: IsA<CellRendererProgress>, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CellRendererProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_value_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<CellRendererProgress>> CellRendererProgressExt for O {}
