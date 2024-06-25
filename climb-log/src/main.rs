#![allow(dead_code, unreachable_patterns)]




mod ui;
use ui::MyApp;



#[tokio::main]
async fn main() {
    println!("Hello, world!");

    /*
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
        attempts: 1,
        send_type: SendType::FreeSolo,
        notes: String::from("Free solo"),
    };
    println!("{}", send);

    let v_grade = Hueco::V13;
    let y_grade: Yosemite = v_grade.into();
    println!("{} is Yosemite {}", v_grade, y_grade);
    let database_test = RoutesDb::new().await.expect("Failed to connect to database");
    println!("\nRoutes Database:");
    if let Err(err) = block_on(database_test.run_db()) {
        panic!("{}", err);
    } else {
        println!("Success!");
    }
    */

    
    
    MyApp::run().await;

    
}

