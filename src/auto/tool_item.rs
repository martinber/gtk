// This file was generated by gir (df8a3f3) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Bin;
use Buildable;
use Container;
use Orientation;
use ReliefStyle;
use SizeGroup;
use ToolbarStyle;
use Widget;

pub type ToolItem = Object<ffi::GtkToolItem>;

unsafe impl Upcast<Widget> for ToolItem { }
unsafe impl Upcast<Container> for ToolItem { }
unsafe impl Upcast<Bin> for ToolItem { }
unsafe impl Upcast<Buildable> for ToolItem { }

impl ToolItem {
    pub fn new() -> ToolItem {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_new())
        }
    }

}

impl types::StaticType for ToolItem {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_tool_item_get_type()) }
    }
}

pub trait ToolItemExt {
    //fn get_ellipsize_mode(&self) -> pango::EllipsizeMode;
    fn get_expand(&self) -> bool;
    fn get_homogeneous(&self) -> bool;
    fn get_icon_size(&self) -> i32;
    fn get_is_important(&self) -> bool;
    fn get_orientation(&self) -> Orientation;
    fn get_proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget>;
    fn get_relief_style(&self) -> ReliefStyle;
    fn get_text_alignment(&self) -> f32;
    fn get_text_orientation(&self) -> Orientation;
    fn get_text_size_group(&self) -> Option<SizeGroup>;
    fn get_toolbar_style(&self) -> ToolbarStyle;
    fn get_use_drag_window(&self) -> bool;
    fn get_visible_horizontal(&self) -> bool;
    fn get_visible_vertical(&self) -> bool;
    fn rebuild_menu(&self);
    fn retrieve_proxy_menu_item(&self) -> Option<Widget>;
    fn set_expand(&self, expand: bool);
    fn set_homogeneous(&self, homogeneous: bool);
    fn set_is_important(&self, is_important: bool);
    fn set_proxy_menu_item<T: Upcast<Widget>>(&self, menu_item_id: &str, menu_item: &T);
    fn set_tooltip_markup(&self, markup: &str);
    fn set_tooltip_text(&self, text: &str);
    fn set_use_drag_window(&self, use_drag_window: bool);
    fn set_visible_horizontal(&self, visible_horizontal: bool);
    fn set_visible_vertical(&self, visible_vertical: bool);
    fn toolbar_reconfigured(&self);
}

impl<O: Upcast<ToolItem>> ToolItemExt for O {
    //fn get_ellipsize_mode(&self) -> pango::EllipsizeMode {
    //    unsafe { TODO: call ffi:gtk_tool_item_get_ellipsize_mode() }
    //}

    fn get_expand(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_expand(self.upcast().to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_homogeneous(self.upcast().to_glib_none().0))
        }
    }

    fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_item_get_icon_size(self.upcast().to_glib_none().0)
        }
    }

    fn get_is_important(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_is_important(self.upcast().to_glib_none().0))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_item_get_orientation(self.upcast().to_glib_none().0)
        }
    }

    fn get_proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_proxy_menu_item(self.upcast().to_glib_none().0, menu_item_id.to_glib_none().0))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            ffi::gtk_tool_item_get_relief_style(self.upcast().to_glib_none().0)
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_item_get_text_alignment(self.upcast().to_glib_none().0)
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            ffi::gtk_tool_item_get_text_orientation(self.upcast().to_glib_none().0)
        }
    }

    fn get_text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_text_size_group(self.upcast().to_glib_none().0))
        }
    }

    fn get_toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            ffi::gtk_tool_item_get_toolbar_style(self.upcast().to_glib_none().0)
        }
    }

    fn get_use_drag_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_use_drag_window(self.upcast().to_glib_none().0))
        }
    }

    fn get_visible_horizontal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_horizontal(self.upcast().to_glib_none().0))
        }
    }

    fn get_visible_vertical(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_vertical(self.upcast().to_glib_none().0))
        }
    }

    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(self.upcast().to_glib_none().0);
        }
    }

    fn retrieve_proxy_menu_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_retrieve_proxy_menu_item(self.upcast().to_glib_none().0))
        }
    }

    fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tool_item_set_expand(self.upcast().to_glib_none().0, expand.to_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_tool_item_set_homogeneous(self.upcast().to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_is_important(&self, is_important: bool) {
        unsafe {
            ffi::gtk_tool_item_set_is_important(self.upcast().to_glib_none().0, is_important.to_glib());
        }
    }

    fn set_proxy_menu_item<T: Upcast<Widget>>(&self, menu_item_id: &str, menu_item: &T) {
        unsafe {
            ffi::gtk_tool_item_set_proxy_menu_item(self.upcast().to_glib_none().0, menu_item_id.to_glib_none().0, menu_item.upcast().to_glib_none().0);
        }
    }

    fn set_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_markup(self.upcast().to_glib_none().0, markup.to_glib_none().0);
        }
    }

    fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_text(self.upcast().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_use_drag_window(&self, use_drag_window: bool) {
        unsafe {
            ffi::gtk_tool_item_set_use_drag_window(self.upcast().to_glib_none().0, use_drag_window.to_glib());
        }
    }

    fn set_visible_horizontal(&self, visible_horizontal: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_horizontal(self.upcast().to_glib_none().0, visible_horizontal.to_glib());
        }
    }

    fn set_visible_vertical(&self, visible_vertical: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_vertical(self.upcast().to_glib_none().0, visible_vertical.to_glib());
        }
    }

    fn toolbar_reconfigured(&self) {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(self.upcast().to_glib_none().0);
        }
    }

}