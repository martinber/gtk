// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(not(gtk_3_10), allow(unused_imports))]

use libc::{c_char, c_long};

use glib::{self, types};
use glib::translate::*;
use ffi;

use object::{Object, Downcast, Upcast};

pub type Buildable = Object<ffi::GtkBuildable>;

impl types::StaticType for Buildable {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_buildable_get_type()) }
    }
}

pub type Builder = Object<ffi::GtkBuilder>;

impl Builder {
    pub fn new() -> Builder {
        unsafe { from_glib_full(ffi::gtk_builder_new()) }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_file(file_name: &str) -> Builder {
        unsafe { from_glib_full(ffi::gtk_builder_new_from_file(file_name.to_glib_none().0)) }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_resource(resource_path: &str) -> Builder {
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_resource(resource_path.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_10)]
    pub fn new_from_string(string: &str) -> Builder {
        unsafe {
            // Don't need a null-terminated string here
            from_glib_full(
                ffi::gtk_builder_new_from_string(string.as_ptr() as *const c_char,
                    string.len() as c_long))
        }
    }

    pub fn get_object<T>(&self, name: &str) -> Option<T>
    where T: Upcast<Buildable> + Upcast<glib::object::Object> {
        unsafe {
            Option::<glib::object::Object>::from_glib_none(
                ffi::gtk_builder_get_object(self.to_glib_none().0, name.to_glib_none().0))
                .and_then(|obj| obj.downcast().ok())
        }
    }
}

impl types::StaticType for Builder {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_builder_get_type()) }
    }
}