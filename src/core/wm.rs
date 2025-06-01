use crate::system::window::get_visible_windows;

pub fn tile_windows() {
    let windows = get_visible_windows();
    let screen_width = 1920;
    let screen_height = 1080;
    let count = windows.len().max(1);
    let tile_width = screen_width / count as i32;

    for (i, hwnd) in windows.iter().enumerate() {
        let x = (i as i32) * tile_width;
        let _ = crate::system::window::move_resize_window(*hwnd, x, 0, tile_width, screen_height);
    }
}
