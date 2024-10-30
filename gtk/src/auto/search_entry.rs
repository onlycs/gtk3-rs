// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{Align,Buildable,CellEditable,Container,Editable,Entry,EntryBuffer,EntryCompletion,InputHints,InputPurpose,Widget};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GtkSearchEntry")]
    pub struct SearchEntry(Object<ffi::GtkSearchEntry, ffi::GtkSearchEntryClass>) @extends Entry, Widget, @implements Buildable, CellEditable, Editable;

    match fn {
        type_ => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
        pub const NONE: Option<&'static SearchEntry> = None;
    

    #[doc(alias = "gtk_search_entry_new")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_entry_new()).unsafe_cast()
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`SearchEntry`] objects.
            ///
            /// This method returns an instance of [`SearchEntryBuilder`](crate::builders::SearchEntryBuilder) which can be used to create [`SearchEntry`] objects.
            pub fn builder() -> SearchEntryBuilder {
                SearchEntryBuilder::new()
            }
        
}

impl Default for SearchEntry {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`SearchEntry`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SearchEntryBuilder {
            builder: glib::object::ObjectBuilder<'static, SearchEntry>,
        }

        impl SearchEntryBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn activates_default(self, activates_default: bool) -> Self {
                            Self { builder: self.builder.property("activates-default", activates_default), }
                        }

                            pub fn attributes(self, attributes: &pango::AttrList) -> Self {
                            Self { builder: self.builder.property("attributes", attributes.clone()), }
                        }

                            pub fn buffer(self, buffer: &impl IsA<EntryBuffer>) -> Self {
                            Self { builder: self.builder.property("buffer", buffer.clone().upcast()), }
                        }

                            pub fn caps_lock_warning(self, caps_lock_warning: bool) -> Self {
                            Self { builder: self.builder.property("caps-lock-warning", caps_lock_warning), }
                        }

                            pub fn completion(self, completion: &impl IsA<EntryCompletion>) -> Self {
                            Self { builder: self.builder.property("completion", completion.clone().upcast()), }
                        }

                            pub fn editable(self, editable: bool) -> Self {
                            Self { builder: self.builder.property("editable", editable), }
                        }

                            pub fn enable_emoji_completion(self, enable_emoji_completion: bool) -> Self {
                            Self { builder: self.builder.property("enable-emoji-completion", enable_emoji_completion), }
                        }

                            pub fn has_frame(self, has_frame: bool) -> Self {
                            Self { builder: self.builder.property("has-frame", has_frame), }
                        }

                            pub fn im_module(self, im_module: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("im-module", im_module.into()), }
                        }

                            pub fn input_hints(self, input_hints: InputHints) -> Self {
                            Self { builder: self.builder.property("input-hints", input_hints), }
                        }

                            pub fn input_purpose(self, input_purpose: InputPurpose) -> Self {
                            Self { builder: self.builder.property("input-purpose", input_purpose), }
                        }

                            pub fn invisible_char(self, invisible_char: u32) -> Self {
                            Self { builder: self.builder.property("invisible-char", invisible_char), }
                        }

                            pub fn invisible_char_set(self, invisible_char_set: bool) -> Self {
                            Self { builder: self.builder.property("invisible-char-set", invisible_char_set), }
                        }

                            pub fn max_length(self, max_length: i32) -> Self {
                            Self { builder: self.builder.property("max-length", max_length), }
                        }

                            pub fn max_width_chars(self, max_width_chars: i32) -> Self {
                            Self { builder: self.builder.property("max-width-chars", max_width_chars), }
                        }

                            pub fn overwrite_mode(self, overwrite_mode: bool) -> Self {
                            Self { builder: self.builder.property("overwrite-mode", overwrite_mode), }
                        }

                            pub fn placeholder_text(self, placeholder_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("placeholder-text", placeholder_text.into()), }
                        }

                            pub fn populate_all(self, populate_all: bool) -> Self {
                            Self { builder: self.builder.property("populate-all", populate_all), }
                        }

                            pub fn primary_icon_activatable(self, primary_icon_activatable: bool) -> Self {
                            Self { builder: self.builder.property("primary-icon-activatable", primary_icon_activatable), }
                        }

                            pub fn primary_icon_gicon(self, primary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
                            Self { builder: self.builder.property("primary-icon-gicon", primary_icon_gicon.clone().upcast()), }
                        }

                            pub fn primary_icon_name(self, primary_icon_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("primary-icon-name", primary_icon_name.into()), }
                        }

                            pub fn primary_icon_pixbuf(self, primary_icon_pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
                            Self { builder: self.builder.property("primary-icon-pixbuf", primary_icon_pixbuf.clone()), }
                        }

                            pub fn primary_icon_sensitive(self, primary_icon_sensitive: bool) -> Self {
                            Self { builder: self.builder.property("primary-icon-sensitive", primary_icon_sensitive), }
                        }

                            pub fn primary_icon_tooltip_markup(self, primary_icon_tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("primary-icon-tooltip-markup", primary_icon_tooltip_markup.into()), }
                        }

                            pub fn primary_icon_tooltip_text(self, primary_icon_tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("primary-icon-tooltip-text", primary_icon_tooltip_text.into()), }
                        }

                            pub fn progress_fraction(self, progress_fraction: f64) -> Self {
                            Self { builder: self.builder.property("progress-fraction", progress_fraction), }
                        }

                            pub fn progress_pulse_step(self, progress_pulse_step: f64) -> Self {
                            Self { builder: self.builder.property("progress-pulse-step", progress_pulse_step), }
                        }

                            pub fn secondary_icon_activatable(self, secondary_icon_activatable: bool) -> Self {
                            Self { builder: self.builder.property("secondary-icon-activatable", secondary_icon_activatable), }
                        }

                            pub fn secondary_icon_gicon(self, secondary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
                            Self { builder: self.builder.property("secondary-icon-gicon", secondary_icon_gicon.clone().upcast()), }
                        }

                            pub fn secondary_icon_name(self, secondary_icon_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("secondary-icon-name", secondary_icon_name.into()), }
                        }

                            pub fn secondary_icon_pixbuf(self, secondary_icon_pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
                            Self { builder: self.builder.property("secondary-icon-pixbuf", secondary_icon_pixbuf.clone()), }
                        }

                            pub fn secondary_icon_sensitive(self, secondary_icon_sensitive: bool) -> Self {
                            Self { builder: self.builder.property("secondary-icon-sensitive", secondary_icon_sensitive), }
                        }

                            pub fn secondary_icon_tooltip_markup(self, secondary_icon_tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("secondary-icon-tooltip-markup", secondary_icon_tooltip_markup.into()), }
                        }

                            pub fn secondary_icon_tooltip_text(self, secondary_icon_tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("secondary-icon-tooltip-text", secondary_icon_tooltip_text.into()), }
                        }

                            pub fn show_emoji_icon(self, show_emoji_icon: bool) -> Self {
                            Self { builder: self.builder.property("show-emoji-icon", show_emoji_icon), }
                        }

                            pub fn tabs(self, tabs: &pango::TabArray) -> Self {
                            Self { builder: self.builder.property("tabs", tabs), }
                        }

                            pub fn text(self, text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("text", text.into()), }
                        }

                            pub fn truncate_multiline(self, truncate_multiline: bool) -> Self {
                            Self { builder: self.builder.property("truncate-multiline", truncate_multiline), }
                        }

                            pub fn visibility(self, visibility: bool) -> Self {
                            Self { builder: self.builder.property("visibility", visibility), }
                        }

                            pub fn width_chars(self, width_chars: i32) -> Self {
                            Self { builder: self.builder.property("width-chars", width_chars), }
                        }

                            pub fn xalign(self, xalign: f32) -> Self {
                            Self { builder: self.builder.property("xalign", xalign), }
                        }

                            pub fn app_paintable(self, app_paintable: bool) -> Self {
                            Self { builder: self.builder.property("app-paintable", app_paintable), }
                        }

                            pub fn can_default(self, can_default: bool) -> Self {
                            Self { builder: self.builder.property("can-default", can_default), }
                        }

                            pub fn can_focus(self, can_focus: bool) -> Self {
                            Self { builder: self.builder.property("can-focus", can_focus), }
                        }

                            pub fn events(self, events: gdk::EventMask) -> Self {
                            Self { builder: self.builder.property("events", events), }
                        }

                            pub fn expand(self, expand: bool) -> Self {
                            Self { builder: self.builder.property("expand", expand), }
                        }

                            pub fn focus_on_click(self, focus_on_click: bool) -> Self {
                            Self { builder: self.builder.property("focus-on-click", focus_on_click), }
                        }

                            pub fn halign(self, halign: Align) -> Self {
                            Self { builder: self.builder.property("halign", halign), }
                        }

                            pub fn has_default(self, has_default: bool) -> Self {
                            Self { builder: self.builder.property("has-default", has_default), }
                        }

                            pub fn has_focus(self, has_focus: bool) -> Self {
                            Self { builder: self.builder.property("has-focus", has_focus), }
                        }

                            pub fn has_tooltip(self, has_tooltip: bool) -> Self {
                            Self { builder: self.builder.property("has-tooltip", has_tooltip), }
                        }

                            pub fn height_request(self, height_request: i32) -> Self {
                            Self { builder: self.builder.property("height-request", height_request), }
                        }

                            pub fn hexpand(self, hexpand: bool) -> Self {
                            Self { builder: self.builder.property("hexpand", hexpand), }
                        }

                            pub fn hexpand_set(self, hexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("hexpand-set", hexpand_set), }
                        }

                            pub fn is_focus(self, is_focus: bool) -> Self {
                            Self { builder: self.builder.property("is-focus", is_focus), }
                        }

                            pub fn margin(self, margin: i32) -> Self {
                            Self { builder: self.builder.property("margin", margin), }
                        }

                            pub fn margin_bottom(self, margin_bottom: i32) -> Self {
                            Self { builder: self.builder.property("margin-bottom", margin_bottom), }
                        }

                            pub fn margin_end(self, margin_end: i32) -> Self {
                            Self { builder: self.builder.property("margin-end", margin_end), }
                        }

                            pub fn margin_start(self, margin_start: i32) -> Self {
                            Self { builder: self.builder.property("margin-start", margin_start), }
                        }

                            pub fn margin_top(self, margin_top: i32) -> Self {
                            Self { builder: self.builder.property("margin-top", margin_top), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn no_show_all(self, no_show_all: bool) -> Self {
                            Self { builder: self.builder.property("no-show-all", no_show_all), }
                        }

                            pub fn opacity(self, opacity: f64) -> Self {
                            Self { builder: self.builder.property("opacity", opacity), }
                        }

                            pub fn parent(self, parent: &impl IsA<Container>) -> Self {
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

                            pub fn receives_default(self, receives_default: bool) -> Self {
                            Self { builder: self.builder.property("receives-default", receives_default), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-markup", tooltip_markup.into()), }
                        }

                            pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("tooltip-text", tooltip_text.into()), }
                        }

                            pub fn valign(self, valign: Align) -> Self {
                            Self { builder: self.builder.property("valign", valign), }
                        }

                            pub fn vexpand(self, vexpand: bool) -> Self {
                            Self { builder: self.builder.property("vexpand", vexpand), }
                        }

                            pub fn vexpand_set(self, vexpand_set: bool) -> Self {
                            Self { builder: self.builder.property("vexpand-set", vexpand_set), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width_request(self, width_request: i32) -> Self {
                            Self { builder: self.builder.property("width-request", width_request), }
                        }

                            pub fn editing_canceled(self, editing_canceled: bool) -> Self {
                            Self { builder: self.builder.property("editing-canceled", editing_canceled), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`SearchEntry`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SearchEntry {
    self.builder.build() }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SearchEntry>> Sealed for T {}
}

pub trait SearchEntryExt: IsA<SearchEntry> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_search_entry_handle_event")]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_entry_handle_event(self.as_ref().to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    #[doc(alias = "next-match")]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_match_trampoline<P: IsA<SearchEntry>, F: Fn(&P) + 'static>(this: *mut ffi::GtkSearchEntry, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"next-match\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(next_match_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_next_match(&self) {
        self.emit_by_name::<()>("next-match", &[]);
    }

    #[doc(alias = "previous-match")]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn previous_match_trampoline<P: IsA<SearchEntry>, F: Fn(&P) + 'static>(this: *mut ffi::GtkSearchEntry, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"previous-match\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(previous_match_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_previous_match(&self) {
        self.emit_by_name::<()>("previous-match", &[]);
    }

    #[doc(alias = "search-changed")]
    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_changed_trampoline<P: IsA<SearchEntry>, F: Fn(&P) + 'static>(this: *mut ffi::GtkSearchEntry, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"search-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(search_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "stop-search")]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_search_trampoline<P: IsA<SearchEntry>, F: Fn(&P) + 'static>(this: *mut ffi::GtkSearchEntry, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stop-search\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(stop_search_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn emit_stop_search(&self) {
        self.emit_by_name::<()>("stop-search", &[]);
    }
}

impl<O: IsA<SearchEntry>> SearchEntryExt for O {}
