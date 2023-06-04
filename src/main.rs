use eframe::egui;
use egui::{FontId, RichText};
use reqwest;


#[derive(Debug, Clone)]
struct Time {
    now: String,
    Location: String,
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("Clock", options, Box::new(|_cc| Box::<Time>::default()))
        .expect("failed to create window instance");
}

fn get(url: String) -> String {
    let Client = reqwest::blocking::Client::new();
    let response: serde_json::Value = Client.get(url).send().unwrap().json().unwrap();
    let JsonPart = response["datetime"].to_string();
    let mut start = 0;
    let mut end = 12;
    let hours: String = JsonPart
        .chars()
        .take(start)
        .chain(JsonPart.chars().skip(end))
        .collect();
    start = 8;
    end = 22;
    let refined_time: String = hours
        .chars()
        .take(start)
        .chain(hours.chars().skip(end))
        .collect();
    return refined_time;
}

impl Default for Time {
    fn default() -> Self {
        Self {
            now: get("http://worldtimeapi.org/api/timezone/Europe/London".to_string()),
            Location: "London".to_string(),
        }
    }
}
impl eframe::App for Time {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.now = get("http://worldtimeapi.org/api/timezone/Europe/".to_owned()+&self.Location);
            ui.menu_button("Locations", |ui| {
                if ui.button("London").clicked() {
                    self.now =
                        get("http://worldtimeapi.org/api/timezone/Europe/London".to_string());
                    self.Location = String::from("London");
                }
                if ui.button("Paris").clicked() {
                    self.now = get("http://worldtimeapi.org/api/timezone/Europe/Paris".to_string());
                    self.Location = String::from("Paris")
                }
                if ui.button("Berlin").clicked() {
                    self.now = get("http://worldtimeapi.org/api/timezone/Europe/Berlin".to_string());
                    self.Location = String::from("Berlin")
                }
                if ui.button("Warsaw").clicked() {
                    self.now = get("http://worldtimeapi.org/api/timezone/Europe/Warsaw".to_string());
                    self.Location = String::from("Warsaw")
                }
                if ui.button("exit").clicked() {
                    ui.close_menu();
                }
            });
            ui.heading(RichText::new(&self.Location).font(FontId::proportional(60.0)));
            ui.label(RichText::new(&self.now).font(FontId::proportional(40.0)));
        });
    }
}
