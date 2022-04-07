// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AxisFlags;
use crate::DeviceToolType;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkDeviceTool")]
    pub struct DeviceTool(Object<ffi::GdkDeviceTool>);

    match fn {
        type_ => || ffi::gdk_device_tool_get_type(),
    }
}

impl DeviceTool {
    #[doc(alias = "gdk_device_tool_get_hardware_id")]
    #[doc(alias = "get_hardware_id")]
    pub fn hardware_id(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_hardware_id(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_tool_get_serial")]
    #[doc(alias = "get_serial")]
    pub fn serial(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_serial(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_device_tool_get_tool_type")]
    #[doc(alias = "get_tool_type")]
    pub fn tool_type(&self) -> DeviceToolType {
        unsafe { from_glib(ffi::gdk_device_tool_get_tool_type(self.to_glib_none().0)) }
    }

    pub fn axes(&self) -> AxisFlags {
        glib::ObjectExt::property(self, "axes")
    }
}

impl fmt::Display for DeviceTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceTool")
    }
}
