#![allow(unused_imports, dead_code, unreachable_patterns)]
use std::io;
use std::fmt;

mod routes_db;
use routes_db::RoutesDb;
use futures::executor::block_on;
mod climbing;
use climbing::*;




fn main() {
    println!("Hello, world!");
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

    /*
    let mut routeName = String::new();
    println!("Enter a route name: ");
    io::stdin().read_line(&mut routeName).expect("Failed to read line");
    let routeName = routeName.trim();
    println!("Enter the grade of the route: ");
    let mut grade = String::new();
    io::stdin().read_line(&mut grade).expect("Failed to read line");
    let grade = grade.trim();
    let grade = match grade {
        "5.1" => Grade::Yosemite(Yosemite::FiveOne),
        "5.2" => Grade::Yosemite(Yosemite::FiveTwo),
        "5.3" => Grade::Yosemite(Yosemite::FiveThree),
        "5.4" => Grade::Yosemite(Yosemite::FiveFour),
        "5.5" => Grade::Yosemite(Yosemite::FiveFive),
        "5.6" => Grade::Yosemite(Yosemite::FiveSix),
        "5.7" => Grade::Yosemite(Yosemite::FiveSeven),
        "5.8" => Grade::Yosemite(Yosemite::FiveEight),
        "5.9" => Grade::Yosemite(Yosemite::FiveNine),
        "5.10a" => Grade::Yosemite(Yosemite::FiveTenA),
        "5.10b" => Grade::Yosemite(Yosemite::FiveTenB),
        "5.10c" => Grade::Yosemite(Yosemite::FiveTenC),
        "5.10d" => Grade::Yosemite(Yosemite::FiveTenD),
        //finish the match
        _ => Grade::Yosemite(Yosemite::FiveNine),
    };

    let mut style = String::new();
    println!("Enter the style of the route: ");
    io::stdin().read_line(&mut style).expect("Failed to read line");
    let style = style.trim();
    let style = match style {
        "Boulder" => Style::Boulder,
        "Top Rope" => Style::TopRope,
        "Sport" => Style::Sport,
        "Trad" => Style::Trad,
        "Ice" => Style::Ice,
        "Alpine" => Style::Alpine,
        "Aid" => Style::Aid,
        "Speed" => Style::Speed,
        "Free Solo" => Style::FreeSolo,
        "Deep Water" => Style::DeepWater,
        _ => Style::Trad,
    };

    
    let mut length = String::new();
    println!("Enter the length of the route: ");
    io::stdin().read_line(&mut length).expect("Failed to read line");
    let length: u16 = length.trim().parse().expect("Please type a number");

    let mut pitches = String::new();
    println!("Enter the number of pitches: ");
    io::stdin().read_line(&mut pitches).expect("Failed to read line");
    let pitches: u8 = pitches.trim().parse().expect("Please type a number");

    let mut location = String::new();
    println!("Enter the location of the route: ");
    io::stdin().read_line(&mut location).expect("Failed to read line");
    let location = location.trim();

    let route = Route {
        name: String::from(routeName),
        grade: grade,
        style: vec![style],
        length: length,
        pitches: pitches,
        location: String::from(location),
    };
    println!("Here's your route: {}", route);

     */

    let v_grade = Hueco::V13;
    let y_grade: Yosemite = v_grade.into();
    println!("{} is Yosemite {}", v_grade, y_grade);

    if let Err(err) = block_on(RoutesDb::run_db()) {
        panic!("{}", err);
    } else {
        println!("Success!");
    }
}
