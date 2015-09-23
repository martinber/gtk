// This file was generated by gir (e0b4c3b) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Buildable;
use CalendarDisplayOptions;
use Widget;

pub type Calendar = Object<ffi::GtkCalendar>;

unsafe impl Upcast<Widget> for Calendar { }
unsafe impl Upcast<Buildable> for Calendar { }

impl Calendar {
    pub fn new() -> Calendar {
        unsafe {
            Widget::from_glib_none(ffi::gtk_calendar_new()).downcast_unchecked()
        }
    }

    pub fn clear_marks(&self) {
        unsafe {
            ffi::gtk_calendar_clear_marks(self.to_glib_none().0);
        }
    }

    pub fn get_date(&self) -> (u32, u32, u32) {
        let mut year = Default::default();
        let mut month = Default::default();
        let mut day = Default::default();
        unsafe {
            ffi::gtk_calendar_get_date(self.to_glib_none().0, &mut year, &mut month, &mut day);
        }
        (year, month, day)
    }

    pub fn get_day_is_marked(&self, day: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_day_is_marked(self.to_glib_none().0, day))
        }
    }

    pub fn get_detail_height_rows(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_height_rows(self.to_glib_none().0)
        }
    }

    pub fn get_detail_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_width_chars(self.to_glib_none().0)
        }
    }

    pub fn get_display_options(&self) -> CalendarDisplayOptions {
        unsafe {
            ffi::gtk_calendar_get_display_options(self.to_glib_none().0)
        }
    }

    pub fn mark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_mark_day(self.to_glib_none().0, day);
        }
    }

    pub fn select_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_select_day(self.to_glib_none().0, day);
        }
    }

    pub fn select_month(&self, month: u32, year: u32) {
        unsafe {
            ffi::gtk_calendar_select_month(self.to_glib_none().0, month, year);
        }
    }

    //pub fn set_detail_func(&self, func: /*Unknown kind*/Unknown rust type: "CalendarDetailFunc", data: Fundamental: Pointer, destroy: /*Unknown kind*/Unknown rust type: "DestroyNotify") {
    //    unsafe { TODO: call ffi:gtk_calendar_set_detail_func() }
    //}

    pub fn set_detail_height_rows(&self, rows: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_height_rows(self.to_glib_none().0, rows);
        }
    }

    pub fn set_detail_width_chars(&self, chars: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_width_chars(self.to_glib_none().0, chars);
        }
    }

    pub fn set_display_options(&self, flags: CalendarDisplayOptions) {
        unsafe {
            ffi::gtk_calendar_set_display_options(self.to_glib_none().0, flags);
        }
    }

    pub fn unmark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_unmark_day(self.to_glib_none().0, day);
        }
    }

}

impl types::StaticType for Calendar {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_calendar_get_type()) }
    }
}
