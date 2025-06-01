use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::*;

pub unsafe fn set_always_on_top(hwnd: HWND) {
    SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
}

pub unsafe fn set_always_on_bottom(hwnd: HWND) {
    SetWindowPos(hwnd, HWND_BOTTOM, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
}

pub unsafe fn hide_window(hwnd: HWND) {
    ShowWindow(hwnd, SW_HIDE);
}

pub unsafe fn show_window(hwnd: HWND) {
    ShowWindow(hwnd, SW_SHOW);
}
