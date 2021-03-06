// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Container;
use Widget;
use ffi;
use gdk;
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
use xlib;

glib_wrapper! {
    pub struct Socket(Object<ffi::GtkSocket, ffi::GtkSocketClass>): Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_socket_get_type(),
    }
}

impl Socket {
    pub fn new() -> Socket {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_socket_new()).downcast_unchecked()
        }
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SocketExt {
    fn add_id(&self, window: xlib::Window);

    fn get_id(&self) -> xlib::Window;

    fn get_plug_window(&self) -> Option<gdk::Window>;

    fn connect_plug_added<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_plug_removed<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Socket> + IsA<glib::object::Object>> SocketExt for O {
    fn add_id(&self, window: xlib::Window) {
        unsafe {
            ffi::gtk_socket_add_id(self.to_glib_none().0, window);
        }
    }

    fn get_id(&self) -> xlib::Window {
        unsafe {
            ffi::gtk_socket_get_id(self.to_glib_none().0)
        }
    }

    fn get_plug_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_socket_get_plug_window(self.to_glib_none().0))
        }
    }

    fn connect_plug_added<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "plug-added",
                transmute(plug_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_plug_removed<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "plug-removed",
                transmute(plug_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn plug_added_trampoline<P>(this: *mut ffi::GtkSocket, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn plug_removed_trampoline<P>(this: *mut ffi::GtkSocket, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Socket> {
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked()).to_glib()
}
