// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::GString;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FontChooser(Object<ffi::GtkFontChooser, ffi::GtkFontChooserIface>);

    match fn {
        get_type => || ffi::gtk_font_chooser_get_type(),
    }
}

pub trait FontChooserExt: 'static {
    fn get_font(&self) -> Option<GString>;

    fn get_font_desc(&self) -> Option<pango::FontDescription>;

    fn get_font_face(&self) -> Option<pango::FontFace>;

    fn get_font_family(&self) -> Option<pango::FontFamily>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_font_map(&self) -> Option<pango::FontMap>;

    fn get_font_size(&self) -> i32;

    fn get_preview_text(&self) -> Option<GString>;

    fn get_show_preview_entry(&self) -> bool;

    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/FontFilterFunc>>>(&self, filter: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_font(&self, fontname: &str);

    fn set_font_desc(&self, font_desc: &pango::FontDescription);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_font_map<'a, P: Into<Option<&'a pango::FontMap>>>(&self, fontmap: P);

    fn set_preview_text(&self, text: &str);

    fn set_show_preview_entry(&self, show_preview_entry: bool);

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FontChooser>> FontChooserExt for O {
    fn get_font(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font(self.to_glib_none().0))
        }
    }

    fn get_font_desc(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_desc(self.to_glib_none().0))
        }
    }

    fn get_font_face(&self) -> Option<pango::FontFace> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_face(self.to_glib_none().0))
        }
    }

    fn get_font_family(&self) -> Option<pango::FontFamily> {
        unsafe {
            from_glib_none(ffi::gtk_font_chooser_get_font_family(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_font_map(&self) -> Option<pango::FontMap> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_font_map(self.to_glib_none().0))
        }
    }

    fn get_font_size(&self) -> i32 {
        unsafe {
            ffi::gtk_font_chooser_get_font_size(self.to_glib_none().0)
        }
    }

    fn get_preview_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::gtk_font_chooser_get_preview_text(self.to_glib_none().0))
        }
    }

    fn get_show_preview_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_chooser_get_show_preview_entry(self.to_glib_none().0))
        }
    }

    //fn set_filter_func<'a, P: Into<Option<&'a /*Unimplemented*/FontFilterFunc>>>(&self, filter: P, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_font_chooser_set_filter_func() }
    //}

    fn set_font(&self, fontname: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_font(self.to_glib_none().0, fontname.to_glib_none().0);
        }
    }

    fn set_font_desc(&self, font_desc: &pango::FontDescription) {
        unsafe {
            ffi::gtk_font_chooser_set_font_desc(self.to_glib_none().0, font_desc.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_font_map<'a, P: Into<Option<&'a pango::FontMap>>>(&self, fontmap: P) {
        let fontmap = fontmap.into();
        let fontmap = fontmap.to_glib_none();
        unsafe {
            ffi::gtk_font_chooser_set_font_map(self.to_glib_none().0, fontmap.0);
        }
    }

    fn set_preview_text(&self, text: &str) {
        unsafe {
            ffi::gtk_font_chooser_set_preview_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_show_preview_entry(&self, show_preview_entry: bool) {
        unsafe {
            ffi::gtk_font_chooser_set_show_preview_entry(self.to_glib_none().0, show_preview_entry.to_glib());
        }
    }

    fn connect_font_activated<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"font-activated\0".as_ptr() as *const _,
                transmute(font_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_font_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::font\0".as_ptr() as *const _,
                transmute(notify_font_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::font-desc\0".as_ptr() as *const _,
                transmute(notify_font_desc_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_preview_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::preview-text\0".as_ptr() as *const _,
                transmute(notify_preview_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_show_preview_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::show-preview-entry\0".as_ptr() as *const _,
                transmute(notify_show_preview_entry_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn font_activated_trampoline<P>(this: *mut ffi::GtkFontChooser, fontname: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).downcast_unchecked(), &GString::from_glib_borrow(fontname))
}

unsafe extern "C" fn notify_font_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_font_desc_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_preview_text_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_show_preview_entry_trampoline<P>(this: *mut ffi::GtkFontChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FontChooser> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FontChooser::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for FontChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontChooser")
    }
}
