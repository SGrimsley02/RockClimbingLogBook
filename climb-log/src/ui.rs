use std::sync::{Arc, Mutex, MutexGuard};
use tokio::runtime::Runtime;
use itertools::Itertools;
use eframe::{egui::{self, CentralPanel, ScrollArea}, App, run_native, NativeOptions};
mod routes_db;
use routes_db::{RoutesDb, entities::{routes::Model as RouteModel, sends::Model as SendModel, grades::Model as GradeModel}};
mod climbing;
use climbing::*;






#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Page {
    Home,
    AddGrade,
    RemoveGrade,
    AddRoute,
    RemoveRoute,
    SearchHome,
    FindRoute,
    ViewRoute,
    ViewAllRoutes,
    LogSession,
    ViewSession,
    History,
    Stats,
    Exit,
}

#[derive(Default, Clone)]
struct RouteOptions {
    name: String,
    grade: FullGrade,
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

#[derive(Default, Clone)]
struct SendOptions {
    date: Date,
    partner: String,
    send_type: SendType,
    attempts: u8,
    notes: String,
    route_name: String,
    route: Option<RouteModel>,
}

pub struct MyApp {
    page: Page,
    route_options: RouteOptions,
    removal_name: String,
    find_name: String,
    all_routes: Arc<Mutex<Vec<(RouteModel, GradeModel)>>>,
    database: Arc<RoutesDb>,
    rt: Arc<Option<Runtime>>,
    should_quit: bool,
    search_result: Arc<Mutex<Option<(RouteModel, GradeModel)>>>,
    viewing: Option<(RouteModel, GradeModel)>,
    send_options: SendOptions,
    session: Vec<SendOptions>,
    session_id: i32,
    cur_session: Arc<Mutex<Vec<SendModel>>>,
    view_session: Option<SendModel>,
    add_grade: FullGrade,
    remove_grade: FullGrade,
    //#[allow(deprecated)]
    //logo: RetainedImage,

}

impl MyApp {

    pub async fn new(rt: &Arc<Option<Runtime>>) -> Self {
        MyApp {
            page: Page::Home,
            route_options: RouteOptions::default(),
            removal_name: String::new(),
            find_name: String::new(),
            all_routes: Arc::new(Mutex::new(Vec::new())),
            database: Arc::new(RoutesDb::new().await.expect("Failed to connect")),
            rt: Arc::clone(rt),
            should_quit: false,
            search_result: Arc::new(Mutex::new(None)),
            viewing: None,
            send_options: SendOptions::default(),
            session: Vec::new(),
            session_id: 0,
            cur_session: Arc::new(Mutex::new(Vec::new())),
            view_session: None,
            add_grade: FullGrade::default(),
            remove_grade: FullGrade::default(),
            //#[allow(deprecated)]
            //logo: RetainedImage::from_image_bytes("AscentLogo.png", include_bytes!("AscentLogo.png")).unwrap(),
        }
    }

    pub async fn run(){
        let rt = Arc::new(Some(Runtime::new().unwrap())); // Set up async runtime to be able to communicate w/ db
        let app = MyApp::new(&rt).await;
        let win_option = NativeOptions::default(); //Using default options for now
        // Run
        let _ = run_native(
            "Ascent Climbing Log",
            win_option,
            Box::new(|_cc| Box::new(app)),
        );

        if let Some(runtime) = Arc::try_unwrap(rt).ok().and_then(|opt| opt) {
            runtime.shutdown_background();
        }
    }

    fn render_home(&mut self, ctx: &eframe::egui::Context) {
        egui::TopBottomPanel::top("Home Header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                
                //Display an image from the file AscentLogo.png


                
                

                
                ui.heading("Ascent");
            });
        });
        egui::SidePanel::left("Menu").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Menu");
                ui.add_space(15.0);
                // All the buttons to go to other pages
                if ui.button("Add Grade").clicked() {
                    self.page = Page::AddGrade;
                }
                ui.add_space(10.0);
                if ui.button("Remove Grade").clicked() {
                    self.page = Page::RemoveGrade;
                }
                ui.add_space(10.0);
                if ui.button("Add Route").clicked() {
                    self.page = Page::AddRoute;
                }
                ui.add_space(10.0);
                if ui.button("Remove Route").clicked() {
                    self.page = Page::RemoveRoute;
                }
                ui.add_space(10.0);
                if ui.button("Search").clicked() {
                    self.page = Page::SearchHome;
                } 
                ui.add_space(10.0);
                if ui.button("Log Session").clicked() {
                    self.page = Page::LogSession;
                }
                ui.add_space(10.0);
                if ui.button("History").clicked() {
                    self.page = Page::History;
                }
                ui.add_space(10.0);
                if ui.button("Stats").clicked() {
                    self.page = Page::Stats;
                }
                ui.add_space(10.0);
                if ui.button("Exit").clicked() {
                    self.page = Page::Exit;
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // Home Page Content on right/central area
            ui.vertical(|ui| {
                ui.heading("Welcome to Ascent!");
                ui.add_space(10.0);
                //Text
            })
        });
    }

    fn render_add_grade(&mut self, ui: &mut eframe::egui::Ui) { //Should not be in end product
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Add Grade");
        ScrollArea::vertical().show(ui, |ui| {
            ui.label("Tall Wall Grade:");
            egui::ComboBox::from_label("Yosemite Grade")
                .selected_text(format!("{}", self.add_grade.yosemite))
                .show_ui(ui, |ui| {
                    Yosemite::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.yosemite, grade, format!("{}", grade));
                    });
                });
            
            ui.separator();
            
            egui::ComboBox::from_label("French Grade")
                .selected_text(format!("{}", self.add_grade.french))
                .show_ui(ui, |ui| {
                    French::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.french, grade, format!("{}", grade));
                    });
                });
            
            ui.separator();
            egui::ComboBox::from_label("UIAA Grade")
                .selected_text(format!("{}", self.add_grade.uiaa))
                .show_ui(ui, |ui| {
                    Uiaa::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.uiaa, grade, format!("{}", grade));
                    });
                });
            
            ui.separator();
            if ui.button("Save Tall Wall Grade").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let grade = self.add_grade.clone();
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    <RoutesDb as Clone>::clone(&db).add_grade(Some(grade.yosemite.to_string()), None, Some(grade.french.to_string()), None, Some(grade.uiaa.to_string())).await.expect("Error, could not add grade.");
                });
                self.reset();
            }

            ui.separator();

            ui.label("Bouldering Grade:");

            egui::ComboBox::from_label("Hueco Grade")
                .selected_text(format!("{}", self.add_grade.hueco))
                .show_ui(ui, |ui| {
                    Hueco::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.hueco, grade, format!("{}", grade));
                    });
                });
            
            ui.separator();
            

            egui::ComboBox::from_label("Font Grade")
                .selected_text(format!("{}", self.add_grade.font))
                .show_ui(ui, |ui| {
                    Font::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.font, grade, format!("{}", grade));
                    });
                });
            
            ui.separator();

            if ui.button("Save Boulder Grade:").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let grade = self.add_grade.clone();
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    <RoutesDb as Clone>::clone(&db).add_grade(None, Some(grade.font.to_string()), None, Some(grade.hueco.to_string()), None).await.expect("Error, could not add grade.");
                });
                self.reset();
            }
        });
    }

    fn render_remove_grade(&mut self, ui: &mut eframe::egui::Ui) { //Should not be in end product
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Remove Grade");

        ScrollArea::vertical().show(ui, |ui| {
            ui.label("Please select a grade to remove:");
            ui.separator();
            
            egui::ComboBox::from_label("Yosemite Grade: ")
                .selected_text(format!("{}", self.remove_grade.yosemite))
                .show_ui(ui, |ui| {
                    Yosemite::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.remove_grade.yosemite, grade, format!("{}", grade));
                    });
                });
            
            ui.separator();

            if ui.button("Remove").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let grade = self.remove_grade.yosemite.clone();
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    let grade_id = <RoutesDb as Clone>::clone(&db).get_grade_id(&grade.to_string()).await.expect("Error, could not get grade id.");
                    <RoutesDb as Clone>::clone(&db).remove_grade(grade_id).await.expect("Error, could not remove grade.");
                });
                self.reset();
            }

            ui.separator();

            egui::ComboBox::from_label("Hueco Grade: ")
                .selected_text(format!("{}", self.remove_grade.hueco))
                .show_ui(ui, |ui| {
                    Hueco::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.remove_grade.hueco, grade, format!("{}", grade));
                    });
                });
        });
    }

    fn render_add_route(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Add Route");
        ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.route_options.name);
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Style: ");
                //Boulder or Tall Wall
                ui.radio_value(&mut self.route_options.boulder, true, "Boulder");
                ui.radio_value(&mut self.route_options.boulder, false, "Tall Wall");
            });
            ui.horizontal(|ui| { //Only show these options if it's a tall wall route
                if !self.route_options.boulder {
                    ui.checkbox(&mut self.route_options.sport, "Sport");
                    ui.checkbox(&mut self.route_options.trad, "Trad");
                    ui.checkbox(&mut self.route_options.aid, "Aid");
                    ui.checkbox(&mut self.route_options.ice, "Ice");
                    ui.checkbox(&mut self.route_options.alpine, "Alpine");
                    ui.checkbox(&mut self.route_options.top_rope, "Top Rope");
                    ui.checkbox(&mut self.route_options.free_solo, "Free Solo");
                    ui.checkbox(&mut self.route_options.deep_water, "Deep Water");
                    ui.checkbox(&mut self.route_options.speed, "Speed");
                }
            });

            ui.separator();

            egui::ComboBox::from_label("Grade")
                .selected_text(format!("{}", { if self.route_options.boulder { self.route_options.grade.hueco.to_string() } else { self.route_options.grade.yosemite.to_string() } }))
                .show_ui(ui, |ui| {
                    if self.route_options.boulder {
                        Hueco::iter().for_each(|grade| {
                            ui.selectable_value(&mut self.route_options.grade.hueco, grade, format!("{}", grade));
                        });
                    } else {
                        Yosemite::iter().for_each(|grade| {
                            ui.selectable_value(&mut self.route_options.grade.yosemite, grade, format!("{}", grade));
                        });
                    }
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
            if !self.route_options.boulder {
            ui.horizontal(|ui| {
                    ui.label("Pitches:");
                    ui.add(eframe::egui::widgets::DragValue::new(&mut self.route_options.pitches).speed(0.5));
                });

                ui.separator();
            } else {
                self.route_options.pitches = 0;
            }

            ui.horizontal(|ui| {
                ui.label("Location:");
                ui.text_edit_singleline(&mut self.route_options.location);
            });

            ui.separator();

            if ui.button("Save").clicked() {
                #[allow(unused_parens)] //parenthesis are used for readability
                if ( //Check if all required fields are filled
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

                    // Clone everything to pass to the async block
                    let style_str = style.iter().map(|s| s.to_string()).join(", ");
                    let name = self.route_options.name.clone();
                    let length = self.route_options.length.into();
                    let pitches = self.route_options.pitches.into();
                    #[allow(unused_variables)] //Location doesn't currently exist in the database
                    let location = self.route_options.location.clone();
                    #[allow(unused_variables)] //Grade is hardcoded for now
                    let grade = self.route_options.grade;
                    let str_grade: String = format!("{}", { if self.route_options.boulder { grade.hueco.to_string() } else { grade.yosemite.to_string() } });
                    
                    // Add the route to the database, starting async stuffe
                    let db = Arc::clone(&self.database);
                    let rt = Arc::clone(&self.rt);
                    
                    rt.as_ref().as_ref().unwrap().spawn(async move { //The two as_refs are actually different: the first is for converting to a shared reference with Arc, the second is for getting an Option to a reference value
                        //<RoutesDb as Clone>::clone(&db)... is used to clone the database connection, preventing a move error from a bad borrow
                        let grade_id: i32 = <RoutesDb as Clone>::clone(&db).get_grade_id(&str_grade).await.expect("Error, could not get grade id.");
                        <RoutesDb as Clone>::clone(&db).add_route(name, length, pitches, style_str, grade_id).await.expect("Error, could not add route."); //grade_id is hardcoded for now
                    });
                    
                    self.reset();
                } else {
                    ui.label("Please select at least one style."); //Currently only flashes, needs fixing
                }
                
            }
        });
    }

    fn render_remove_route(&mut self, ui: &mut eframe::egui::Ui) { //Make sure to check if route exists before removing, not currently doing
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Remove Route");
        
        ScrollArea::vertical().show(ui, |ui| {
            ui.label("Please select a route to remove:");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Route:");
                ui.text_edit_singleline(&mut self.removal_name);
            });
            ui.separator();
            //TODO: Add a dropdown to select from multiple routes with same name when needed
            ui.separator();
            if ui.button("Remove").clicked() {
                let name = self.removal_name.clone();
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    let route = <RoutesDb as Clone>::clone(&db).get_route_id(&name).await.expect("Error, could not get route id.");
                    <RoutesDb as Clone>::clone(&db).remove_route(route).await.expect("Error, could not remove route.");
                });
                self.reset();
            }
        });
    }

    fn render_search_home(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Search");
        ui.horizontal(|ui| {
            if ui.button("Find Route").clicked() {
                self.page = Page::FindRoute;
            }
            else if ui.button("View All Routes").clicked() {
                self.page = Page::ViewAllRoutes;
            }
        });
    }

    fn render_find_route(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        } else {
        
        ui.heading("Find Route");

        ScrollArea::vertical().show(ui, |ui| {
            ui.label("Please enter the name of the route:");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Route:");
                ui.text_edit_singleline(&mut self.removal_name);
            });
            ui.separator();
            if ui.button("Find").clicked() {
                let name = self.removal_name.clone();
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let search_result = Arc::clone(&self.search_result);

                rt.as_ref().as_ref().unwrap().spawn(async move {
                    let (route, grade) = <RoutesDb as Clone>::clone(&db).find_route_and_grade(&name).await.expect("Error, could not find route.");

                    let mut result_lock = search_result.lock().unwrap();
                    *result_lock = Some((route, grade));
                });
            }
            ui.separator();
            if let Some((route, grade)) = &*self.search_result.lock().unwrap() {
                ui.label(format!("Name: {}", route.name));
                ui.label(format!("Grade Id: {}", { if route.pitches == 0 { grade.hueco.clone().unwrap().to_string() } else { grade.yosemite.clone().unwrap().to_string() } }));
                ui.label(format!("Style: {}", route.style));
                ui.label(format!("Length: {} ft", route.length));
                ui.label(format!("Pitches: {}", route.pitches));
                //ui.label(format!("Location: {}", route.location));
            }
        });

        }
    }

    fn render_view_route(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        } else {
        ui.heading("View Route");
        let (view_route, view_grade) = self.viewing.clone().unwrap();
        ui.label(format!("Grade Id: {}", { if view_route.pitches == 0 { view_grade.hueco.unwrap().to_string() } else { view_grade.yosemite.unwrap().to_string() } }));
        ui.label(format!("Style: {}", view_route.style));
        ui.label(format!("Length: {} ft", view_route.length));
        ui.label(format!("Pitches: {}", view_route.pitches));
        //ui.label(format!("Location: {}", route.location));
        //Display notes too once implemented
        }
    }

    fn render_all_routes(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("All Routes");
        
        ScrollArea::vertical().show(ui, |ui| {
            let db = Arc::clone(&self.database);
            let rt = Arc::clone(&self.rt);
            let results = Arc::clone(&self.all_routes);
            rt.as_ref().as_ref().unwrap().spawn(async move {
                let routes = <RoutesDb as Clone>::clone(&db.clone()).find_all_routes_and_grade().await.expect("Error, could not find all routes.");
                let mut routes_guard = results.lock().unwrap();
                *routes_guard = routes;
            });
            
            let routes = self.all_routes.lock().unwrap().clone();
            for (i, (route, grade)) in routes.iter().enumerate() {
                
                ui.horizontal(|ui| {
                    ui.label(format!("Route {}: {}", i, route.name));
                    if ui.button("View").clicked() {
                        self.viewing = Some((route.clone(), grade.clone()));
                        self.page = Page::ViewRoute;
                    }
                });
                ui.label(format!("Grade: {}", if route.pitches == 0 { grade.clone().hueco.unwrap().to_string() } else { grade.clone().yosemite.unwrap().to_string() }));
                
                ui.label(format!("Style: {}", route.style));
                ui.label(format!("Length: {} ft", route.length));
                ui.label(format!("Pitches: {}", route.pitches));
                //ui.label(format!("Location: {}", route.location));
                ui.separator();
            }
        });
    }

    fn render_log_session(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Log Session");

        ScrollArea::vertical().show(ui, |ui| {
            /*ui.horizontal(|ui| {
                ui.label("Indoor or Outdoor?");
                ui.radio_value(&mut self.route_options.indoor, true, "Indoor");
                ui.radio_value(&mut self.route_options.indoor, false, "Outdoor");
            }); */ //Not implemented for indoor yet- outdoor is actually simpler at the moment. Indoor likely needs its own table
            // ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Date (mm/dd/yyyy): ");
                ui.add(egui::DragValue::new(&mut self.send_options.date.month).speed(1.0).clamp_range(1..=12));
                ui.label("/");
                ui.add(egui::DragValue::new(&mut self.send_options.date.day).speed(1.0).clamp_range(1..=31));
                ui.label("/");
                ui.add(egui::DragValue::new(&mut self.send_options.date.year).speed(1.0).clamp_range(2000..=3000));
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Partner:");
                ui.text_edit_singleline(&mut self.send_options.partner);
            });

            ui.separator();

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Send Type")
                    .selected_text(format!("{}", self.send_options.send_type))
                    .show_ui(ui, |ui| {
                        SendType::iter().for_each(|send_type| {
                            ui.selectable_value(&mut self.send_options.send_type, send_type, format!("{}", send_type));
                        });
                    });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Attempts:");
                ui.add(eframe::egui::widgets::DragValue::new(&mut self.send_options.attempts).speed(1.0));
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Notes:");
                ui.text_edit_multiline(&mut self.send_options.notes);
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Route:");
                ui.text_edit_singleline(&mut self.send_options.route_name);
            });

            ui.separator();

            //TODO: Add a dropdown to select from multiple routes with same name when needed
                //Along with this, make it so if a route is not found, it can be added from here
            //TODO: When beautifying, make it easier to add multiple routes at once

            if ui.button("Add Send").clicked() {
                //Add send to the session vector
                self.session.push(self.send_options.clone());
                let date = self.send_options.date;
                self.send_options = SendOptions::default();
                self.send_options.date = date;
            }

            if ui.button("Log Session").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let sends = self.session.clone();
                
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    
                    
                    let session_id = <RoutesDb as Clone>::clone(&db).get_next_session_id().await.expect("Error, could not get next session id.");
                    for send in sends.iter() {
                        let find_name = send.route_name.clone();
                        let partner = if send.partner.is_empty() { None } else { Some(send.partner.clone()) };
                        let notes = if send.notes.is_empty() { None } else { Some(send.notes.clone()) };
                        let route = <RoutesDb as Clone>::clone(&db).find_route_name(&find_name).await.expect("Error, could not find route.");
                        let route = route.unwrap();
                        <RoutesDb as Clone>::clone(&db).add_send(session_id, route, send.date.to_string(), partner, send.send_type.to_string(), send.attempts as i32, notes).await.expect("Error, could not log session.");
                    }
                    
                });
                self.reset();
            }
        });
    }

    fn render_history(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("History");

        // Get all sessions using the database
        // Display them all in a scroll area, backwards by session id (ie, most recent first)
        // Each session should have a button to view it in more detail

        ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.add(eframe::egui::widgets::DragValue::new(&mut self.session_id).speed(1.0));
            });

            ui.separator();

            let db = Arc::clone(&self.database);
            let rt = Arc::clone(&self.rt);
            let results = Arc::clone(&self.cur_session);
            let session_id = self.session_id;
            rt.as_ref().as_ref().unwrap().spawn(async move {
                let sessions = <RoutesDb as Clone>::clone(&db).get_session(session_id).await.expect("Error, could not get all sessions.");
                let mut sessions_guard = results.lock().unwrap();
                *sessions_guard = sessions;
            });

            let sessions: MutexGuard<Vec<SendModel>> = self.cur_session.lock().unwrap();
            for session in sessions.iter() {
                ui.horizontal(|ui| {
                    ui.label(format!("Session {}: ", session.session));
                    if ui.button("View").clicked() {
                        self.view_session = Some(session.clone());
                        self.page = Page::ViewSession;
                    }
                });
                let session = session.clone();
                ui.label(format!("Date: {}", session.date));
                ui.label(format!("Partner: {}", session.partner.unwrap_or("None".to_string())));
                ui.label(format!("Type: {}", session.r#type));
                ui.label(format!("Attempts: {}", session.attempts));
                ui.label(format!("Route: {}", session.route));
                ui.separator();
            }
            
        });
    }

    fn render_view_session(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("View Session");
    }

    fn render_stats(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Stats");
    }

    fn render_exit(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Are you sure you'd like to exit?");
        ui.horizontal(|ui| {
            if ui.button("Yes").clicked() {
                self.should_quit = true;
            }
            else if ui.button("No").clicked() {
                self.reset();
            }
        });
    }

    fn reset(&mut self) {
        self.page = Page::Home;
        self.route_options = RouteOptions::default();
        self.removal_name = String::new();
        self.find_name = String::new();
        self.all_routes = Arc::new(Mutex::new(Vec::new()));
        self.search_result = Arc::new(Mutex::new(None));
        self.viewing = None;
        self.send_options = SendOptions::default();
        self.session = Vec::new();
        self.cur_session = Arc::new(Mutex::new(Vec::new()));
        self.view_session = None;
    }
}

impl App for MyApp {
    

    
    #[allow(unused_variables)]
    fn update(&mut self, context: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(context, |ui| {
            match self.page {
                Page::Home => self.render_home(context),
                Page::AddGrade => self.render_add_grade(ui),
                Page::RemoveGrade => self.render_remove_grade(ui),
                Page::AddRoute => self.render_add_route(ui),
                Page::RemoveRoute => self.render_remove_route(ui),
                Page::SearchHome => self.render_search_home(ui),
                Page::FindRoute => self.render_find_route(ui),
                Page::ViewRoute => self.render_view_route(ui),
                Page::ViewAllRoutes => self.render_all_routes(ui),
                Page::LogSession => self.render_log_session(ui),
                Page::ViewSession => self.render_view_session(ui),
                Page::History => self.render_history(ui),
                Page::Stats => self.render_stats(ui),
                Page::Exit => self.render_exit(ui),
            }

        });
        if self.should_quit {
            let ctx = context.clone();
            std::thread::spawn(move || {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            });
        }
    }

    /*
    fn name(&self) -> &str {
        "Climbing Log"
    }
    */
}