use app::{NaukaApp, App, AppConfiguration};
use clap::Parser;

mod opcode;
mod emulator;
mod app;

fn main() {
    let args = AppConfiguration::parse();

    let mut app = NaukaApp::new(args);
    app.run();
}
