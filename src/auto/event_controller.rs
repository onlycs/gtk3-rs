// This file was generated by gir (14876d3) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_14")]
use PropagationPhase;
#[cfg(feature = "v3_14")]
use Widget;
use ffi;
#[cfg(feature = "v3_14")]
use gdk;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct EventController(Object<ffi::GtkEventController>);

    match fn {
        get_type => || ffi::gtk_event_controller_get_type(),
    }
}

pub trait EventControllerExt {
    #[cfg(feature = "v3_14")]
    fn get_propagation_phase(&self) -> PropagationPhase;

    #[cfg(feature = "v3_14")]
    fn get_widget(&self) -> Option<Widget>;

    #[cfg(feature = "v3_14")]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[cfg(feature = "v3_14")]
    fn reset(&self);

    #[cfg(feature = "v3_14")]
    fn set_propagation_phase(&self, phase: PropagationPhase);
}

impl<O: IsA<EventController>> EventControllerExt for O {
    #[cfg(feature = "v3_14")]
    fn get_propagation_phase(&self) -> PropagationPhase {
        unsafe {
            from_glib(ffi::gtk_event_controller_get_propagation_phase(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_widget(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_handle_event(self.to_glib_none().0, event.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn reset(&self) {
        unsafe {
            ffi::gtk_event_controller_reset(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_propagation_phase(&self, phase: PropagationPhase) {
        unsafe {
            ffi::gtk_event_controller_set_propagation_phase(self.to_glib_none().0, phase.to_glib());
        }
    }
}
