// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Display;
use crate::Rectangle;
use crate::SubpixelLayout;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GdkMonitor")]
    pub struct Monitor(Object<ffi::GdkMonitor, ffi::GdkMonitorClass>);

    match fn {
        type_ => || ffi::gdk_monitor_get_type(),
    }
}

impl Monitor {
    #[doc(alias = "gdk_monitor_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_geometry")]
    #[doc(alias = "get_geometry")]
    pub fn geometry(&self) -> Rectangle {
        unsafe {
            let mut geometry = Rectangle::uninitialized();
            ffi::gdk_monitor_get_geometry(self.to_glib_none().0, geometry.to_glib_none_mut().0);
            geometry
        }
    }

    #[doc(alias = "gdk_monitor_get_height_mm")]
    #[doc(alias = "get_height_mm")]
    pub fn height_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_height_mm(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_manufacturer")]
    #[doc(alias = "get_manufacturer")]
    pub fn manufacturer(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_manufacturer(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_refresh_rate")]
    #[doc(alias = "get_refresh_rate")]
    pub fn refresh_rate(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_refresh_rate(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_scale_factor")]
    #[doc(alias = "get_scale_factor")]
    pub fn scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_scale_factor(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_subpixel_layout")]
    #[doc(alias = "get_subpixel_layout")]
    pub fn subpixel_layout(&self) -> SubpixelLayout {
        unsafe { from_glib(ffi::gdk_monitor_get_subpixel_layout(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_monitor_get_width_mm")]
    #[doc(alias = "get_width_mm")]
    pub fn width_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_width_mm(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_monitor_get_workarea")]
    #[doc(alias = "get_workarea")]
    pub fn workarea(&self) -> Rectangle {
        unsafe {
            let mut workarea = Rectangle::uninitialized();
            ffi::gdk_monitor_get_workarea(self.to_glib_none().0, workarea.to_glib_none_mut().0);
            workarea
        }
    }

    #[doc(alias = "gdk_monitor_is_primary")]
    pub fn is_primary(&self) -> bool {
        unsafe { from_glib(ffi::gdk_monitor_is_primary(self.to_glib_none().0)) }
    }

    #[doc(alias = "invalidate")]
    pub fn connect_invalidate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn invalidate_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"invalidate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    invalidate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "geometry")]
    pub fn connect_geometry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_geometry_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::geometry\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_geometry_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "height-mm")]
    pub fn connect_height_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_mm_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height-mm\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_mm_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "manufacturer")]
    pub fn connect_manufacturer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_manufacturer_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::manufacturer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_manufacturer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "refresh-rate")]
    pub fn connect_refresh_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_refresh_rate_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::refresh-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_refresh_rate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scale-factor")]
    pub fn connect_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "subpixel-layout")]
    pub fn connect_subpixel_layout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subpixel_layout_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subpixel-layout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subpixel_layout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "width-mm")]
    pub fn connect_width_mm_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_mm_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width-mm\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_mm_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "workarea")]
    pub fn connect_workarea_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_workarea_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::workarea\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_workarea_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Monitor")
    }
}
