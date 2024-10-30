// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{Box,Buildable,Container,Orientable,Stack,Widget};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkStackSwitcher")]
    pub struct StackSwitcher(Object<ffi::GtkStackSwitcher, ffi::GtkStackSwitcherClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_stack_switcher_get_type(),
    }
}

impl StackSwitcher {
        pub const NONE: Option<&'static StackSwitcher> = None;
    

    #[doc(alias = "gtk_stack_switcher_new")]
    pub fn new() -> StackSwitcher {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_switcher_new()).unsafe_cast()
        }
    }
}

impl Default for StackSwitcher {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::StackSwitcher>> Sealed for T {}
}

pub trait StackSwitcherExt: IsA<StackSwitcher> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_stack_switcher_get_stack")]
    #[doc(alias = "get_stack")]
    fn stack(&self) -> Option<Stack> {
        unsafe {
            from_glib_none(ffi::gtk_stack_switcher_get_stack(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_stack_switcher_set_stack")]
    fn set_stack(&self, stack: Option<&impl IsA<Stack>>) {
        unsafe {
            ffi::gtk_stack_switcher_set_stack(self.as_ref().to_glib_none().0, stack.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    #[doc(alias = "stack")]
    fn connect_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stack_trampoline<P: IsA<StackSwitcher>, F: Fn(&P) + 'static>(this: *mut ffi::GtkStackSwitcher, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(StackSwitcher::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::stack\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(notify_stack_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<StackSwitcher>> StackSwitcherExt for O {}
