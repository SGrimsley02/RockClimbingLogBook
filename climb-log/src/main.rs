#![allow(unused_imports, dead_code, unreachable_patterns)]
use std::io;
use std::fmt;
use std::sync::Arc;
use itertools::Itertools;

mod routes_db;
use eframe::egui::FontDefinitions;
use eframe::egui::ScrollArea;
use eframe::NativeOptions;
use routes_db::RoutesDb;
use futures::executor::block_on;
mod climbing;
use climbing::*;


use eframe::{run_native, egui::CentralPanel, epi::App};
use eframe::egui;
use futures::future::FutureExt;
use sea_orm::DatabaseConnection;
use sea_orm::Database;
use tokio::runtime::Runtime;


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    println!("\nClimbing Structures:");
    println!("{:?}", Yosemite::FiveTenA);
    let route = Route {
        name: String::from("The Nose"),
        grade: Grade::Yosemite(Yosemite::FiveNine),
        style: vec![Style::Aid, Style::Trad],
        length: 3000,
        pitches: 31,
        location: String::from("Yosemite Valley"),
    };
    let send = Send {
        route: route,
        date: Date {
            year: 2021,
            month: 6,
            day: 1,
        },
        partner: String::from("Alex Honnold"),
        completed: true,
        attempts: 1,
        send_type: SendType::FreeSolo,
        notes: String::from("Free solo"),
    };
    println!("{}", send);

    let v_grade = Hueco::V13;
    let y_grade: Yosemite = v_grade.into();
    println!("{} is Yosemite {}", v_grade, y_grade);

    println!("\nRoutes Database:");
    if let Err(err) = block_on(RoutesDb::run_db()) {
        panic!("{}", err);
    } else {
        println!("Success!");
    }


    println!("\nEGUI User Interface:");
    
    // Tutorial: https://www.youtube.com/watch?v=NtUkr_z7l84
    let rt = Arc::new(Runtime::new().unwrap());
    let app = MyApp {
        page: Page::Home,
        route_options: RouteOptions::default(),
        database: Arc::new(RoutesDb::new().await.expect("Failed to connect to database")),
        rt: Arc::clone(&rt),
    };
    let win_option = NativeOptions::default();
    run_native(
        Box::new(app),
        win_option,
    );
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Page {
    Home,
    AddGrade,
    RemoveGrade,
    AddRoute,
    RemoveRoute,
    FindRoute,
    LogSession,
    History,
    Stats,
    Exit,
}

#[derive(Default)]
struct RouteOptions {
    name: String,
    grade: climbing::Yosemite, //Defaulting to this for now
    boulder: bool,
    sport: bool,
    trad: bool,
    aid: bool,
    ice: bool,
    alpine: bool,
    top_rope: bool,
    free_solo: bool,
    deep_water: bool,
    speed: bool,
    length_buffer: String,
    length: u16,
    pitches: u8,
    location: String,
}

struct MyApp {
    page: Page,
    route_options: RouteOptions,
    database: Arc<RoutesDb>,
    rt: Arc<Runtime>,
}

impl MyApp {

    fn render_home(&mut self, ui: &mut eframe::egui::Ui) {
        ScrollArea::auto_sized().show(ui, |ui| {
            ui.heading("Climbing Log");
            ui.colored_label(eframe::egui::Color32::RED, "Welcome to the climbing log!");
            ui.label("Please select an option:");
            
            ui.horizontal(|ui| {
                if ui.button("Add Grade").clicked() {
                    self.page = Page::AddGrade;
                }
                else if ui.button("Remove Grade").clicked() {
                    self.page = Page::RemoveGrade;
                }
                else if ui.button("Add Route").clicked() {
                    self.page = Page::AddRoute;
                }
                else if ui.button("Remove Route").clicked() {
                    self.page = Page::RemoveRoute;
                }
                else if ui.button("Find Route").clicked() {
                    self.page = Page::FindRoute;
                }
                else if ui.button("Log Session").clicked() {
                    self.page = Page::LogSession;
                }
                else if ui.button("History").clicked() {
                    self.page = Page::History;
                }
                else if ui.button("Stats").clicked() {
                    self.page = Page::Stats;
                }
                else if ui.button("Exit").clicked() {
                    self.page = Page::Exit;
                }
            });
        });
    }

    fn render_add_grade(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Add Grade");
    }

    fn render_remove_grade(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Remove Grade");
    }

    fn render_add_route(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Add Route");
        ScrollArea::auto_sized().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.route_options.name);
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Style: ");
                ui.checkbox(&mut self.route_options.boulder, "Boulder");
                ui.checkbox(&mut self.route_options.sport, "Sport");
                ui.checkbox(&mut self.route_options.trad, "Trad");
                ui.checkbox(&mut self.route_options.aid, "Aid");
                ui.checkbox(&mut self.route_options.ice, "Ice");
                ui.checkbox(&mut self.route_options.alpine, "Alpine");
                ui.checkbox(&mut self.route_options.top_rope, "Top Rope");
                ui.checkbox(&mut self.route_options.free_solo, "Free Solo");
                ui.checkbox(&mut self.route_options.deep_water, "Deep Water");
                ui.checkbox(&mut self.route_options.speed, "Speed");
            });

            ui.separator();

            egui::ComboBox::from_label("Grade")
                .selected_text(format!("{}", self.route_options.grade))
                .show_ui(ui, |ui| {
                    Yosemite::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.route_options.grade, grade, format!("{}", grade));
                    });
                });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Length (ft):");
                ui.text_edit_singleline(&mut self.route_options.length_buffer);
                if let Ok(length) = self.route_options.length_buffer.parse::<u16>() {
                    self.route_options.length = length;
                } else {
                    ui.label("Invalid length, please enter a number.");
                }
            });
            
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Pitches:");
                ui.add(eframe::egui::widgets::DragValue::new(&mut self.route_options.pitches).speed(1.0));
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Location:");
                ui.text_edit_singleline(&mut self.route_options.location);
            });

            ui.separator();

            if ui.button("Save").clicked() {
                #[allow(unused_parens)] //parenthesis are used for readability
                if (
                    self.route_options.boulder ||
                    self.route_options.sport ||
                    self.route_options.trad ||
                    self.route_options.aid ||
                    self.route_options.ice ||
                    self.route_options.alpine ||
                    self.route_options.top_rope ||
                    self.route_options.free_solo ||
                    self.route_options.deep_water ||
                    self.route_options.speed
                    &&
                    self.route_options.length > 0
                    &&
                    self.route_options.pitches > 0
                    &&
                    !self.route_options.location.is_empty()
                ) {
                    // Make the style vector
                    let mut style = Vec::new();
                    if self.route_options.boulder {
                        style.push(Style::Boulder);
                    }
                    if self.route_options.sport {
                        style.push(Style::Sport);
                    }
                    if self.route_options.trad {
                        style.push(Style::Trad);
                    }
                    if self.route_options.aid {
                        style.push(Style::Aid);
                    }
                    if self.route_options.ice {
                        style.push(Style::Ice);
                    }
                    if self.route_options.alpine {
                        style.push(Style::Alpine);
                    }
                    if self.route_options.top_rope {
                        style.push(Style::TopRope);
                    }
                    if self.route_options.free_solo {
                        style.push(Style::FreeSolo);
                    }
                    if self.route_options.deep_water {
                        style.push(Style::DeepWater);
                    }
                    if self.route_options.speed {
                        style.push(Style::Speed);
                    }

                    let style_str = style.iter().map(|s| s.to_string()).join(", ");
                    let name = self.route_options.name.clone();
                    let length = self.route_options.length.into();
                    let pitches = self.route_options.pitches.into();
                    let location = self.route_options.location.clone();
                    let grade = self.route_options.grade.clone();

                    // Add the route to the database
                    let db = Arc::clone(&self.database);
                    let rt = Arc::clone(&self.rt);

                    rt.spawn(async move {
                        <RoutesDb as Clone>::clone(&db).add_route(name, length, pitches, style_str, 10).await.expect("Error, could not add route."); //grade_id is hardcoded for now
                    });
                    
                    
                } else {
                    ui.label("Please select at least one style.");
                }
                
            }



        })
    }

    fn render_remove_route(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Remove Route");
    }

    fn render_find_route(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Find Route");
    }

    fn render_log_session(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Log Session");
    }

    fn render_history(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("History");
    }

    fn render_stats(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Stats");
    }

    fn render_exit(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Exit");
    }

}


impl App for MyApp {
    fn setup(&mut self, _context: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>, _storage: Option<&dyn eframe::epi::Storage>) {
        //self.configure_fonts(context);
        
    }

    
    #[allow(unused_variables)]
    fn update(&mut self, context: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(context, |ui| {
            match self.page {
                Page::Home => self.render_home(ui),
                Page::AddGrade => self.render_add_grade(ui),
                Page::RemoveGrade => self.render_remove_grade(ui),
                Page::AddRoute => self.render_add_route(ui),
                Page::RemoveRoute => self.render_remove_route(ui),
                Page::FindRoute => self.render_find_route(ui),
                Page::LogSession => self.render_log_session(ui),
                Page::History => self.render_history(ui),
                Page::Stats => self.render_stats(ui),
                Page::Exit => self.render_exit(ui),
            }

        });
    }

    fn name(&self) -> &str {
        "MyApp"
    }
}