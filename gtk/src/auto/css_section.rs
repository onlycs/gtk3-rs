// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{CssSectionType};
use glib::{translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CssSection(Shared<ffi::GtkCssSection>);

    match fn {
        ref => |ptr| ffi::gtk_css_section_ref(ptr),
        unref => |ptr| ffi::gtk_css_section_unref(ptr),
        type_ => || ffi::gtk_css_section_get_type(),
    }
}

impl CssSection {
    #[doc(alias = "gtk_css_section_get_end_line")]
    #[doc(alias = "get_end_line")]
    pub fn end_line(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_end_line(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gtk_css_section_get_end_position")]
    #[doc(alias = "get_end_position")]
    pub fn end_position(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_end_position(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gtk_css_section_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_css_section_get_file(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_css_section_get_parent")]
    #[doc(alias = "get_parent")]
#[must_use]
    pub fn parent(&self) -> Option<CssSection> {
        unsafe {
            from_glib_none(ffi::gtk_css_section_get_parent(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_css_section_get_section_type")]
    #[doc(alias = "get_section_type")]
    pub fn section_type(&self) -> CssSectionType {
        unsafe {
            from_glib(ffi::gtk_css_section_get_section_type(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_css_section_get_start_line")]
    #[doc(alias = "get_start_line")]
    pub fn start_line(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_start_line(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gtk_css_section_get_start_position")]
    #[doc(alias = "get_start_position")]
    pub fn start_position(&self) -> u32 {
        unsafe {
            ffi::gtk_css_section_get_start_position(self.to_glib_none().0)
        }
    }
}
