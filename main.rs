#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod data;
mod converse;
mod python;

use std::io::Write;
use eframe::egui;
use crate::converse::Conversations;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0 / 4.0, 1080.0 / 3.0]),
        ..Default::default()
    };
    eframe::run_native(
        "YonI",
        options,
        Box::new(|cc| {
            // Image support if visual processing is implemented:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

pub struct MyApp {
    name: String,
    input: String,
    response: String,
    default_response: bool,
    conversations: Conversations
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "user".to_owned(),
            input: "".to_string(),
            response: "How may I assist you today?".to_string(),
            default_response: true,
            conversations: Conversations::new(),
        }
    }
}

fn set_name(new_name: &str) -> String {
    new_name.to_string()
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
            egui::widgets::global_dark_light_mode_buttons(ui);
            ui.heading("YonI Chat");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
                if self.conversations.conversations.is_empty() && self.name != "user" {
                    self.response = format!("Hello {}, How may I assist you today?", self.name).to_string();
                }
            });

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input);
                if ui.button("Send").clicked() {
                    let conversation: bool = Conversations::send_input(&mut self.conversations, &mut self.input);
                    println!("{}", Conversations::get_last_output(&mut self.conversations));
                    if conversation {
                        self.response = Conversations::get_last_output(&mut self.conversations);
                    }
                }
            });
            ui.text_edit_multiline(&mut self.response);
        });

    }
}
