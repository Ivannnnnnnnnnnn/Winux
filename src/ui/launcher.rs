use eframe::egui::{self, Context, Key, TextEdit, Window};

static mut LAUNCHER_OPEN: bool = false;

pub fn toggle_launcher() {
    unsafe {
        LAUNCHER_OPEN = !LAUNCHER_OPEN;
    }
}

pub fn draw_launcher(ctx: &Context) {
    unsafe {
        if LAUNCHER_OPEN {
            Window::new("Launcher")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add(TextEdit::singleline(&mut String::new()).hint_text("Search apps..."));
                });

            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                LAUNCHER_OPEN = false;
            }
        }
    }
}
