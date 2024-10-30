// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{StateFlags,Widget};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WidgetPath(Shared<ffi::GtkWidgetPath>);

    match fn {
        ref => |ptr| ffi::gtk_widget_path_ref(ptr),
        unref => |ptr| ffi::gtk_widget_path_unref(ptr),
        type_ => || ffi::gtk_widget_path_get_type(),
    }
}

impl WidgetPath {
    #[doc(alias = "gtk_widget_path_new")]
    pub fn new() -> WidgetPath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_widget_path_new())
        }
    }

    #[doc(alias = "gtk_widget_path_append_for_widget")]
    pub fn append_for_widget(&self, widget: &impl IsA<Widget>) -> i32 {
        unsafe {
            ffi::gtk_widget_path_append_for_widget(self.to_glib_none().0, widget.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "gtk_widget_path_append_type")]
    pub fn append_type(&self, type_: glib::types::Type) -> i32 {
        unsafe {
            ffi::gtk_widget_path_append_type(self.to_glib_none().0, type_.into_glib())
        }
    }

    #[doc(alias = "gtk_widget_path_append_with_siblings")]
    pub fn append_with_siblings(&self, siblings: &WidgetPath, sibling_index: u32) -> i32 {
        unsafe {
            ffi::gtk_widget_path_append_with_siblings(self.to_glib_none().0, siblings.to_glib_none().0, sibling_index)
        }
    }

    #[doc(alias = "gtk_widget_path_copy")]
#[must_use]
    pub fn copy(&self) -> Option<WidgetPath> {
        unsafe {
            from_glib_full(ffi::gtk_widget_path_copy(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_widget_path_get_object_type")]
    #[doc(alias = "get_object_type")]
    pub fn object_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_widget_path_get_object_type(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_widget_path_has_parent")]
    pub fn has_parent(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_has_parent(self.to_glib_none().0, type_.into_glib()))
        }
    }

    #[doc(alias = "gtk_widget_path_is_type")]
    pub fn is_type(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_is_type(self.to_glib_none().0, type_.into_glib()))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_add_class")]
    pub fn iter_add_class(&self, pos: i32, name: &str) {
        unsafe {
            ffi::gtk_widget_path_iter_add_class(self.to_glib_none().0, pos, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_path_iter_clear_classes")]
    pub fn iter_clear_classes(&self, pos: i32) {
        unsafe {
            ffi::gtk_widget_path_iter_clear_classes(self.to_glib_none().0, pos);
        }
    }

    #[doc(alias = "gtk_widget_path_iter_get_name")]
    pub fn iter_get_name(&self, pos: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_widget_path_iter_get_name(self.to_glib_none().0, pos))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_get_object_name")]
    pub fn iter_get_object_name(&self, pos: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_widget_path_iter_get_object_name(self.to_glib_none().0, pos))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_get_object_type")]
    pub fn iter_get_object_type(&self, pos: i32) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_get_object_type(self.to_glib_none().0, pos))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_get_sibling_index")]
    pub fn iter_get_sibling_index(&self, pos: i32) -> u32 {
        unsafe {
            ffi::gtk_widget_path_iter_get_sibling_index(self.to_glib_none().0, pos)
        }
    }

    #[doc(alias = "gtk_widget_path_iter_get_siblings")]
#[must_use]
    pub fn iter_get_siblings(&self, pos: i32) -> Option<WidgetPath> {
        unsafe {
            from_glib_none(ffi::gtk_widget_path_iter_get_siblings(self.to_glib_none().0, pos))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_get_state")]
    pub fn iter_get_state(&self, pos: i32) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_get_state(self.to_glib_none().0, pos))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_has_class")]
    pub fn iter_has_class(&self, pos: i32, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_class(self.to_glib_none().0, pos, name.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_has_name")]
    pub fn iter_has_name(&self, pos: i32, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_name(self.to_glib_none().0, pos, name.to_glib_none().0))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_has_qclass")]
    pub fn iter_has_qclass(&self, pos: i32, qname: glib::Quark) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_qclass(self.to_glib_none().0, pos, qname.into_glib()))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_has_qname")]
    pub fn iter_has_qname(&self, pos: i32, qname: glib::Quark) -> bool {
        unsafe {
            from_glib(ffi::gtk_widget_path_iter_has_qname(self.to_glib_none().0, pos, qname.into_glib()))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_list_classes")]
    pub fn iter_list_classes(&self, pos: i32) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_widget_path_iter_list_classes(self.to_glib_none().0, pos))
        }
    }

    #[doc(alias = "gtk_widget_path_iter_remove_class")]
    pub fn iter_remove_class(&self, pos: i32, name: &str) {
        unsafe {
            ffi::gtk_widget_path_iter_remove_class(self.to_glib_none().0, pos, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_path_iter_set_name")]
    pub fn iter_set_name(&self, pos: i32, name: &str) {
        unsafe {
            ffi::gtk_widget_path_iter_set_name(self.to_glib_none().0, pos, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_path_iter_set_object_name")]
    pub fn iter_set_object_name(&self, pos: i32, name: Option<&str>) {
        unsafe {
            ffi::gtk_widget_path_iter_set_object_name(self.to_glib_none().0, pos, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_widget_path_iter_set_object_type")]
    pub fn iter_set_object_type(&self, pos: i32, type_: glib::types::Type) {
        unsafe {
            ffi::gtk_widget_path_iter_set_object_type(self.to_glib_none().0, pos, type_.into_glib());
        }
    }

    #[doc(alias = "gtk_widget_path_iter_set_state")]
    pub fn iter_set_state(&self, pos: i32, state: StateFlags) {
        unsafe {
            ffi::gtk_widget_path_iter_set_state(self.to_glib_none().0, pos, state.into_glib());
        }
    }

    #[doc(alias = "gtk_widget_path_length")]
    pub fn length(&self) -> i32 {
        unsafe {
            ffi::gtk_widget_path_length(self.to_glib_none().0)
        }
    }

    #[doc(alias = "gtk_widget_path_prepend_type")]
    pub fn prepend_type(&self, type_: glib::types::Type) {
        unsafe {
            ffi::gtk_widget_path_prepend_type(self.to_glib_none().0, type_.into_glib());
        }
    }

    #[doc(alias = "gtk_widget_path_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_widget_path_to_string(self.to_glib_none().0))
        }
    }
}

impl Default for WidgetPath {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

impl std::fmt::Display for WidgetPath {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}
