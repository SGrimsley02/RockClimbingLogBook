#![allow(dead_code, unreachable_patterns)]

mod ui;
use ui::MyApp;

#[tokio::main]
async fn main() {

    MyApp::run().await;

}

