use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard}};
use tokio::runtime::Runtime;
use itertools::Itertools;
use eframe::{egui::{self, CentralPanel, ScrollArea}, App, run_native, NativeOptions};
mod routes_db;
use routes_db::{entities::{grades::Model as GradeModel, routes::Model as RouteModel, sends::Model as SendModel}, RoutesDb};
mod climbing;
use climbing::{Font, French, FullGrade, Hueco, SendType, Style, TallGradeSys, BoulderGradeSys, Uiaa, Yosemite};
use chrono;

struct UserSettings { // App settings
    dark_mode: bool,
    tall_grade_sys: TallGradeSys,
    boulder_grade_sys: BoulderGradeSys,
    user: User,
}
impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            dark_mode: false, // Currently not being used thanks to egui's built-in dark mode
            tall_grade_sys: TallGradeSys::Yosemite,
            boulder_grade_sys: BoulderGradeSys::Hueco,
            user: User::default(),
        }
    }
}

impl UserSettings {
    pub fn render(&mut self, _ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        ui.add_space(20.0);
        ui.heading("Settings");
        ui.horizontal(|ui| {
            ui.label("Dark Mode:");
            egui::global_dark_light_mode_buttons(ui);
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Tall Wall Grade System:");
            egui::ComboBox::from_label("Tall Wall Grade System")
                .selected_text(format!("{}", self.tall_grade_sys))
                .show_ui(ui, |ui| {
                    TallGradeSys::iter().for_each(|grade_sys| {
                        ui.selectable_value(&mut self.tall_grade_sys, grade_sys, format!("{grade_sys}"));
                    });
                });
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Boulder Grade System:");
            egui::ComboBox::from_label("Boulder Grade System")
                .selected_text(format!("{}", self.boulder_grade_sys))
                .show_ui(ui, |ui| {
                    BoulderGradeSys::iter().for_each(|grade_sys| {
                        ui.selectable_value(&mut self.boulder_grade_sys, grade_sys, format!("{grade_sys}"));
                    });
                });
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("User:");
            ui.text_edit_singleline(&mut self.user.name);
        });
        ui.separator();
        if ui.button("Save").clicked() {
            // Save settings
        }
    }
}


struct Stats;
// May try to refactor the helper functions to belong to this, but not sure if it's worth it since most
// info belongs to the app itself

struct User {
    name: String,
}
impl Default for User {
    fn default() -> Self {
        User {
            name: String::new(),
        }
    }
}

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
    Settings,
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

#[derive(Clone)]
struct SendOptions { // All the info needed to log a send
    date: sea_orm::prelude::Date, //EGUI works really well with sea_orm's Date type so just using that
    partner: String, // Partner's name, optional (but does not need to have an Option)
    send_type: SendType, // Type of send
    attempts: i32, // Number of attempts, i32 bc that's what sea_orm/sqlite uses
    notes: String, // Any notes
    route_name: String, // Name of route, should match with a route in database
    route: Option<RouteModel>, // Route from database, fetched for the user
}
impl Default for SendOptions {
    fn default() -> Self {
        SendOptions {
            date: chrono::Utc::now().naive_utc().into(),
            partner: String::new(),
            send_type: SendType::Onsight,
            attempts: 1,
            notes: String::new(),
            route_name: String::new(),
            route: None,
        }
    }
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
    all_sessions_buffer: Arc<Mutex<Vec<SendModel>>>, // All sessions in the database, in an async context
    all_sessions: Vec<SendModel>, // All sessions in the database, out of the async
    routes_w_grades_buffer: Arc<Mutex<Vec<(RouteModel, GradeModel)>>>, // All grades in the database, in an async context
    routes_w_grades: Vec<(RouteModel, GradeModel)>, // All grades in the database, out of the async
    search_date: sea_orm::prelude::Date, // Date to search for sessions
    settings: UserSettings, // Settings for the app
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
            all_sessions_buffer: Arc::new(Mutex::new(Vec::new())),
            all_sessions: Vec::new(),
            routes_w_grades_buffer: Arc::new(Mutex::new(Vec::new())),
            routes_w_grades: Vec::new(),
            search_date: chrono::Utc::now().naive_utc().into(),
            settings: UserSettings::default(),
        };
        app.session.push(SendOptions::default());
        app
    }

    pub async fn run(){ // Run the app, can call using MyApp::run().await; and will create from scratch
        let rt = Arc::new(Some(Runtime::new().unwrap())); // Set up async runtime to be able to communicate w/ db
        let app = MyApp::new(&rt).await;
        let win_option = NativeOptions::default(); //Using default options for now
        // Run
        let _ = run_native (
            "Ascent Climbing Log",
            win_option,
            Box::new(|_cc| Ok(Box::new(app) as Box<dyn App + 'static>)),
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
                if ui.button("Settings").clicked() {
                    self.page = Page::Settings;
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

    fn header(&mut self, ctx: &eframe::egui::Context) { // Header for all pages
        egui::TopBottomPanel::top("Header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                //Display an image from the file AscentLogo.png
                let logo = egui::include_image!("../assets/AscentLogo.png");
                ui.image(logo);
                ui.heading("Ascent Climbing Log");
                
                // Back button aligned on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Back").clicked() {
                        self.reset();
                    }
                });
                
            });
        });
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_add_grade(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) { //Should not be in end product
        self.header(&ctx);
        ui.add_space(20.0);
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
    fn render_remove_grade(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) { //Should not be in end product
        self.header(&ctx);
        ui.add_space(20.0);
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
    fn render_add_route(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        self.header(&ctx);
        ui.add_space(20.0);
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
    fn render_remove_route(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) { //Make sure to check if route exists before removing, not currently doing
        self.header(&ctx);
        ui.add_space(20.0);
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
    fn render_search_home(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        self.header(&ctx);
        ui.add_space(20.0);
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
    fn render_find_route(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        self.header(&ctx);
        ui.add_space(20.0);
        
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
                    if let Ok((route, grade)) = <RoutesDb as Clone>::clone(&db).find_route_and_grade(&name).await {
                        let mut result_lock = search_result.lock().unwrap();
                        *result_lock = Some((route, grade));
                    } else {
                        let mut result_lock = search_result.lock().unwrap();
                        *result_lock = None;
                    }
                    //let mut result_lock = search_result.lock().unwrap();
                    //*result_lock = Some((route, grade));
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

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_view_route(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        self.header(&ctx);
        ui.add_space(20.0);
        ui.heading("View Route");
        if let Some((view_route, view_grade)) = self.viewing.clone() {
            ui.label(format!("Grade Id: {}", { if view_route.pitches == 0 { view_grade.hueco.unwrap().to_string() } else { view_grade.yosemite.unwrap().to_string() } }));
            ui.label(format!("Style: {}", view_route.style));
            ui.label(format!("Length: {} ft", view_route.length));
            ui.label(format!("Pitches: {}", view_route.pitches));
            //ui.label(format!("Location: {}", route.location));
            //Display notes too once implemented
        }
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_all_routes(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        self.header(&ctx);
        ui.add_space(20.0);
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
    fn render_log_session(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        self.header(&ctx);
        ui.add_space(20.0);
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
    fn render_delete_session(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        self.header(&ctx);
        ui.add_space(20.0);
        ui.heading("Delete Session");
        
        ScrollArea::vertical().show(ui, |ui| {
            
            ui.label("Please enter the session id to delete:");
            ui.separator();
            ui.horizontal(|ui| {
                ui.add(egui_extras::DatePickerButton::new(&mut self.search_date));
            });
            ui.separator();
            if ui.button("Find").clicked() {
                let session_date = self.search_date.to_string();
                let db = Arc::clone(&self.database);
                let rt = Arc::clone(&self.rt);
                let results = Arc::clone(&self.cur_session);
                rt.as_ref().as_ref().unwrap().spawn(async move {
                    let sessions = <RoutesDb as Clone>::clone(&db).get_session_by_date(session_date).await.expect("Error, could not get all sessions.");
                    let mut sessions_guard = results.lock().unwrap();
                    *sessions_guard = sessions;
                });
            }
            let mut sessions: MutexGuard<Vec<SendModel>> = self.cur_session.lock().unwrap();
            for session in sessions.clone().iter() {
                if ui.button("Delete").clicked() {
                    let db = Arc::clone(&self.database);
                    let rt = Arc::clone(&self.rt);
                    let session_id = session.session;
                    rt.as_ref().as_ref().unwrap().spawn(async move {
                        <RoutesDb as Clone>::clone(&db).remove_session(session_id).await.expect("Error, could not remove session.");
                    });
                    sessions.retain(|s| s.session != session_id);
                } else {
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
            }
            ui.separator();

            
        });
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_history(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        // Display sessions in a scroll area
        self.header(&ctx);
        ui.add_space(20.0);
        ui.heading("History");

        ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search: ");
                // Search using date picker
                ui.add(egui_extras::DatePickerButton::new(&mut self.search_date));
            });

            ui.separator();
            // Get all sessions thru async
            let db = Arc::clone(&self.database);
            let rt = Arc::clone(&self.rt);
            let results = Arc::clone(&self.cur_session);
            let session_date = self.search_date.to_string();
            rt.as_ref().as_ref().unwrap().spawn(async move {
                let sessions = <RoutesDb as Clone>::clone(&db).get_session_by_date(session_date).await.expect("Error, could not get all sessions.");
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
    fn render_view_session(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        // Display info for a single session
        self.header(&ctx);
        ui.add_space(20.0);
        ui.heading("View Session");
    }

    #[allow(clippy::too_many_lines)] //This function is long, but it's mostly just UI stuff
    fn render_stats(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        // Display a variety of stats for the user
        self.header(&ctx);
        ui.add_space(20.0);
        ui.heading("Stats");

        /* Stats to Display:
            - Total number of sends
            - Total number of sessions
            - Average number of sends per session
            - Average number of attempts per send
            - Average grade
            - Favorite climbing style
            - Favorite Route
            - Favorite Partner
            - Favorite Crag (location)
            - Flash Grade (80% chance of flashing)
            - Redpoint Grade (80% chance of sending in one session)
            - Top Tall Wall Grade (& route)
            - Top Boulder Grade (& route)
            - Scan notes for some more stats (would have to be done with some sort of ai or very complex regex)
        Other notes:
            This is a place to really get creative with stats and displays.
            Should have lots of fun stuff to look at like graphs, charts, graphics, etc.
            But before all of that, have to actually get all the relevant info to calculate the stats.
         */

        // Get the sends info
        let db = Arc::clone(&self.database);
        let rt = Arc::clone(&self.rt);
        let all_sessions = Arc::clone(&self.all_sessions_buffer);
        rt.as_ref().as_ref().unwrap().spawn(async move {
            let sessions = <RoutesDb as Clone>::clone(&db).get_all_sends().await.expect("Error, could not get all sessions.");
            let mut sessions_guard = all_sessions.lock().unwrap();
            *sessions_guard = sessions;
        });
        self.all_sessions = self.all_sessions_buffer.lock().unwrap().clone();

        
        let db = Arc::clone(&self.database);
        let rt = Arc::clone(&self.rt);
        let all_routes_w_grades = Arc::clone(&self.routes_w_grades_buffer);
        rt.as_ref().as_ref().unwrap().spawn(async move {
            let routes = <RoutesDb as Clone>::clone(&db).find_all_routes_and_grade().await.expect("Error, could not find all routes.");
            let mut routes_guard = all_routes_w_grades.lock().unwrap();
            *routes_guard = routes;
        });
        self.routes_w_grades = self.routes_w_grades_buffer.lock().unwrap().clone();

        // Display the stats
        if self.all_sessions.is_empty() || self.routes_w_grades.is_empty() {
            // Change this to a loading circle at some point
            ui.label("Loading...");
            return;
        }
        ui.vertical(|ui| {
            self.render_stats_content(ui);
        });
    }

    fn render_settings(&mut self, ctx: eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        // Display settings for the user
        self.header(&ctx);
        self.settings.render(ctx, ui);
        ui.separator();
        if ui.button("Export").clicked() {
            // Export the database
            let db = Arc::clone(&self.database);
            let rt = Arc::clone(&self.rt);
            rt.as_ref().as_ref().unwrap().spawn(async move {
                <RoutesDb as Clone>::clone(&db).to_csv().await.expect("Error, could not export database.");
            });
        }
    }

    fn render_stats_content(&self, ui: &mut eframe::egui::Ui) {
        ui.label(format!("Total Sends: {}", self.total_sends()));
        ui.label(format!("Total Sessions: {}", self.total_sessions()));
        ui.label(format!("Average Sends per Session: {}", self.avg_sends()));
        ui.label(format!("Average Attempts per Send: {}", self.avg_attempts()));
        ui.label(format!("Average Tall Wall Grade: {}", self.avg_tall_grade()));
        ui.label(format!("Average Boulder Grade: {}", self.avg_boulder_grade()));
        ui.label(format!("Favorite Style: {}", self.fav_style()));
        ui.label(format!("Favorite Route: {}", self.fav_route()));
        ui.label(format!("Favorite Partner: {}", self.fav_partner()));
        ui.label(format!("Favorite Crag: {}", self.fav_crag()));
        ui.label(format!("Flash Grade (Tall Wall): {}", self.flash_grade_tall()));
        ui.label(format!("Flash Grade (Boulder): {}", self.flash_grade_boulder()));
        ui.label(format!("Redpoint Grade (Tall Wall): {}", self.redpoint_grade_tall()));
        ui.label(format!("Redpoint Grade (Boulder): {}", self.redpoint_grade_boulder()));
        let (tall_grade, tall_route) = self.top_tall_grade();
        let (boulder_grade, boulder_route) = self.top_boulder_grade();
        ui.label(format!("Top Tall Wall Grade: {} ({})", tall_grade, tall_route));
        ui.label(format!("Top Boulder Grade: {} ({})", boulder_grade, boulder_route));
        ui.label(format!("Other Stats: {}", self.other_stats()));
    }

    fn total_sends(&self) -> i32 {
        // Get the total number of sends
        self.all_sessions.len() as i32
    }

    fn total_sessions(&self) -> i32 {
        // Get the total number of sessions (Currently just the highest id, but logic should be changed to get the number of unique ids)
        let mut total_sessions: Vec<i32> = Vec::new();
        for session in &self.all_sessions {
            if total_sessions.contains(&session.session) {
                continue;
            } else {
                total_sessions.push(session.session);
            }
        }

        total_sessions.len() as i32
    }

    fn avg_sends(&self) -> f32 {
        // Get the average number of sends per session
        if self.total_sessions() == 0 {
            return 0.0;
        }
        self.total_sends() as f32 / self.total_sessions() as f32
    }

    fn avg_attempts(&self) -> f32 {
        // Get the average number of attempts per send
        if self.total_sends() == 0 {
            return 0.0;
        }
        self.all_sessions.iter().map(|session| session.attempts).sum::<i32>() as f32 / self.total_sends() as f32
    }

    fn avg_tall_grade(&self) -> Yosemite {
        // Get the average tall wall grade
        let mut routes_and_grades: Vec<(RouteModel, GradeModel)> = Vec::new();
        for session in &self.all_sessions {
            
            let route: (RouteModel, GradeModel) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().clone();
            if route.0.pitches == 0 {
                continue;
            }
            routes_and_grades.push(route);
        }

        let mut grade_sum = 0;
        for (_route, grade) in &routes_and_grades {
            grade_sum += Yosemite::from(grade.yosemite.clone().unwrap()) as i32;
        }
        if routes_and_grades.is_empty() {
            return Yosemite::None;
        }
        let grade_num = (grade_sum / routes_and_grades.len() as i32) as i32;
        Yosemite::from(grade_num)

    }

    fn avg_boulder_grade(&self) -> Hueco {
        // Get the average boulder grade
        let mut routes_and_grades: Vec<(RouteModel, GradeModel)> = Vec::new();
        for session in &self.all_sessions {
            
            let route: (RouteModel, GradeModel) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().clone();
            if route.0.pitches != 0 {
                continue;
            }
            routes_and_grades.push(route);
        }

        let mut grade_sum = 0;
        for (_, grade) in &routes_and_grades {
            grade_sum += Hueco::from(grade.hueco.clone().unwrap()) as i32 - 1; // For some reason this needs to be -1, not sure why but it fixes everything here
        }
        if routes_and_grades.is_empty() {
            return Hueco::None;
        }
        let grade_num = grade_sum / routes_and_grades.len() as i32;
        Hueco::from(grade_num)
    }

    fn fav_style(&self) -> String {
        // Get the favorite climbing style
        
        let mut routes: Vec<RouteModel> = Vec::new();
        for session in &self.all_sessions {
            let route = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().0.clone();
            routes.push(route);
        }
        let mut style_map = std::collections::HashMap::new();
        for route in routes {
            let count = style_map.entry(route.style.clone()).or_insert(0);
            *count += 1;
        }
        let mut max = 0;
        let mut fav = String::new();
        for (style, count) in style_map {
            if count > max {
                max = count;
                fav = style;
            }
        }
        fav
    }

    fn fav_route(&self) -> String {
        // Get the favorite route
        let mut routes: Vec<RouteModel> = Vec::new();
        for session in &self.all_sessions {
            let route = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().0.clone();
            routes.push(route);
        }
        let mut route_map = std::collections::HashMap::new();
        for route in routes {
            let count = route_map.entry(route.name.clone()).or_insert(0);
            *count += 1;
        }
        let mut max = 0;
        let mut fav = String::new();
        for (route, count) in route_map {
            if count > max {
                max = count;
                fav = route;
            }
        }
        fav
    }

    fn fav_partner(&self) -> String {
        // Get the favorite partner
        let partners = self.all_sessions.iter().map(|session| session.partner.clone().unwrap_or("Solo".to_string()));
        let mut partner_map = std::collections::HashMap::new();
        for partner in partners {
            let count = partner_map.entry(partner).or_insert(0);
            *count += 1;
        }
        let mut max = 0;
        let mut fav = "Solo".to_string();
        for (partner, count) in partner_map {
            if count > max {
                max = count;
                fav = partner;
            }
        }
        fav
    }

    fn fav_crag(&self) -> String {
        // Get the favorite crag
        // Need to fix the location stuff in the database first
        "Crag".to_string()
    }

    fn flash_grade_tall(&self) -> Yosemite {
        // Get the flash grade for tall walls
        // Flash grade = hardest grade you can send first try 80% of the time
        let mut routes_and_grades: Vec<(RouteModel, GradeModel, SendModel)> = Vec::new();
        for session in &self.all_sessions {
            
            let route: (RouteModel, GradeModel) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().clone();
            if route.0.pitches == 0 {
                continue;
            }
            routes_and_grades.push((route.0, route.1, session.clone()));
        }

        // flash_map is a hashmap of the grade and (non-flashed, flashed) sends
        let mut flash_map: HashMap<GradeModel, (i32, i32)> = std::collections::HashMap::new();
        for (_route, grade, session) in &routes_and_grades {
            let count = flash_map.entry(grade.clone()).or_insert((0, 0));
            if session.attempts == 1 {
                count.1 += 1;
            } else if session.r#type != "Repeat" {
                count.0 += 1;
            }
        }
        let mut max_grade = -5;
        for (grade, (non_flashed, flashed)) in flash_map {
            if flashed as f32 / (flashed + non_flashed) as f32 >= 0.8 {
                let grade_num = Yosemite::from(grade.yosemite.clone().unwrap());
                if grade_num as i32 > max_grade {
                    max_grade = grade_num as i32;
                }
            }
        }
        Yosemite::from(max_grade)

    }

    fn flash_grade_boulder(&self) -> Hueco {
        // Get the flash grade for bouldering
        // Flash grade = hardest grade you can send first try 80% of the time
        let mut routes_and_grades: Vec<(RouteModel, GradeModel, SendModel)> = Vec::new();
        for session in &self.all_sessions {
            
            let route: (RouteModel, GradeModel) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().clone();
            if route.0.pitches != 0 {
                continue;
            }
            routes_and_grades.push((route.0, route.1, session.clone()));
        }

        let mut flash_map: HashMap<GradeModel, (i32, i32)> = std::collections::HashMap::new();
        for (_route, grade, session) in &routes_and_grades {
            let count = flash_map.entry(grade.clone()).or_insert((0, 0));
            if session.attempts == 1 {
                count.1 += 1;
            } else if session.r#type != "Repeat" {
                count.0 += 1;
            }
        }
        let mut max_grade = -5;
        for (grade, (non_flashed, flashed)) in flash_map {
            if flashed as f32 / (flashed + non_flashed) as f32 >= 0.8 && flashed >= 10 {
                let grade_num = Hueco::from(grade.hueco.clone().unwrap()) as i32 - 1;
                if grade_num > max_grade {
                    max_grade = grade_num;
                }
            }
        }
        Hueco::from(max_grade)
    }

    fn redpoint_grade_tall(&self) -> Yosemite {
        // Get the redpoint grade
        // Max grade you can get after projecting (80% chance of sending in one session)
        let mut routes_and_grades: Vec<(RouteModel, GradeModel, SendModel)> = Vec::new();
        for session in &self.all_sessions {
            
            let route: (RouteModel, GradeModel) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().clone();
            if route.0.pitches == 0 {
                continue;
            }
            routes_and_grades.push((route.0, route.1, session.clone()));
        }

        let mut redpoint_map: HashMap<GradeModel, (i32, i32)> = std::collections::HashMap::new();
        for (_route, grade, session) in &routes_and_grades {
            let count = redpoint_map.entry(grade.clone()).or_insert((0, 0));
            if session.r#type == "Redpoint" || session.r#type == "Onsight" || session.r#type == "Flash" {
                count.1 += 1;
            } else if session.r#type != "Repeat" {
                count.0 += 1;
            }
        }
        
        let mut max_grade = -5;
        for (grade, (non_redpoint, redpoint)) in redpoint_map {
            if redpoint as f32 / (redpoint + non_redpoint) as f32 >= 0.8 && redpoint >= 5 {
                let grade_num = Yosemite::from(grade.yosemite.clone().unwrap()) as i32;
                if grade_num > max_grade {
                    max_grade = grade_num;
                }
            }
        }
        Yosemite::from(max_grade)
    }

    fn redpoint_grade_boulder(&self) -> Hueco {
        // Get the redpoint grade for bouldering
        let mut routes_and_grades: Vec<(RouteModel, GradeModel, SendModel)> = Vec::new();
        for session in &self.all_sessions {
            
            let route: (RouteModel, GradeModel) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route).unwrap().clone();
            if route.0.pitches > 0 {
                continue;
            }
            routes_and_grades.push((route.0, route.1, session.clone()));
        }

        let mut redpoint_map: HashMap<GradeModel, (i32, i32)> = std::collections::HashMap::new();
        for (_route, grade, session) in &routes_and_grades {
            let count = redpoint_map.entry(grade.clone()).or_insert((0, 0));
            if session.r#type == "Redpoint" || session.r#type == "Onsight" || session.r#type == "Flash" {
                count.1 += 1;
            } else if session.r#type != "Repeat" {
                count.0 += 1;
            }
        }
        
        let mut max_grade = -5;
        for (grade, (non_redpoint, redpoint)) in redpoint_map {
            if redpoint as f32 / (redpoint + non_redpoint) as f32 >= 0.8 && redpoint >= 5 {
                let grade_num = Hueco::from(grade.hueco.clone().unwrap()) as i32;
                if grade_num > max_grade {
                    max_grade = grade_num;
                }
            }
        }
        Hueco::from(max_grade)
    }

    fn top_tall_grade(&self) -> (Yosemite, String) {
        // Get the top tall wall grade
        let mut routes_and_grades: Vec<(RouteModel, GradeModel)> = Vec::new();
        for session in &self.all_sessions {
            
            if let Some(route) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route) {
                if route.0.pitches == 0 {
                    continue;
                }
                routes_and_grades.push(route.clone());
            }
        }
        let mut max_grade = -5;
        let mut hardest_route: Option<RouteModel> = None;
        for (route, grade) in &routes_and_grades {
            let grade_num = Yosemite::from(grade.yosemite.clone().unwrap()) as i32;
            if grade_num > max_grade {
                max_grade = grade_num;
                hardest_route = Some(route.clone());
            }
        }
        if let Some(route) = hardest_route.clone() {
            return (Yosemite::from(max_grade), route.name.clone());
        } else {
            return (Yosemite::None, "None".to_string());
        }
    }

    fn top_boulder_grade(&self) -> (Hueco, String) {
        // Get the top boulder grade
        let mut routes_and_grades: Vec<(RouteModel, GradeModel)> = Vec::new();
        for session in &self.all_sessions {
            if let Some(route) = self.routes_w_grades.iter().find(|(route, _)| route.id == session.route) {
                if route.0.pitches != 0 {
                    continue;
                }
                routes_and_grades.push(route.clone());
            }
        }
        let mut max_grade = -5;
        let mut hardest_route: Option<RouteModel> = None;
        for (route, grade) in &routes_and_grades {
            let grade_num = Hueco::from(grade.hueco.clone().unwrap()) as i32 - 1;
            if grade_num > max_grade {
                max_grade = grade_num;
                hardest_route = Some(route.clone());
            }
        }
        if let Some(route) = hardest_route.clone() {
            return (Hueco::from(max_grade), route.name.clone());
        } else {
            return (Hueco::None, "None".to_string());
        }
        

    }

    fn other_stats(&self) -> String {
        // Get other stats from notes
        "Other stats may be added in the future.".to_string()
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
        self.all_sessions_buffer = Arc::new(Mutex::new(Vec::new()));
        self.all_sessions = Vec::new();
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
                Page::AddGrade => self.render_add_grade(context.clone(), ui),
                Page::RemoveGrade => self.render_remove_grade(context.clone(), ui),
                Page::AddRoute => self.render_add_route(context.clone(), ui),
                Page::RemoveRoute => self.render_remove_route(context.clone(), ui),
                Page::SearchHome => self.render_search_home(context.clone(), ui),
                Page::FindRoute => self.render_find_route(context.clone(), ui),
                Page::ViewRoute => self.render_view_route(context.clone(), ui),
                Page::ViewAllRoutes => self.render_all_routes(context.clone(), ui),
                Page::LogSession => self.render_log_session(context.clone(), ui),
                Page::RemoveSession => self.render_delete_session(context.clone(), ui),
                Page::ViewSession => self.render_view_session(context.clone(), ui),
                Page::History => self.render_history(context.clone(), ui),
                Page::Stats => self.render_stats(context.clone(), ui),
                Page::Settings => self.render_settings(context.clone(), ui),
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
