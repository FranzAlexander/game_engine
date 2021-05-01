fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{
            GetModuleHandleA, DXGI_STATUS_OCCLUDED, HINSTANCE, LRESULT,
            PSTR,
        },
         Windows::Win32::Gdi::{BeginPaint, EndPaint, ValidateRect, PAINTSTRUCT, FillRect, GetStockObject},

        Windows::Win32::WindowsAndMessaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PeekMessageA,
            PostQuitMessage, RegisterClassA, CREATESTRUCTA, HWND, LPARAM, MINMAXINFO, MSG, WNDCLASSA,
            WPARAM, LoadCursorW, IDC_ARROW, SIZE_MINIMIZED, WM_DESTROY, WM_ACTIVATE, WM_DISPLAYCHANGE,
            WM_NCCREATE, WM_PAINT, WM_QUIT, WM_SIZE, WM_USER, WNDCLASS_STYLES,
            CW_USEDEFAULT, IDC_HAND, SetWindowLongA, SetWindowLongPtrA, GetWindowLongA, GetWindowLongPtrA,
        },
    )
}
