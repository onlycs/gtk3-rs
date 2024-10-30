// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{EventController,Gesture,GestureDrag,GestureSingle,Orientation,PanDirection,PropagationPhase,Widget};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkGesturePan")]
    pub struct GesturePan(Object<ffi::GtkGesturePan, ffi::GtkGesturePanClass>) @extends GestureDrag, GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_pan_get_type(),
    }
}

impl GesturePan {
    #[doc(alias = "gtk_gesture_pan_new")]
    pub fn new(widget: &impl IsA<Widget>, orientation: Orientation) -> GesturePan {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_pan_new(widget.as_ref().to_glib_none().0, orientation.into_glib())).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`GesturePan`] objects.
            ///
            /// This method returns an instance of [`GesturePanBuilder`](crate::builders::GesturePanBuilder) which can be used to create [`GesturePan`] objects.
            pub fn builder() -> GesturePanBuilder {
                GesturePanBuilder::new()
            }
        

    #[doc(alias = "gtk_gesture_pan_get_orientation")]
    #[doc(alias = "get_orientation")]
    pub fn orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_gesture_pan_get_orientation(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_gesture_pan_set_orientation")]
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::gtk_gesture_pan_set_orientation(self.to_glib_none().0, orientation.into_glib());
        }
    }

    #[doc(alias = "pan")]
    pub fn connect_pan<F: Fn(&Self, PanDirection, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pan_trampoline<F: Fn(&GesturePan, PanDirection, f64) + 'static>(this: *mut ffi::GtkGesturePan, direction: ffi::GtkPanDirection, offset: libc::c_double, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(direction), offset)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pan\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(pan_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "orientation")]
    pub fn connect_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_orientation_trampoline<F: Fn(&GesturePan) + 'static>(this: *mut ffi::GtkGesturePan, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::orientation\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_orientation_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for GesturePan {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`GesturePan`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GesturePanBuilder {
            builder: glib::object::ObjectBuilder<'static, GesturePan>,
        }

        impl GesturePanBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn orientation(self, orientation: Orientation) -> Self {
                            Self { builder: self.builder.property("orientation", orientation), }
                        }

                            pub fn button(self, button: u32) -> Self {
                            Self { builder: self.builder.property("button", button), }
                        }

                            pub fn exclusive(self, exclusive: bool) -> Self {
                            Self { builder: self.builder.property("exclusive", exclusive), }
                        }

                            pub fn touch_only(self, touch_only: bool) -> Self {
                            Self { builder: self.builder.property("touch-only", touch_only), }
                        }

                            pub fn n_points(self, n_points: u32) -> Self {
                            Self { builder: self.builder.property("n-points", n_points), }
                        }

                            pub fn window(self, window: &gdk::Window) -> Self {
                            Self { builder: self.builder.property("window", window.clone()), }
                        }

                            pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
                            Self { builder: self.builder.property("propagation-phase", propagation_phase), }
                        }

                            pub fn widget(self, widget: &impl IsA<Widget>) -> Self {
                            Self { builder: self.builder.property("widget", widget.clone().upcast()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`GesturePan`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GesturePan {
    self.builder.build() }
}
