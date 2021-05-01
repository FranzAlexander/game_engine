mod bindings;

use bindings::{
    Windows::Win32::Gdi::*, Windows::Win32::SystemServices::*,
    Windows::Win32::WindowsAndMessaging::*,
};

use windows::*;

pub struct Window {
    hwnd: HWND,
    visable: bool,
}

impl Window {
    pub fn new() -> Result<Self> {
        Ok(Window {
            hwnd: HWND(0),
            visable: false,
        })
    }

    pub fn create_window(&mut self) -> Result<()> {
        unsafe {
            let instance = HINSTANCE(GetModuleHandleA(None));
            debug_assert!(instance.0 != 0);

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_HAND),
                hInstance: instance,
                lpszClassName: PSTR(b"window\0".as_ptr() as _),
                style: WNDCLASS_STYLES::CS_HREDRAW | WNDCLASS_STYLES::CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(
                Default::default(),
                "window",
                "Sample Window",
                WINDOW_STYLE::WS_OVERLAPPEDWINDOW | WINDOW_STYLE::WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                self as *mut _ as _,
            );

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.hwnd);
            let mut message = MSG::default();

            loop {
                if self.visable {
                    while PeekMessageA(
                        &mut message,
                        None,
                        0,
                        0,
                        PEEK_MESSAGE_REMOVE_TYPE::PM_REMOVE,
                    )
                    .into()
                    {
                        if message.message == WM_QUIT {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                } else {
                    GetMessageA(&mut message, None, 0, 0);

                    if message.message == WM_QUIT {
                        return Ok(());
                    }

                    DispatchMessageA(&message);
                }
            }
        }
    }

    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    ValidateRect(self.hwnd, std::ptr::null());
                    LRESULT(0)
                }
                WM_ACTIVATE => {
                    self.visable = true; // TODO: unpack !HIWORD(wparam);
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(self.hwnd, message, wparam, lparam),
            }
        }
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).hwnd = window;

                SetWindowLong(window, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLong(window, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcA(window, message, wparam, lparam)
        }
    }
}
// fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
//     match message {
//         WM_PAINT => {
//             let mut ps = PAINTSTRUCT::default();
//             BeginPaint(self.handle, &mut ps);
//             self.render().unwrap();
//             EndPaint(self.handle, &ps);
//             LRESULT(0)
// }
// extern "system" fn wndproc(
//     window: HWND,
//     message: u32,
//     wparam: WPARAM,
//     lparam: LPARAM,
// ) -> LRESULT {
//     unsafe {
//         match message {
//             WM_PAINT => {
//                 let mut ps = PAINTSTRUCT::default();
//                 BeginPaint(self.h)
//             }
//         }
//     }
// }

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}
