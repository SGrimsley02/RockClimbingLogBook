#![allow(unused_imports, dead_code, unreachable_patterns)]
use std::io;
use std::fmt;

mod routes_db;
use eframe::egui::FontDefinitions;
use eframe::egui::ScrollArea;
use eframe::NativeOptions;
use routes_db::RoutesDb;
use futures::executor::block_on;
mod climbing;
use climbing::*;


use eframe::{run_native, egui::CentralPanel, epi::App};
use futures::future::FutureExt;
use sea_orm::DatabaseConnection;


fn main() {
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
    let app = MyApp;
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}


struct MyApp;
impl MyApp {
    fn new() -> Self {
        Self
    }
    /*
    fn configure_fonts(&self, context) {
        let mut font_def = FontDefinitions::default();
    } 
    */

    fn render_options(&self, ui: &mut eframe::egui::Ui) {
        ui.heading("Climbing Log");
        ui.colored_label(eframe::egui::Color32::RED, "Welcome to the climbing log!");
        ui.label("Please select an option:");
        
        ui.add_space(10.0);
        ui.add(eframe::egui::Label::new("1. Add Grade").text_style(eframe::egui::TextStyle::Button));
        ui.label("2. Remove Grade");
        ui.label("3. Add Route");
        ui.label("4. Remove Route");
        ui.label("5. Find Route");
        ui.label("6. Find All Routes");
        ui.label("7. Find Routes at Grade");
        ui.label("8. Log Session");
        ui.label("9. History");
        ui.label("10. Stats");
        ui.label("11. Exit");
    }
}

impl App for MyApp {
    fn setup(&mut self, _context: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>, _storage: Option<&dyn eframe::epi::Storage>) {
        //self.configure_fonts(context);
        
    }

    
    #[allow(unused_variables)]
    fn update(&mut self, context: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(context, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_options(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "MyApp"
    }
}