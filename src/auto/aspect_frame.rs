// This file was generated by gir (e0b4c3b) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Bin;
use Buildable;
use Container;
use Frame;
use Widget;

pub type AspectFrame = Object<ffi::GtkAspectFrame>;

unsafe impl Upcast<Widget> for AspectFrame { }
unsafe impl Upcast<Container> for AspectFrame { }
unsafe impl Upcast<Bin> for AspectFrame { }
unsafe impl Upcast<Frame> for AspectFrame { }
unsafe impl Upcast<Buildable> for AspectFrame { }

impl AspectFrame {
    pub fn new(label: Option<&str>, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) -> AspectFrame {
        unsafe {
            Widget::from_glib_none(ffi::gtk_aspect_frame_new(label.to_glib_none().0, xalign, yalign, ratio, obey_child.to_glib())).downcast_unchecked()
        }
    }

    pub fn set(&self, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) {
        unsafe {
            ffi::gtk_aspect_frame_set(self.to_glib_none().0, xalign, yalign, ratio, obey_child.to_glib());
        }
    }

}

impl types::StaticType for AspectFrame {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_aspect_frame_get_type()) }
    }
}
