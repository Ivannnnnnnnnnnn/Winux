mod ui;
mod core;
mod system;
mod utils;

use eframe::egui::Context;
use eframe::{App, NativeOptions};
use core::keybinds::start_key_listener;
use ui::{
    bar::draw_bar,
    launcher::{draw_launcher},
    workspace::draw_workspace,
};
use std::sync::{Arc, Mutex};

struct WinuxApp {
    launcher_open: Arc<Mutex<bool>>,
}

impl Default for WinuxApp {
    fn default() -> Self {
        Self {
            launcher_open: Arc::new(Mutex::new(false)),
        }
    }
}

impl App for WinuxApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        draw_bar(ctx);
        draw_workspace(ctx);

        {
            let mut open = self.launcher_open.lock().unwrap();
            if *open {
                draw_launcher(ctx);  // <- changed: no &mut open arg anymore
            }
        }

        ctx.request_repaint();
    }
}

fn main() {
    let options = NativeOptions {
        vsync: true,
        // initial_window_size is deprecated/removed; remove it or handle differently
        ..Default::default()
    };

    let launcher_open = Arc::new(Mutex::new(false));

    {
        let launcher_clone = launcher_open.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                start_key_listener(move |key| {
                    if key.to_lowercase() == "super" || key.to_lowercase() == "meta" {
                        let mut open = launcher_clone.lock().unwrap();
                        *open = !*open;
                    }
                })
                .await;
            });
        });
    }

    eframe::run_native(
        "winux-bar",
        options,
        Box::new(|_cc| Box::new(WinuxApp { launcher_open })),
    )
    .unwrap();
}
