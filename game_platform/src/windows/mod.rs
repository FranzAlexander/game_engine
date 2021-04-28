mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE}
};

pub fn create_message_window() {
    unsafe {
        MessageBoxA(None, "Hello", "World", MESSAGEBOX_STYLE::MB_OK);
    }
}
