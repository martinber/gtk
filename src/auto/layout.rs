// This file was generated by gir (e0b4c3b) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Adjustment;
use Buildable;
use Container;
use Widget;

pub type Layout = Object<ffi::GtkLayout>;

unsafe impl Upcast<Widget> for Layout { }
unsafe impl Upcast<Container> for Layout { }
unsafe impl Upcast<Buildable> for Layout { }

impl Layout {
    pub fn new(hadjustment: Option<&Adjustment>, vadjustment: Option<&Adjustment>) -> Layout {
        unsafe {
            Widget::from_glib_none(ffi::gtk_layout_new(hadjustment.to_glib_none().0, vadjustment.to_glib_none().0)).downcast_unchecked()
        }
    }

    //pub fn get_bin_window(&self) -> Option<gdk::Window> {
    //    unsafe { TODO: call ffi:gtk_layout_get_bin_window() }
    //}

    pub fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_hadjustment(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let mut width = Default::default();
        let mut height = Default::default();
        unsafe {
            ffi::gtk_layout_get_size(self.to_glib_none().0, &mut width, &mut height);
        }
        (width, height)
    }

    pub fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_vadjustment(self.to_glib_none().0))
        }
    }

    pub fn move_<T: Upcast<Widget>>(&self, child_widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(self.to_glib_none().0, child_widget.upcast().to_glib_none().0, x, y);
        }
    }

    pub fn put<T: Upcast<Widget>>(&self, child_widget: &T, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_put(self.to_glib_none().0, child_widget.upcast().to_glib_none().0, x, y);
        }
    }

    pub fn set_hadjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_layout_set_hadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            ffi::gtk_layout_set_size(self.to_glib_none().0, width, height);
        }
    }

    pub fn set_vadjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            ffi::gtk_layout_set_vadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

}

impl types::StaticType for Layout {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_layout_get_type()) }
    }
}
