// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use TextBuffer;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TextMark(Object<ffi::GtkTextMark, ffi::GtkTextMarkClass>);

    match fn {
        get_type => || ffi::gtk_text_mark_get_type(),
    }
}

impl TextMark {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P, left_gravity: bool) -> TextMark {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_text_mark_new(name.0, left_gravity.to_glib()))
        }
    }
}

pub trait TextMarkExt {
    fn get_buffer(&self) -> Option<TextBuffer>;

    fn get_deleted(&self) -> bool;

    fn get_left_gravity(&self) -> bool;

    fn get_name(&self) -> Option<String>;

    fn get_visible(&self) -> bool;

    fn set_visible(&self, setting: bool);

    fn connect_property_left_gravity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TextMark> + IsA<glib::object::Object>> TextMarkExt for O {
    fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_buffer(self.to_glib_none().0))
        }
    }

    fn get_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_deleted(self.to_glib_none().0))
        }
    }

    fn get_left_gravity(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_left_gravity(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_name(self.to_glib_none().0))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_visible(self.to_glib_none().0))
        }
    }

    fn set_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_mark_set_visible(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn connect_property_left_gravity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::left-gravity",
                transmute(notify_left_gravity_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_left_gravity_trampoline<P>(this: *mut ffi::GtkTextMark, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TextMark> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TextMark::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::GtkTextMark, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TextMark> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TextMark::from_glib_borrow(this).downcast_unchecked())
}
