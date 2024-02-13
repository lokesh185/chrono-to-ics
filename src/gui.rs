#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use chrono_to_ics::api::client::ApiClient;
use chrono_to_ics::ics;
use eframe::egui;
use std::fs::File;
use std::io::prelude::*;
pub fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([250.0, 125.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Chrono to ics",
        options,
        Box::new(|_cc| Box::<Gui>::default()),
    )
}
#[derive(Default)]
struct Gui {
    api_client: Option<ApiClient>,
    link: String,
    window_info: String,
    window_open: bool,
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.window_open {
            egui::Window::new("Result")
                // .open(&mut self.window_open)
                .show(ctx, |ui| {
                    ui.label(self.window_info.as_str());
                    if ui.button("ok").clicked() {
                        self.window_open = false;
                    }
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Chrono to ics");

                let name_label = ui.label("Your link: ");
                ui.text_edit_singleline(&mut self.link)
                    .labelled_by(name_label.id);

                ui.add_space(10.0);
                if ui.button("run").clicked() {
                    self.window_info = match self.run() {
                        Ok(_) => "success saved to timetable.ics".to_string(),
                        Err(e) => e.to_string(),
                    };
                    self.window_open = true;
                }
            });
        });
    }
}
impl Gui {
    fn run(&mut self) -> Result<(), GuiError> {
        let id = get_id_from_link(&self.link).ok_or(GuiError::InvalidLink)?;
        let api = match self.api_client.as_mut() {
            Some(client) => {
                client.id = id;
                client
            }
            None => {
                self.api_client = Some(match ApiClient::new(id) {
                    Ok(a) => a,
                    Err(_) => {
                        return Err(GuiError::UnableToFetchCourseData);
                    }
                });
                self.api_client.as_mut().unwrap()
            }
        };

        match api.fetch_timetable() {
            Ok(_) => {}
            Err(_) => {
                return Err(GuiError::InvalidLink);
            }
        }
        match api.update_time_table() {
            Some(_) => {}
            None => {
                return Err(GuiError::InvalidTimeTableData);
            }
        }
        match write_to_file(&ics::make_calendar(api.timetable.as_ref().unwrap())) {
            Some(_) => (),
            None => {
                return Err(GuiError::UnableToWriteData);
            }
        };

        Ok(())
    }
}

fn get_id_from_link(link: &str) -> Option<String> {
    Some(link.split('/').last()?.to_string())
}

fn write_to_file(data: &String) -> Option<()> {
    let mut file = File::create("timetable.ics").unwrap();
    file.write_all(data.as_bytes()).unwrap();
    Some(())
}

enum GuiError {
    InvalidLink,
    // UnableToFetchTimetable,
    UnableToFetchCourseData,
    UnableToWriteData,
    InvalidTimeTableData,
}
impl GuiError {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidLink => "your link is invalid".to_string(),
            // Self::UnableToFetchTimetable => "unable to access internet".to_string(),
            Self::UnableToFetchCourseData => "unable to access internet".to_string(),
            Self::UnableToWriteData => "unable to write data".to_string(),
            Self::InvalidTimeTableData => "timetable is invalid".to_string(),
        }
    }
}
