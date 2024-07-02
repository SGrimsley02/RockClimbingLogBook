#![allow(dead_code, unreachable_patterns)]
#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]

mod ui;
use ui::MyApp;

#[tokio::main]
async fn main() {

    MyApp::run().await;

}

