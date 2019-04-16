// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use gtk_sys;

bitflags! {
    pub struct AccelFlags: u32 {
        const VISIBLE = 1;
        const LOCKED = 2;
        const MASK = 7;
    }
}

#[doc(hidden)]
impl ToGlib for AccelFlags {
    type GlibType = gtk_sys::GtkAccelFlags;

    fn to_glib(&self) -> gtk_sys::GtkAccelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkAccelFlags> for AccelFlags {
    fn from_glib(value: gtk_sys::GtkAccelFlags) -> AccelFlags {
        skip_assert_initialized!();
        AccelFlags::from_bits_truncate(value)
    }
}

impl StaticType for AccelFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_accel_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AccelFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AccelFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for AccelFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ApplicationInhibitFlags: u32 {
        const LOGOUT = 1;
        const SWITCH = 2;
        const SUSPEND = 4;
        const IDLE = 8;
    }
}

#[doc(hidden)]
impl ToGlib for ApplicationInhibitFlags {
    type GlibType = gtk_sys::GtkApplicationInhibitFlags;

    fn to_glib(&self) -> gtk_sys::GtkApplicationInhibitFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkApplicationInhibitFlags> for ApplicationInhibitFlags {
    fn from_glib(value: gtk_sys::GtkApplicationInhibitFlags) -> ApplicationInhibitFlags {
        skip_assert_initialized!();
        ApplicationInhibitFlags::from_bits_truncate(value)
    }
}

impl StaticType for ApplicationInhibitFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_application_inhibit_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ApplicationInhibitFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ApplicationInhibitFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ApplicationInhibitFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct CalendarDisplayOptions: u32 {
        const SHOW_HEADING = 1;
        const SHOW_DAY_NAMES = 2;
        const NO_MONTH_CHANGE = 4;
        const SHOW_WEEK_NUMBERS = 8;
        const SHOW_DETAILS = 32;
    }
}

#[doc(hidden)]
impl ToGlib for CalendarDisplayOptions {
    type GlibType = gtk_sys::GtkCalendarDisplayOptions;

    fn to_glib(&self) -> gtk_sys::GtkCalendarDisplayOptions {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkCalendarDisplayOptions> for CalendarDisplayOptions {
    fn from_glib(value: gtk_sys::GtkCalendarDisplayOptions) -> CalendarDisplayOptions {
        skip_assert_initialized!();
        CalendarDisplayOptions::from_bits_truncate(value)
    }
}

impl StaticType for CalendarDisplayOptions {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_calendar_display_options_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CalendarDisplayOptions {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CalendarDisplayOptions {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for CalendarDisplayOptions {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct CellRendererState: u32 {
        const SELECTED = 1;
        const PRELIT = 2;
        const INSENSITIVE = 4;
        const SORTED = 8;
        const FOCUSED = 16;
        const EXPANDABLE = 32;
        const EXPANDED = 64;
    }
}

#[doc(hidden)]
impl ToGlib for CellRendererState {
    type GlibType = gtk_sys::GtkCellRendererState;

    fn to_glib(&self) -> gtk_sys::GtkCellRendererState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkCellRendererState> for CellRendererState {
    fn from_glib(value: gtk_sys::GtkCellRendererState) -> CellRendererState {
        skip_assert_initialized!();
        CellRendererState::from_bits_truncate(value)
    }
}

impl StaticType for CellRendererState {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_cell_renderer_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CellRendererState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CellRendererState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for CellRendererState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DestDefaults: u32 {
        const MOTION = 1;
        const HIGHLIGHT = 2;
        const DROP = 4;
        const ALL = 7;
    }
}

#[doc(hidden)]
impl ToGlib for DestDefaults {
    type GlibType = gtk_sys::GtkDestDefaults;

    fn to_glib(&self) -> gtk_sys::GtkDestDefaults {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkDestDefaults> for DestDefaults {
    fn from_glib(value: gtk_sys::GtkDestDefaults) -> DestDefaults {
        skip_assert_initialized!();
        DestDefaults::from_bits_truncate(value)
    }
}

impl StaticType for DestDefaults {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_dest_defaults_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DestDefaults {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DestDefaults {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DestDefaults {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DialogFlags: u32 {
        const MODAL = 1;
        const DESTROY_WITH_PARENT = 2;
        const USE_HEADER_BAR = 4;
    }
}

#[doc(hidden)]
impl ToGlib for DialogFlags {
    type GlibType = gtk_sys::GtkDialogFlags;

    fn to_glib(&self) -> gtk_sys::GtkDialogFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkDialogFlags> for DialogFlags {
    fn from_glib(value: gtk_sys::GtkDialogFlags) -> DialogFlags {
        skip_assert_initialized!();
        DialogFlags::from_bits_truncate(value)
    }
}

impl StaticType for DialogFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_dialog_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DialogFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DialogFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DialogFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct FileFilterFlags: u32 {
        const FILENAME = 1;
        const URI = 2;
        const DISPLAY_NAME = 4;
        const MIME_TYPE = 8;
    }
}

#[doc(hidden)]
impl ToGlib for FileFilterFlags {
    type GlibType = gtk_sys::GtkFileFilterFlags;

    fn to_glib(&self) -> gtk_sys::GtkFileFilterFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkFileFilterFlags> for FileFilterFlags {
    fn from_glib(value: gtk_sys::GtkFileFilterFlags) -> FileFilterFlags {
        skip_assert_initialized!();
        FileFilterFlags::from_bits_truncate(value)
    }
}

impl StaticType for FileFilterFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_file_filter_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FileFilterFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FileFilterFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FileFilterFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
bitflags! {
    pub struct FontChooserLevel: u32 {
        const FAMILY = 0;
        const STYLE = 1;
        const SIZE = 2;
        const VARIATIONS = 4;
        const FEATURES = 8;
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for FontChooserLevel {
    type GlibType = gtk_sys::GtkFontChooserLevel;

    fn to_glib(&self) -> gtk_sys::GtkFontChooserLevel {
        self.bits()
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gtk_sys::GtkFontChooserLevel> for FontChooserLevel {
    fn from_glib(value: gtk_sys::GtkFontChooserLevel) -> FontChooserLevel {
        skip_assert_initialized!();
        FontChooserLevel::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl StaticType for FontChooserLevel {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_font_chooser_level_get_type()) }
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl<'a> FromValueOptional<'a> for FontChooserLevel {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl<'a> FromValue<'a> for FontChooserLevel {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_24", feature = "dox"))]
impl SetValue for FontChooserLevel {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct IconLookupFlags: u32 {
        const NO_SVG = 1;
        const FORCE_SVG = 2;
        const USE_BUILTIN = 4;
        const GENERIC_FALLBACK = 8;
        const FORCE_SIZE = 16;
        const FORCE_REGULAR = 32;
        const FORCE_SYMBOLIC = 64;
        const DIR_LTR = 128;
        const DIR_RTL = 256;
    }
}

#[doc(hidden)]
impl ToGlib for IconLookupFlags {
    type GlibType = gtk_sys::GtkIconLookupFlags;

    fn to_glib(&self) -> gtk_sys::GtkIconLookupFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkIconLookupFlags> for IconLookupFlags {
    fn from_glib(value: gtk_sys::GtkIconLookupFlags) -> IconLookupFlags {
        skip_assert_initialized!();
        IconLookupFlags::from_bits_truncate(value)
    }
}

impl StaticType for IconLookupFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_icon_lookup_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for IconLookupFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for IconLookupFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for IconLookupFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct InputHints: u32 {
        const NONE = 0;
        const SPELLCHECK = 1;
        const NO_SPELLCHECK = 2;
        const WORD_COMPLETION = 4;
        const LOWERCASE = 8;
        const UPPERCASE_CHARS = 16;
        const UPPERCASE_WORDS = 32;
        const UPPERCASE_SENTENCES = 64;
        const INHIBIT_OSK = 128;
        const VERTICAL_WRITING = 256;
        #[cfg(any(feature = "v3_22_20", feature = "dox"))]
        const EMOJI = 512;
        #[cfg(any(feature = "v3_22_20", feature = "dox"))]
        const NO_EMOJI = 1024;
    }
}

#[doc(hidden)]
impl ToGlib for InputHints {
    type GlibType = gtk_sys::GtkInputHints;

    fn to_glib(&self) -> gtk_sys::GtkInputHints {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkInputHints> for InputHints {
    fn from_glib(value: gtk_sys::GtkInputHints) -> InputHints {
        skip_assert_initialized!();
        InputHints::from_bits_truncate(value)
    }
}

impl StaticType for InputHints {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_input_hints_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InputHints {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InputHints {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for InputHints {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct JunctionSides: u32 {
        const NONE = 0;
        const CORNER_TOPLEFT = 1;
        const CORNER_TOPRIGHT = 2;
        const CORNER_BOTTOMLEFT = 4;
        const CORNER_BOTTOMRIGHT = 8;
        const TOP = 3;
        const BOTTOM = 12;
        const LEFT = 5;
        const RIGHT = 10;
    }
}

#[doc(hidden)]
impl ToGlib for JunctionSides {
    type GlibType = gtk_sys::GtkJunctionSides;

    fn to_glib(&self) -> gtk_sys::GtkJunctionSides {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkJunctionSides> for JunctionSides {
    fn from_glib(value: gtk_sys::GtkJunctionSides) -> JunctionSides {
        skip_assert_initialized!();
        JunctionSides::from_bits_truncate(value)
    }
}

impl StaticType for JunctionSides {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_junction_sides_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for JunctionSides {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for JunctionSides {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for JunctionSides {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PlacesOpenFlags: u32 {
        const NORMAL = 1;
        const NEW_TAB = 2;
        const NEW_WINDOW = 4;
    }
}

#[doc(hidden)]
impl ToGlib for PlacesOpenFlags {
    type GlibType = gtk_sys::GtkPlacesOpenFlags;

    fn to_glib(&self) -> gtk_sys::GtkPlacesOpenFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkPlacesOpenFlags> for PlacesOpenFlags {
    fn from_glib(value: gtk_sys::GtkPlacesOpenFlags) -> PlacesOpenFlags {
        skip_assert_initialized!();
        PlacesOpenFlags::from_bits_truncate(value)
    }
}

impl StaticType for PlacesOpenFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_places_open_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PlacesOpenFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PlacesOpenFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PlacesOpenFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct RecentFilterFlags: u32 {
        const URI = 1;
        const DISPLAY_NAME = 2;
        const MIME_TYPE = 4;
        const APPLICATION = 8;
        const GROUP = 16;
        const AGE = 32;
    }
}

#[doc(hidden)]
impl ToGlib for RecentFilterFlags {
    type GlibType = gtk_sys::GtkRecentFilterFlags;

    fn to_glib(&self) -> gtk_sys::GtkRecentFilterFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkRecentFilterFlags> for RecentFilterFlags {
    fn from_glib(value: gtk_sys::GtkRecentFilterFlags) -> RecentFilterFlags {
        skip_assert_initialized!();
        RecentFilterFlags::from_bits_truncate(value)
    }
}

impl StaticType for RecentFilterFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_recent_filter_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RecentFilterFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RecentFilterFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for RecentFilterFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct RegionFlags: u32 {
        const EVEN = 1;
        const ODD = 2;
        const FIRST = 4;
        const LAST = 8;
        const ONLY = 16;
        const SORTED = 32;
    }
}

#[doc(hidden)]
impl ToGlib for RegionFlags {
    type GlibType = gtk_sys::GtkRegionFlags;

    fn to_glib(&self) -> gtk_sys::GtkRegionFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkRegionFlags> for RegionFlags {
    fn from_glib(value: gtk_sys::GtkRegionFlags) -> RegionFlags {
        skip_assert_initialized!();
        RegionFlags::from_bits_truncate(value)
    }
}

impl StaticType for RegionFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_region_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RegionFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RegionFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for RegionFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct StateFlags: u32 {
        const NORMAL = 0;
        const ACTIVE = 1;
        const PRELIGHT = 2;
        const SELECTED = 4;
        const INSENSITIVE = 8;
        const INCONSISTENT = 16;
        const FOCUSED = 32;
        const BACKDROP = 64;
        const DIR_LTR = 128;
        const DIR_RTL = 256;
        const LINK = 512;
        const VISITED = 1024;
        const CHECKED = 2048;
        const DROP_ACTIVE = 4096;
    }
}

#[doc(hidden)]
impl ToGlib for StateFlags {
    type GlibType = gtk_sys::GtkStateFlags;

    fn to_glib(&self) -> gtk_sys::GtkStateFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkStateFlags> for StateFlags {
    fn from_glib(value: gtk_sys::GtkStateFlags) -> StateFlags {
        skip_assert_initialized!();
        StateFlags::from_bits_truncate(value)
    }
}

impl StaticType for StateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_state_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StateFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StateFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for StateFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
bitflags! {
    pub struct StyleContextPrintFlags: u32 {
        const NONE = 0;
        const RECURSE = 1;
        const SHOW_STYLE = 2;
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for StyleContextPrintFlags {
    type GlibType = gtk_sys::GtkStyleContextPrintFlags;

    fn to_glib(&self) -> gtk_sys::GtkStyleContextPrintFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gtk_sys::GtkStyleContextPrintFlags> for StyleContextPrintFlags {
    fn from_glib(value: gtk_sys::GtkStyleContextPrintFlags) -> StyleContextPrintFlags {
        skip_assert_initialized!();
        StyleContextPrintFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
impl StaticType for StyleContextPrintFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_style_context_print_flags_get_type()) }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
impl<'a> FromValueOptional<'a> for StyleContextPrintFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
impl<'a> FromValue<'a> for StyleContextPrintFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
impl SetValue for StyleContextPrintFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct TargetFlags: u32 {
        const SAME_APP = 1;
        const SAME_WIDGET = 2;
        const OTHER_APP = 4;
        const OTHER_WIDGET = 8;
    }
}

#[doc(hidden)]
impl ToGlib for TargetFlags {
    type GlibType = gtk_sys::GtkTargetFlags;

    fn to_glib(&self) -> gtk_sys::GtkTargetFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkTargetFlags> for TargetFlags {
    fn from_glib(value: gtk_sys::GtkTargetFlags) -> TargetFlags {
        skip_assert_initialized!();
        TargetFlags::from_bits_truncate(value)
    }
}

impl StaticType for TargetFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_target_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TargetFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TargetFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for TargetFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct TextSearchFlags: u32 {
        const VISIBLE_ONLY = 1;
        const TEXT_ONLY = 2;
        const CASE_INSENSITIVE = 4;
    }
}

#[doc(hidden)]
impl ToGlib for TextSearchFlags {
    type GlibType = gtk_sys::GtkTextSearchFlags;

    fn to_glib(&self) -> gtk_sys::GtkTextSearchFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkTextSearchFlags> for TextSearchFlags {
    fn from_glib(value: gtk_sys::GtkTextSearchFlags) -> TextSearchFlags {
        skip_assert_initialized!();
        TextSearchFlags::from_bits_truncate(value)
    }
}

impl StaticType for TextSearchFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_text_search_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TextSearchFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TextSearchFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for TextSearchFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ToolPaletteDragTargets: u32 {
        const ITEMS = 1;
        const GROUPS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for ToolPaletteDragTargets {
    type GlibType = gtk_sys::GtkToolPaletteDragTargets;

    fn to_glib(&self) -> gtk_sys::GtkToolPaletteDragTargets {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkToolPaletteDragTargets> for ToolPaletteDragTargets {
    fn from_glib(value: gtk_sys::GtkToolPaletteDragTargets) -> ToolPaletteDragTargets {
        skip_assert_initialized!();
        ToolPaletteDragTargets::from_bits_truncate(value)
    }
}

impl StaticType for ToolPaletteDragTargets {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_tool_palette_drag_targets_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ToolPaletteDragTargets {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ToolPaletteDragTargets {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ToolPaletteDragTargets {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct TreeModelFlags: u32 {
        const ITERS_PERSIST = 1;
        const LIST_ONLY = 2;
    }
}

#[doc(hidden)]
impl ToGlib for TreeModelFlags {
    type GlibType = gtk_sys::GtkTreeModelFlags;

    fn to_glib(&self) -> gtk_sys::GtkTreeModelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gtk_sys::GtkTreeModelFlags> for TreeModelFlags {
    fn from_glib(value: gtk_sys::GtkTreeModelFlags) -> TreeModelFlags {
        skip_assert_initialized!();
        TreeModelFlags::from_bits_truncate(value)
    }
}

impl StaticType for TreeModelFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gtk_sys::gtk_tree_model_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TreeModelFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TreeModelFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for TreeModelFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

