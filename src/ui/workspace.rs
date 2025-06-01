use eframe::egui::{self, CentralPanel, Context};

pub fn draw_workspace(ctx: &Context) {
    CentralPanel::default().show(ctx, |_ui| {
        // reserved for virtual desktop, wallpaper, etc.
    });
}
