// This file was generated by gir (df8a3f3) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Buildable;
use Widget;
use std::mem;

pub type Misc = Object<ffi::GtkMisc>;

unsafe impl Upcast<Widget> for Misc { }
unsafe impl Upcast<Buildable> for Misc { }

impl types::StaticType for Misc {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_misc_get_type()) }
    }
}

pub trait MiscExt {
    fn get_alignment(&self) -> (f32, f32);
    fn get_padding(&self) -> (i32, i32);
    fn set_alignment(&self, xalign: f32, yalign: f32);
    fn set_padding(&self, xpad: i32, ypad: i32);
}

impl<O: Upcast<Misc>> MiscExt for O {
    fn get_alignment(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_misc_get_alignment(self.upcast().to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_padding(&self) -> (i32, i32) {
        unsafe {
            let mut xpad = mem::uninitialized();
            let mut ypad = mem::uninitialized();
            ffi::gtk_misc_get_padding(self.upcast().to_glib_none().0, &mut xpad, &mut ypad);
            (xpad, ypad)
        }
    }

    fn set_alignment(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_misc_set_alignment(self.upcast().to_glib_none().0, xalign, yalign);
        }
    }

    fn set_padding(&self, xpad: i32, ypad: i32) {
        unsafe {
            ffi::gtk_misc_set_padding(self.upcast().to_glib_none().0, xpad, ypad);
        }
    }

}