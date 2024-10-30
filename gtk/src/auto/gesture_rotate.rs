// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{EventController,Gesture,PropagationPhase,Widget};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkGestureRotate")]
    pub struct GestureRotate(Object<ffi::GtkGestureRotate, ffi::GtkGestureRotateClass>) @extends Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_rotate_get_type(),
    }
}

impl GestureRotate {
    #[doc(alias = "gtk_gesture_rotate_new")]
    pub fn new(widget: &impl IsA<Widget>) -> GestureRotate {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_rotate_new(widget.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`GestureRotate`] objects.
            ///
            /// This method returns an instance of [`GestureRotateBuilder`](crate::builders::GestureRotateBuilder) which can be used to create [`GestureRotate`] objects.
            pub fn builder() -> GestureRotateBuilder {
                GestureRotateBuilder::new()
            }
        

    #[doc(alias = "gtk_gesture_rotate_get_angle_delta")]
    #[doc(alias = "get_angle_delta")]
    pub fn angle_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_rotate_get_angle_delta(self.to_glib_none().0)
        }
    }

    #[doc(alias = "angle-changed")]
    pub fn connect_angle_changed<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn angle_changed_trampoline<F: Fn(&GestureRotate, f64, f64) + 'static>(this: *mut ffi::GtkGestureRotate, angle: libc::c_double, angle_delta: libc::c_double, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), angle, angle_delta)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"angle-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(angle_changed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for GestureRotate {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`GestureRotate`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureRotateBuilder {
            builder: glib::object::ObjectBuilder<'static, GestureRotate>,
        }

        impl GestureRotateBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
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
    /// Build the [`GestureRotate`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureRotate {
    self.builder.build() }
}
