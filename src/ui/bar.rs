use eframe::egui::{self, Color32, FontId, RichText, Context, TopBottomPanel};

pub fn draw_bar(ctx: &Context) {
    TopBottomPanel::top("bar").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.add_space(10.0);
            ui.label(RichText::new("  Winux").font(FontId::proportional(18.0)).color(Color32::from_rgb(0, 200, 255)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new("🕒 12:00").font(FontId::proportional(16.0)));
            });
        });
    });
}
