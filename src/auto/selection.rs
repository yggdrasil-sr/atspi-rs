// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Accessible;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "AtspiSelection")]
    pub struct Selection(Interface<ffi::AtspiSelection>);

    match fn {
        type_ => || ffi::atspi_selection_get_type(),
    }
}

pub const NONE_SELECTION: Option<&Selection> = None;

pub trait SelectionExt: 'static {
    #[doc(alias = "atspi_selection_clear_selection")]
    fn clear_selection(&self) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_selection_deselect_child")]
    fn deselect_child(&self, child_index: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_selection_deselect_selected_child")]
    fn deselect_selected_child(&self, selected_child_index: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_selection_get_n_selected_children")]
    #[doc(alias = "get_n_selected_children")]
    fn n_selected_children(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_selection_get_selected_child")]
    #[doc(alias = "get_selected_child")]
    fn selected_child(&self, selected_child_index: i32) -> Result<Accessible, glib::Error>;

    #[doc(alias = "atspi_selection_is_child_selected")]
    fn is_child_selected(&self, child_index: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_selection_select_all")]
    fn select_all(&self) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_selection_select_child")]
    fn select_child(&self, child_index: i32) -> Result<(), glib::Error>;
}

impl<O: IsA<Selection>> SelectionExt for O {
    fn clear_selection(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_selection_clear_selection(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn deselect_child(&self, child_index: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_selection_deselect_child(self.as_ref().to_glib_none().0, child_index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn deselect_selected_child(&self, selected_child_index: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_selection_deselect_selected_child(self.as_ref().to_glib_none().0, selected_child_index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn n_selected_children(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_selection_get_n_selected_children(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn selected_child(&self, selected_child_index: i32) -> Result<Accessible, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_selection_get_selected_child(self.as_ref().to_glib_none().0, selected_child_index, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_child_selected(&self, child_index: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_selection_is_child_selected(self.as_ref().to_glib_none().0, child_index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_all(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_selection_select_all(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn select_child(&self, child_index: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_selection_select_child(self.as_ref().to_glib_none().0, child_index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Selection")
    }
}
