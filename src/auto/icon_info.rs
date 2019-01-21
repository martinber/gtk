// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use IconTheme;
use StyleContext;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use cairo;
use ffi;
#[cfg(feature = "futures")]
#[cfg(any(feature = "v3_8", feature = "dox"))]
use futures_core;
use gdk;
use gdk_pixbuf;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use gio;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use gio_ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use gobject_ffi;
use std;
#[cfg(feature = "futures")]
#[cfg(any(feature = "v3_8", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IconInfo(Object<ffi::GtkIconInfo, ffi::GtkIconInfoClass, IconInfoClass>);

    match fn {
        get_type => || ffi::gtk_icon_info_get_type(),
    }
}

impl IconInfo {
    pub fn new_for_pixbuf<P: IsA<IconTheme>>(icon_theme: &P, pixbuf: &gdk_pixbuf::Pixbuf) -> IconInfo {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_icon_info_new_for_pixbuf(icon_theme.as_ref().to_glib_none().0, pixbuf.to_glib_none().0))
        }
    }
}

pub const NONE_ICON_INFO: Option<&IconInfo> = None;

pub trait IconInfoExt: 'static {
    #[cfg_attr(feature = "v3_8", deprecated)]
    fn copy(&self) -> Option<IconInfo>;

    //#[cfg_attr(feature = "v3_14", deprecated)]
    //fn get_attach_points(&self, points: /*Ignored*/Vec<gdk::Point>) -> Option<i32>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_base_scale(&self) -> i32;

    fn get_base_size(&self) -> i32;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_builtin_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_display_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn get_embedded_rect(&self) -> Option<gdk::Rectangle>;

    fn get_filename(&self) -> Option<std::path::PathBuf>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn is_symbolic(&self) -> bool;

    fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error>;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_icon_async<'a, P: IsA<gio::Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static>(&self, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_icon_async_future(&self) -> Box_<futures_core::Future<Item = (Self, gdk_pixbuf::Pixbuf), Error = (Self, Error)>> where Self: Sized + Clone;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn load_surface<'a, P: IsA<gdk::Window> + 'a, Q: Into<Option<&'a P>>>(&self, for_window: Q) -> Result<cairo::Surface, Error>;

    fn load_symbolic<'a, 'b, 'c, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_async<'a, 'b, 'c, 'd, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>, S: IsA<gio::Cancellable> + 'd, T: Into<Option<&'d S>>, U: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R, cancellable: T, callback: U);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_async_future<'a, 'b, 'c, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R) -> Box_<futures_core::Future<Item = (Self, (gdk_pixbuf::Pixbuf, bool)), Error = (Self, Error)>> where Self: Sized + Clone;

    fn load_symbolic_for_context<P: IsA<StyleContext>>(&self, context: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_for_context_async<'a, P: IsA<StyleContext>, Q: IsA<gio::Cancellable> + 'a, R: Into<Option<&'a Q>>, S: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, context: &P, cancellable: R, callback: S);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_for_context_async_future<P: IsA<StyleContext> + Clone + 'static>(&self, context: &P) -> Box_<futures_core::Future<Item = (Self, (gdk_pixbuf::Pixbuf, bool)), Error = (Self, Error)>> where Self: Sized + Clone;

    #[cfg_attr(feature = "v3_14", deprecated)]
    fn set_raw_coordinates(&self, raw_coordinates: bool);
}

impl<O: IsA<IconInfo>> IconInfoExt for O {
    fn copy(&self) -> Option<IconInfo> {
        unsafe {
            from_glib_full(ffi::gtk_icon_info_copy(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_attach_points(&self, points: /*Ignored*/Vec<gdk::Point>) -> Option<i32> {
    //    unsafe { TODO: call ffi::gtk_icon_info_get_attach_points() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_base_scale(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_info_get_base_scale(self.as_ref().to_glib_none().0)
        }
    }

    fn get_base_size(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_info_get_base_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_builtin_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_icon_info_get_builtin_pixbuf(self.as_ref().to_glib_none().0))
        }
    }

    fn get_display_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_icon_info_get_display_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_embedded_rect(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rectangle = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_icon_info_get_embedded_rect(self.as_ref().to_glib_none().0, rectangle.to_glib_none_mut().0));
            if ret { Some(rectangle) } else { None }
        }
    }

    fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::gtk_icon_info_get_filename(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn is_symbolic(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_info_is_symbolic(self.as_ref().to_glib_none().0))
        }
    }

    fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_icon(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_icon_async<'a, P: IsA<gio::Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static>(&self, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn load_icon_async_trampoline<R: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_icon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_icon_async_trampoline::<R>;
        unsafe {
            ffi::gtk_icon_info_load_icon_async(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_icon_async_future(&self) -> Box_<futures_core::Future<Item = (Self, gdk_pixbuf::Pixbuf), Error = (Self, Error)>> where Self: Sized + Clone {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.load_icon_async(
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn load_surface<'a, P: IsA<gdk::Window> + 'a, Q: Into<Option<&'a P>>>(&self, for_window: Q) -> Result<cairo::Surface, Error> {
        let for_window = for_window.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_surface(self.as_ref().to_glib_none().0, for_window.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_symbolic<'a, 'b, 'c, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        let success_color = success_color.into();
        let warning_color = warning_color.into();
        let error_color = error_color.into();
        unsafe {
            let mut was_symbolic = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_symbolic(self.as_ref().to_glib_none().0, fg.to_glib_none().0, success_color.to_glib_none().0, warning_color.to_glib_none().0, error_color.to_glib_none().0, &mut was_symbolic, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_async<'a, 'b, 'c, 'd, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>, S: IsA<gio::Cancellable> + 'd, T: Into<Option<&'d S>>, U: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R, cancellable: T, callback: U) {
        let success_color = success_color.into();
        let warning_color = warning_color.into();
        let error_color = error_color.into();
        let cancellable = cancellable.into();
        let user_data: Box<Box<U>> = Box::new(Box::new(callback));
        unsafe extern "C" fn load_symbolic_async_trampoline<U: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::uninitialized();
            let ret = ffi::gtk_icon_info_load_symbolic_finish(_source_object as *mut _, res, &mut was_symbolic, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<U>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_async_trampoline::<U>;
        unsafe {
            ffi::gtk_icon_info_load_symbolic_async(self.as_ref().to_glib_none().0, fg.to_glib_none().0, success_color.to_glib_none().0, warning_color.to_glib_none().0, error_color.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_async_future<'a, 'b, 'c, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R) -> Box_<futures_core::Future<Item = (Self, (gdk_pixbuf::Pixbuf, bool)), Error = (Self, Error)>> where Self: Sized + Clone {
        use gio::GioFuture;
        use fragile::Fragile;

        let fg = fg.clone();
        let success_color = success_color.into();
        let success_color = success_color.map(ToOwned::to_owned);
        let warning_color = warning_color.into();
        let warning_color = warning_color.map(ToOwned::to_owned);
        let error_color = error_color.into();
        let error_color = error_color.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.load_symbolic_async(
                 &fg,
                 success_color.as_ref().map(::std::borrow::Borrow::borrow),
                 warning_color.as_ref().map(::std::borrow::Borrow::borrow),
                 error_color.as_ref().map(::std::borrow::Borrow::borrow),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn load_symbolic_for_context<P: IsA<StyleContext>>(&self, context: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        unsafe {
            let mut was_symbolic = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_symbolic_for_context(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, &mut was_symbolic, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_for_context_async<'a, P: IsA<StyleContext>, Q: IsA<gio::Cancellable> + 'a, R: Into<Option<&'a Q>>, S: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, context: &P, cancellable: R, callback: S) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<S>> = Box::new(Box::new(callback));
        unsafe extern "C" fn load_symbolic_for_context_async_trampoline<S: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut gio_ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::uninitialized();
            let ret = ffi::gtk_icon_info_load_symbolic_for_context_finish(_source_object as *mut _, res, &mut was_symbolic, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<S>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_for_context_async_trampoline::<S>;
        unsafe {
            ffi::gtk_icon_info_load_symbolic_for_context_async(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn load_symbolic_for_context_async_future<P: IsA<StyleContext> + Clone + 'static>(&self, context: &P) -> Box_<futures_core::Future<Item = (Self, (gdk_pixbuf::Pixbuf, bool)), Error = (Self, Error)>> where Self: Sized + Clone {
        use gio::GioFuture;
        use fragile::Fragile;

        let context = context.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.load_symbolic_for_context_async(
                 &context,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn set_raw_coordinates(&self, raw_coordinates: bool) {
        unsafe {
            ffi::gtk_icon_info_set_raw_coordinates(self.as_ref().to_glib_none().0, raw_coordinates.to_glib());
        }
    }
}

impl fmt::Display for IconInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconInfo")
    }
}
