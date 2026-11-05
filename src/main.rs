mod app;
mod config;
mod editor;
mod events;
mod filesystem;
mod opener;
mod search;
mod transfer;
mod ui;

fn main() {
    if let Err(err) = app::App::run_main() {
        eprintln!("TFSV error: {err}");
        std::process::exit(1);
    }
}
