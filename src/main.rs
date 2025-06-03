use chrono::Local;
use eframe::{egui, App, Frame};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};

struct LinuxStyleShellApp {
    show_menu: bool,
    show_file_manager: bool,
    show_terminal: bool,
    show_browser: bool,
    show_settings: bool,
    current_dir: PathBuf,
    terminal_output: Arc<Mutex<Vec<String>>>,
    terminal_input: String,
    terminal_tx: Option<Sender<String>>,
    terminal_rx: Option<Receiver<String>>,
    bg_color: egui::Color32,
}

impl Default for LinuxStyleShellApp {
    fn default() -> Self {
        let bg_color = egui::Color32::from_rgb(40, 42, 54);
        let current_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let terminal_output = Arc::new(Mutex::new(Vec::new()));

        let (tx, cmd_rx) = mpsc::channel::<String>();
        let (out_tx, out_rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            #[cfg(target_os = "windows")]
            let mut shell = Command::new("cmd")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to spawn shell");

            #[cfg(not(target_os = "windows"))]
            let mut shell = Command::new("sh")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to spawn shell");

            let mut stdin = shell.stdin.take().expect("Failed to open stdin");
            let stdout = shell.stdout.take().expect("Failed to open stdout");
            let stderr = shell.stderr.take().expect("Failed to open stderr");

            let out_tx_clone = out_tx.clone();

            let stdout_reader = BufReader::new(stdout);
            thread::spawn(move || {
                for line in stdout_reader.lines() {
                    if let Ok(line) = line {
                        let _ = out_tx_clone.send(line);
                    }
                }
            });

            let stderr_reader = BufReader::new(stderr);
            let out_tx_clone = out_tx.clone();
            thread::spawn(move || {
                for line in stderr_reader.lines() {
                    if let Ok(line) = line {
                        let _ = out_tx_clone.send(line);
                    }
                }
            });

            for cmd in cmd_rx {
                let cmd_with_newline = if cfg!(target_os = "windows") {
                    format!("{}\r\n", cmd)
                } else {
                    format!("{}\n", cmd)
                };
                if let Err(_) = stdin.write_all(cmd_with_newline.as_bytes()) {
                    break;
                }
            }
        });

        Self {
            show_menu: false,
            show_file_manager: false,
            show_terminal: false,
            show_browser: false,
            show_settings: false,
            current_dir,
            terminal_output,
            terminal_input: String::new(),
            terminal_tx: Some(tx),
            terminal_rx: Some(out_rx),
            bg_color,
        }
    }
}

impl App for LinuxStyleShellApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(self.bg_color))
            .show(ctx, |ui| {
                let available_rect = ui.max_rect();
                let top_bar_height = 30.0;
                let top_bar_rect = egui::Rect::from_min_size(
                    available_rect.min,
                    egui::vec2(available_rect.width(), top_bar_height),
                );
                ui.allocate_ui_at_rect(top_bar_rect, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.label(" LinuxStyleShell");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let time_str = Local::now().format("%H:%M:%S").to_string();
                            ui.label(time_str);
                            ui.separator();
                            if ui.button("☰").clicked() {
                                self.show_menu = !self.show_menu;
                            }
                        });
                    });
                });

                if self.show_menu {
                    egui::Area::new("menu_area")
                        .fixed_pos(egui::pos2(
                            available_rect.right() - 100.0,
                            available_rect.top() + top_bar_height,
                        ))
                        .show(ctx, |ui| {
                            ui.set_min_size(egui::vec2(100.0, 140.0));
                            ui.vertical(|ui| {
                                if ui.button("Terminal").clicked() {
                                    self.show_menu = false;
                                    self.show_terminal = true;
                                }
                                if ui.button("Files").clicked() {
                                    self.show_menu = false;
                                    self.show_file_manager = true;
                                }
                                if ui.button("Settings").clicked() {
                                    self.show_menu = false;
                                    self.show_settings = true;
                                }
                                if ui.button("Logout").clicked() {
                                    frame.quit();
                                }
                            });
                        });
                }

                let dock_height = 60.0;
                let dock_rect = egui::Rect::from_min_size(
                    egui::pos2(available_rect.left(), available_rect.bottom() - dock_height),
                    egui::vec2(available_rect.width(), dock_height),
                );
                ui.allocate_ui_at_rect(dock_rect, |ui| {
                    ui.horizontal_centered(|ui| {
                        let icon_size = egui::vec2(48.0, 48.0);
                        if ui.add_sized(icon_size, egui::Button::new("🐧")).clicked() {
                            self.show_terminal = true;
                        }
                        if ui.add_sized(icon_size, egui::Button::new("📁")).clicked() {
                            self.show_file_manager = true;
                        }
                        if ui.add_sized(icon_size, egui::Button::new("🌐")).clicked() {
                            self.show_browser = false;
                            let _ = open::that("https://www.google.com");
                        }
                        if ui.add_sized(icon_size, egui::Button::new("⚙")).clicked() {
                            self.show_settings = true;
                        }
                    });
                });
            });

        if self.show_file_manager {
            egui::Window::new("File Manager")
                .resizable(true)
                .default_size(egui::vec2(600.0, 400.0))
                .show(ctx, |ui| {
                    if self.current_dir.parent().is_some() {
                        if ui.button("⬅ Back").clicked() {
                            self.current_dir.pop();
                        }
                    }

                    ui.label(format!("Current directory: {}", self.current_dir.display()));

                    if let Ok(entries) = std::fs::read_dir(&self.current_dir) {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                                if path.is_dir() {
                                    if ui.button(format!("📁 {}", file_name)).clicked() {
                                        self.current_dir = path;
                                    }
                                } else {
                                    ui.label(format!("📄 {}", file_name));
                                }
                            }
                        });
                    } else {
                        ui.label("Cannot read directory");
                    }

                    if ui.button("Close").clicked() {
                        self.show_file_manager = false;
                    }
                });
        }

        if self.show_terminal {
            egui::Window::new("Terminal")
                .resizable(true)
                .default_size(egui::vec2(600.0, 400.0))
                .show(ctx, |ui| {
                    let output = self.terminal_output.lock().unwrap();
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for line in output.iter() {
                            ui.label(line);
                        }
                    });

                    let input = &mut self.terminal_input;
                    let response = ui.text_edit_singleline(input);

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Some(tx) = &self.terminal_tx {
                            let cmd = input.trim();
                            if !cmd.is_empty() {
                                drop(output);
                                let mut output = self.terminal_output.lock().unwrap();
                                output.push(format!("> {}", cmd));
                                let _ = tx.send(cmd.to_string());
                                *input = String::new();
                            }
                        }
                    }

                    if ui.button("Close").clicked() {
                        self.show_terminal = false;
                    }
                });
        }

        if let Some(rx) = &self.terminal_rx {
            while let Ok(line) = rx.try_recv() {
                let mut output = self.terminal_output.lock().unwrap();
                output.push(line);
                let len = output.len();
                if len > 100 {
                    output.drain(0..(len - 100));
                }
            }
        }

        if self.show_settings {
            egui::Window::new("Settings")
                .resizable(true)
                .default_size(egui::vec2(400.0, 300.0))
                .show(ctx, |ui| {
                    ui.label("Appearance");
                    ui.color_edit_button_srgba(&mut self.bg_color);
                    ui.separator();
                    ui.label("Note: This is a demo settings window.");
                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
        }

        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "Linux Style Shell",
        options,
        Box::new(|_cc| Box::new(LinuxStyleShellApp::default())),
    );
}
