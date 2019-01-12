// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use PadActionType;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use Window;
use ffi;
use gdk;
use gio;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::fmt;

glib_wrapper! {
    pub struct PadController(Object<ffi::GtkPadController, ffi::GtkPadControllerClass>): EventController;

    match fn {
        get_type => || ffi::gtk_pad_controller_get_type(),
    }
}

impl PadController {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn new<'a, P: IsA<Window>, Q: IsA<gio::ActionGroup>, R: IsA<gdk::Device> + 'a, S: Into<Option<&'a R>>>(window: &P, group: &Q, pad: S) -> PadController {
        skip_assert_initialized!();
        let pad = pad.into();
        let pad = pad.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_pad_controller_new(window.to_glib_none().0, group.to_glib_none().0, pad.0))
        }
    }
}

pub trait PadControllerExt: 'static {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action(&self, type_: PadActionType, index: i32, mode: i32, label: &str, action_name: &str);

    fn get_property_action_group(&self) -> Option<gio::ActionGroup>;

    fn get_property_pad(&self) -> Option<gdk::Device>;
}

impl<O: IsA<PadController>> PadControllerExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn set_action(&self, type_: PadActionType, index: i32, mode: i32, label: &str, action_name: &str) {
        unsafe {
            ffi::gtk_pad_controller_set_action(self.to_glib_none().0, type_.to_glib(), index, mode, label.to_glib_none().0, action_name.to_glib_none().0);
        }
    }

    fn get_property_action_group(&self) -> Option<gio::ActionGroup> {
        unsafe {
            let mut value = Value::from_type(<gio::ActionGroup as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"action-group\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_pad(&self) -> Option<gdk::Device> {
        unsafe {
            let mut value = Value::from_type(<gdk::Device as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"pad\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }
}

impl fmt::Display for PadController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PadController")
    }
}
