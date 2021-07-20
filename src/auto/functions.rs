// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Accessible;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;


//#[doc(alias = "atspi_dbus_connection_setup_with_g_main")]
//pub fn dbus_connection_setup_with_g_main(connection: /*Ignored*/&mut dbus::Connection, context: /*Ignored*/&glib::MainContext) {
//    unsafe { TODO: call ffi:atspi_dbus_connection_setup_with_g_main() }
//}

//#[doc(alias = "atspi_deregister_device_event_listener")]
//pub fn deregister_device_event_listener(listener: /*Ignored*/&DeviceListener, filter: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:atspi_deregister_device_event_listener() }
//}

//#[doc(alias = "atspi_deregister_keystroke_listener")]
//pub fn deregister_keystroke_listener(listener: /*Ignored*/&DeviceListener, key_set: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 52 }, modmask: KeyMaskType, event_types: KeyEventMask) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:atspi_deregister_keystroke_listener() }
//}

#[doc(alias = "atspi_exit")]
pub fn exit() -> i32 {
    unsafe {
        ffi::atspi_exit()
    }
}

//#[doc(alias = "atspi_generate_keyboard_event")]
//pub fn generate_keyboard_event(keyval: libc::c_long, keystring: Option<&str>, synth_type: /*Ignored*/KeySynthType) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:atspi_generate_keyboard_event() }
//}

#[doc(alias = "atspi_generate_mouse_event")]
pub fn generate_mouse_event(x: libc::c_long, y: libc::c_long, name: &str) -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let _ = ffi::atspi_generate_mouse_event(x, y, name.to_glib_none().0, &mut error);
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

//#[doc(alias = "atspi_get_a11y_bus")]
//#[doc(alias = "get_a11y_bus")]
//pub fn a11y_bus() -> /*Ignored*/Option<dbus::Connection> {
//    unsafe { TODO: call ffi:atspi_get_a11y_bus() }
//}

#[doc(alias = "atspi_get_desktop")]
#[doc(alias = "get_desktop")]
pub fn desktop(i: i32) -> Option<Accessible> {
    unsafe {
        from_glib_full(ffi::atspi_get_desktop(i))
    }
}

#[doc(alias = "atspi_get_desktop_count")]
#[doc(alias = "get_desktop_count")]
pub fn desktop_count() -> i32 {
    unsafe {
        ffi::atspi_get_desktop_count()
    }
}

//#[doc(alias = "atspi_get_desktop_list")]
//#[doc(alias = "get_desktop_list")]
//pub fn desktop_list() -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 17 } {
//    unsafe { TODO: call ffi:atspi_get_desktop_list() }
//}

#[doc(alias = "atspi_init")]
pub fn init() -> i32 {
    unsafe {
        ffi::atspi_init()
    }
}

#[doc(alias = "atspi_is_initialized")]
pub fn is_initialized() -> bool {
    unsafe {
        from_glib(ffi::atspi_is_initialized())
    }
}

//#[doc(alias = "atspi_register_device_event_listener")]
//pub fn register_device_event_listener(listener: /*Ignored*/&DeviceListener, event_types: DeviceEventMask, filter: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:atspi_register_device_event_listener() }
//}

//#[doc(alias = "atspi_register_keystroke_listener")]
//pub fn register_keystroke_listener(listener: /*Ignored*/&DeviceListener, key_set: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 52 }, modmask: KeyMaskType, event_types: KeyEventMask, sync_type: /*Ignored*/KeyListenerSyncType) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:atspi_register_keystroke_listener() }
//}

//#[doc(alias = "atspi_set_main_context")]
//pub fn set_main_context(cnx: /*Ignored*/&glib::MainContext) {
//    unsafe { TODO: call ffi:atspi_set_main_context() }
//}

#[doc(alias = "atspi_set_reference_window")]
pub fn set_reference_window<P: IsA<Accessible>>(accessible: &P) {
    unsafe {
        ffi::atspi_set_reference_window(accessible.as_ref().to_glib_none().0);
    }
}

#[doc(alias = "atspi_set_timeout")]
pub fn set_timeout(val: i32, startup_time: i32) {
    unsafe {
        ffi::atspi_set_timeout(val, startup_time);
    }
}
