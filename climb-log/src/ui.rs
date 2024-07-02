use std::sync::{Arc, Mutex, MutexGuard};
use tokio::runtime::Runtime;
use itertools::Itertools;
use eframe::{egui::{self, CentralPanel, ScrollArea}, App, run_native, NativeOptions};
mod routes_db;
use routes_db::{RoutesDb, entities::{routes::Model as RouteModel, sends::Model as SendModel, grades::Model as GradeModel}};
mod climbing;
use climbing::{Font, French, FullGrade, Hueco, SendType, Style, Uiaa, Yosemite};






#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Page { // All the possible pages for the app to display
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
    RemoveSession,
    ViewSession,
    History,
    Stats,
    Exit,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Clone)]
struct RouteOptions { // All the info needed to add a route
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
    length: i32,
    pitches: i32,
    location: String,
    //notes: String,
}

#[derive(Default, Clone)]
struct SendOptions { // All the info needed to log a send
    date: sea_orm::prelude::Date, //EGUI works really well with sea_orm's Date type so just using that
    partner: String, // Partner's name, optional (but does not need to have an Option)
    send_type: SendType, // Type of send
    attempts: i32, // Number of attempts, i32 bc that's what sea_orm/sqlite uses
    notes: String, // Any notes
    route_name: String, // Name of route, should match with a route in database
    route: Option<RouteModel>, // Route from database, fetched for the user
}

pub struct MyApp { // The main app struct
    page: Page, // Current page to display
    route_options: RouteOptions, // Options for adding a route
    removal_name: String, // Name of route to remove
    find_name: String, // Name of route to find
    all_routes: Arc<Mutex<Vec<(RouteModel, GradeModel)>>>, // All routes in the database, in an async context
    database: Arc<RoutesDb>, // Database through RoutesDb
    rt: Arc<Option<Runtime>>, // Runtime for async stuff
    should_quit: bool, // Quit flag
    search_result: Arc<Mutex<Option<(RouteModel, GradeModel)>>>, // Result of a single search, async context
    viewing: Option<(RouteModel, GradeModel)>, // Route to view in more detail, out of the async
    send_options: SendOptions, // Options for logging a send
    session: Vec<SendOptions>, // All sends in a session
    session_id: i32, // Session id to search for
    cur_session: Arc<Mutex<Vec<SendModel>>>, // Current session to view, async context
    view_session: Option<SendModel>, // Session to view in more detail, out of the async
    add_grade: FullGrade, // Grade to add, with options for all types
    remove_grade: FullGrade, // Grade to remove, with options for all types
}

impl MyApp {

    pub async fn new(rt: &Arc<Option<Runtime>>) -> Self { // Create a new app
        let mut app = MyApp { // Initialize all fields
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
        };
        app.session.push(SendOptions::default());
        app
    }

    pub async fn run(){ // Run the app, can call using MyApp::run().await; and will create from scratch
        let rt = Arc::new(Some(Runtime::new().unwrap())); // Set up async runtime to be able to communicate w/ db
        let app = MyApp::new(&rt).await;
        let win_option = NativeOptions::default(); //Using default options for now
        // Run
        let _ = run_native(
            "Ascent Climbing Log",
            win_option,
            Box::new(|_cc| Box::new(app)),
        );
        // Shutdown the runtime when quitting
        if let Some(runtime) = Arc::try_unwrap(rt).ok().and_then(|opt| opt) {
            runtime.shutdown_background();
        }
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_home(&mut self, ctx: &eframe::egui::Context) { // Render functions for each page
        egui::TopBottomPanel::top("Home Header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                
                //Display an image from the file AscentLogo.png
                let logo = egui::include_image!("../assets/AscentLogo.png");
                ui.image(logo);

                ui.heading("Ascent Climbing Log");
            });
        });

        // Menu on the left
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
                if ui.button("Remove Session").clicked() {
                    self.page = Page::RemoveSession;
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

        // Home page content in central area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Home Page Content on right/central area
            ui.vertical(|ui| {
                ui.heading("Welcome to Ascent!");
                ui.separator();
                ui.add_space(10.0);
                //Text
                ui.label("Ascent is a climbing log application designed to help you keep track of your climbing sessions. You can add and remove grades, routes, and log your climbing sessions. You can also search for routes and view your climbing history. To get started, use the menu on the left to navigate to the desired page.");
            })
        });
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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
                        ui.selectable_value(&mut self.add_grade.yosemite, grade, format!("{grade}"));
                    });
                });
            
            ui.separator();
            
            egui::ComboBox::from_label("French Grade")
                .selected_text(format!("{}", self.add_grade.french))
                .show_ui(ui, |ui| {
                    French::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.french, grade, format!("{grade}"));
                    });
                });
            
            ui.separator();
            egui::ComboBox::from_label("UIAA Grade")
                .selected_text(format!("{}", self.add_grade.uiaa))
                .show_ui(ui, |ui| {
                    Uiaa::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.uiaa, grade, format!("{grade}"));
                    });
                });
            
            ui.separator();
            if ui.button("Save Tall Wall Grade").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let grade = self.add_grade;
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
                        ui.selectable_value(&mut self.add_grade.hueco, grade, format!("{grade}"));
                    });
                });
            
            ui.separator();
            

            egui::ComboBox::from_label("Font Grade")
                .selected_text(format!("{}", self.add_grade.font))
                .show_ui(ui, |ui| {
                    Font::iter().for_each(|grade| {
                        ui.selectable_value(&mut self.add_grade.font, grade, format!("{grade}"));
                    });
                });
            
            ui.separator();

            if ui.button("Save Boulder Grade:").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let grade = self.add_grade;
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    <RoutesDb as Clone>::clone(&db).add_grade(None, Some(grade.font.to_string()), None, Some(grade.hueco.to_string()), None).await.expect("Error, could not add grade.");
                });
                self.reset();
            }
        });
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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
                        ui.selectable_value(&mut self.remove_grade.yosemite, grade, format!("{grade}"));
                    });
                });
            
            ui.separator();

            if ui.button("Remove Tall Wall Grade").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let grade = self.remove_grade.yosemite;
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
                        ui.selectable_value(&mut self.remove_grade.hueco, grade, format!("{grade}"));
                    });
                });
            
            ui.separator();

            if ui.button("Remove Boulder Grade").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let grade = self.remove_grade.hueco;
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    let grade_id = <RoutesDb as Clone>::clone(&db).get_grade_id(&grade.to_string()).await.expect("Error, could not get grade id.");
                    <RoutesDb as Clone>::clone(&db).remove_grade(grade_id).await.expect("Error, could not remove grade.");
                });
                self.reset();
            }
        });
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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
                .selected_text({ if self.route_options.boulder { self.route_options.grade.hueco.to_string() } else { self.route_options.grade.yosemite.to_string() } }.to_string())
                .show_ui(ui, |ui| {
                    if self.route_options.boulder {
                        Hueco::iter().for_each(|grade| {
                            ui.selectable_value(&mut self.route_options.grade.hueco, grade, format!("{grade}"));
                        });
                    } else {
                        Yosemite::iter().for_each(|grade| {
                            ui.selectable_value(&mut self.route_options.grade.yosemite, grade, format!("{grade}"));
                        });
                    }
                });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Length (ft):");
                ui.text_edit_singleline(&mut self.route_options.length_buffer);
                if let Ok(length) = self.route_options.length_buffer.parse::<u16>() { //Use u16 to prevent negative numbers
                    self.route_options.length = i32::from(length); //Convert to i32 for database
                } else {
                    ui.label("Invalid length, please enter a number.");
                }
            });
            
            ui.separator();
            if self.route_options.boulder {
                self.route_options.pitches = 0;
            } else {
                ui.horizontal(|ui| {
                    ui.label("Pitches:");
                    ui.add(eframe::egui::widgets::DragValue::new(&mut self.route_options.pitches).speed(0.5));
                });
                ui.separator();
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
                    let style_str = style.iter().map(std::string::ToString::to_string).join(", ");
                    let name = self.route_options.name.clone();
                    let length = self.route_options.length;
                    let pitches = self.route_options.pitches;
                    #[allow(unused_variables)] //Location doesn't currently exist in the database
                    let location = self.route_options.location.clone();
                    #[allow(unused_variables)] //Grade is hardcoded for now
                    let grade = self.route_options.grade;
                    let str_grade: String = { if self.route_options.boulder { grade.hueco.to_string() } else { grade.yosemite.to_string() } }.to_string();
                    
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
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
            let mut to_remove = None;
            for index in 0..self.session.len() {
                let send = &mut self.session[index];

                ui.group(|ui| {
                    ui.add(egui_extras::DatePickerButton::new(&mut send.date));

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Partner:");
                        ui.text_edit_singleline(&mut send.partner);
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Send Type: ");
                        egui::ComboBox::from_id_source(index)
                            .selected_text(format!("{}", send.send_type))
                            .show_ui(ui, |ui| {
                                SendType::iter().for_each(|send_type| {
                                    ui.selectable_value(&mut send.send_type, send_type, format!("{send_type}"));
                                });
                            });
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Attempts:");
                        ui.add(eframe::egui::widgets::DragValue::new(&mut send.attempts).speed(1.0));
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Notes:");
                        ui.text_edit_multiline(&mut send.notes);
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Route:");
                        ui.text_edit_singleline(&mut send.route_name);
                    });

                    ui.separator();
                
                    if ui.button("Remove").clicked() {
                        to_remove = Some(index);
                    }
                });
                ui.separator();
            }
            //TODO: Make route search more of a search instead of just typing in the name

            if let Some(index) = to_remove {
                self.session.remove(index);
            }

            if ui.button("Add Send").clicked() {
                //Add a new send to the form
                self.session.push(SendOptions::default());
            }

            if ui.button("Log Session").clicked() {
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let sends = self.session.clone();
                
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    
                    
                    let session_id = <RoutesDb as Clone>::clone(&db).get_next_session_id().await.expect("Error, could not get next session id.");
                    for send in &sends {
                        let find_name = send.route_name.clone();
                        let partner = if send.partner.is_empty() { None } else { Some(send.partner.clone()) };
                        let notes = if send.notes.is_empty() { None } else { Some(send.notes.clone()) };
                        let route = <RoutesDb as Clone>::clone(&db).find_route_name(&find_name).await.expect("Error, could not find route.");
                        let route = route.unwrap();
                        <RoutesDb as Clone>::clone(&db).add_send(session_id, route, send.date.to_string(), partner, send.send_type.to_string(), send.attempts, notes).await.expect("Error, could not log session.");
                    }
                    
                });
                self.reset();
            }
        });
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_delete_session(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Delete Session");
        
        ScrollArea::vertical().show(ui, |ui| {
            // Still no good search so just doing this for now
            ui.label("Please enter the session id to delete:");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Session Id:");
                ui.add(eframe::egui::widgets::DragValue::new(&mut self.session_id).speed(1.0));
            });
            ui.separator();
            if ui.button("Delete").clicked() {
                // Async delete session
                let session_id = self.session_id;
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    <RoutesDb as Clone>::clone(&db).remove_session(session_id).await.expect("Error, could not remove session.");
                });
                self.reset();
            }
        });
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_history(&mut self, ui: &mut eframe::egui::Ui) {
        // Display sessions in a scroll area
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("History");

        // Get all sessions using the database
        // Display them all in a scroll area, backwards by session id (ie, most recent first)
        // Each session should have a button to view it in more detail

        ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| { //Still don't have a good search so just doing this for now
                ui.label("Search: ");
                ui.add(eframe::egui::widgets::DragValue::new(&mut self.session_id).speed(1.0));
            });

            ui.separator();
            // Get all sessions thru async
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
            // Display all sessions
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_view_session(&mut self, ui: &mut eframe::egui::Ui) {
        // Display info for a single session
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("View Session");
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_stats(&mut self, ui: &mut eframe::egui::Ui) {
        // Display a variety of stats for the user
        if ui.button("Back").clicked() {
            self.reset();
        }
        ui.heading("Stats");
    }

    fn render_exit(&mut self, ui: &mut eframe::egui::Ui) {
        // Quit confirmation page
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
        // Reset all meaningful fields to default
        self.page = Page::Home;
        self.route_options = RouteOptions::default();
        self.removal_name = String::new();
        self.find_name = String::new();
        self.all_routes = Arc::new(Mutex::new(Vec::new()));
        self.search_result = Arc::new(Mutex::new(None));
        self.viewing = None;
        self.send_options = SendOptions::default();
        self.session = Vec::new();
        self.session.push(SendOptions::default());
        self.cur_session = Arc::new(Mutex::new(Vec::new()));
        self.view_session = None;
        self.add_grade = FullGrade::default();
        self.remove_grade = FullGrade::default();
        self.session_id = 0;
    }
}

impl App for MyApp {
    
    #[allow(unused_variables)] //frame is needed for update but not being used for anything
    fn update(&mut self, context: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // Control function to move between pages. Also adds the image loaders
        egui_extras::install_image_loaders(context);
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
                Page::RemoveSession => self.render_delete_session(ui),
                Page::ViewSession => self.render_view_session(ui),
                Page::History => self.render_history(ui),
                Page::Stats => self.render_stats(ui),
                Page::Exit => self.render_exit(ui),
            }
        });
        // Safe quit
        if self.should_quit {
            let ctx = context.clone();
            std::thread::spawn(move || {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            });
        }
    }

}