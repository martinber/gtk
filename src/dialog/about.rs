// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use object::{Object, Downcast, Upcast};
use widgets::widget::Widget;

pub type AboutDialog = Object<ffi::GtkAboutDialog>;

impl AboutDialog {
    pub fn new() -> AboutDialog {
        unsafe { Widget::from_glib_none(ffi::gtk_about_dialog_new()).downcast_unchecked() }
    }

    pub fn get_program_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_program_name(self.to_glib_none().0))
        }
    }

    pub fn set_program_name(&self, name: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(self.to_glib_none().0, name.to_glib_none().0)
        };
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_version(self.to_glib_none().0))
        }
    }

    pub fn set_version(&self, version: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_version(self.to_glib_none().0, version.to_glib_none().0)
        };
    }

    pub fn get_copyright(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_copyright(self.to_glib_none().0))
        }
    }

    pub fn set_copyright(&self, copyright: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_copyright(self.to_glib_none().0, copyright.to_glib_none().0)
        };
    }

    pub fn get_comments(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_comments(self.to_glib_none().0))
        }
    }

    pub fn set_comments(&self, comments: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_comments(self.to_glib_none().0, comments.to_glib_none().0)
        };
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_license(self.to_glib_none().0))
        }
    }

    pub fn set_license(&self, license: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_license(self.to_glib_none().0, license.to_glib_none().0)
        };
    }

    pub fn get_wrap_license(&self) -> bool {
        unsafe { from_glib(ffi::gtk_about_dialog_get_wrap_license(self.to_glib_none().0)) }
    }

    pub fn set_wrap_license(&self, wrap_license: bool) {
        unsafe { ffi::gtk_about_dialog_set_wrap_license(self.to_glib_none().0, wrap_license.to_glib()) }
    }

    pub fn get_license_type(&self) -> ::License {
        unsafe { ffi::gtk_about_dialog_get_license_type(self.to_glib_none().0) }
    }

    pub fn set_license_type(&self, license_type: ::License) {
        unsafe { ffi::gtk_about_dialog_set_license_type(self.to_glib_none().0, license_type) }
    }

    pub fn get_website(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_website(self.to_glib_none().0))
        }
    }

    pub fn set_website(&self, website: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_website(self.to_glib_none().0, website.to_glib_none().0)
        };
    }

    pub fn get_website_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_website_label(self.to_glib_none().0))
        }
    }

    pub fn set_website_label(&self, website_label: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_website_label(self.to_glib_none().0, website_label.to_glib_none().0)
        };
    }

    pub fn get_authors(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_none(
                ffi::gtk_about_dialog_get_authors(self.to_glib_none().0))
        }
    }

    pub fn set_authors(&self, authors: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_authors(
                self.to_glib_none().0,
                authors.to_glib_none().0);
        }
    }

    pub fn get_artists(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_none(
                ffi::gtk_about_dialog_get_artists(self.to_glib_none().0))
        }
    }

    pub fn set_artists(&self, artists: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_artists(
                self.to_glib_none().0,
                artists.to_glib_none().0);
        }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        unsafe {
            Vec::from_glib_none(
                ffi::gtk_about_dialog_get_documenters(self.to_glib_none().0))
        }
    }

    pub fn set_documenters(&self, documenters: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(
                self.to_glib_none().0,
                documenters.to_glib_none().0);
        }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_translator_credits(self.to_glib_none().0))
        }
    }

    pub fn set_translator_credits(&self, translator_credits: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(
                self.to_glib_none().0,
                translator_credits.to_glib_none().0)
        };
    }

    /*pub fn get_logo(&self) -> Option<String> {
        let logo = unsafe { ffi::gtk_about_dialog_set_logo(self.pointer)) };

        if logo.is_null() {
            None
        } else {
            Some(unsafe { ::FFIWidget::wrap_widget(logo) })
        }
    }

    pub fn set_logo(&self, logo: Pixbuf) {
        unsafe { ffi::gtk_about_dialog_set_logo(self.to_glib_none().0, GDK_PIXBUF(logo.unwrap_widget())) }
    }*/

    pub fn get_logo_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_about_dialog_get_logo_icon_name(self.to_glib_none().0))
        }
    }

    pub fn set_logo_icon_name(&self, logo_icon_name: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(
                self.to_glib_none().0,
                logo_icon_name.to_glib_none().0)
        };
    }

    pub fn add_credit_section(&self, section_name: &str, people: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(
                self.to_glib_none().0,
                section_name.to_glib_none().0,
                people.to_glib_none().0)
        }
    }

    /*pub fn show(parent: Window, properties: Vec<String>) {
        unsafe { ffi::gtk_show_about_dialog(GTK_WINDOW(parent), first_property_name, ...) }
    }*/
}

impl types::StaticType for AboutDialog {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_about_dialog_get_type()) }
    }
}

unsafe impl Upcast<Widget> for AboutDialog { }
unsafe impl Upcast<::widgets::container::Container> for AboutDialog { }
unsafe impl Upcast<::widgets::bin::Bin> for AboutDialog { }
unsafe impl Upcast<::window::Window> for AboutDialog { }
unsafe impl Upcast<super::Dialog> for AboutDialog { }
unsafe impl Upcast<::builder::Buildable> for AboutDialog { }