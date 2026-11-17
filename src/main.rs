use tfsv::app::App;

fn restore_terminal() {
    let _ = crossterm::terminal::disable_raw_mode();
    let mut stdout = std::io::stdout();
    let _ = crossterm::execute!(
        stdout,
        crossterm::event::DisableMouseCapture,
        crossterm::terminal::LeaveAlternateScreen
    );
}

fn main() {
    // Set a panic hook to guarantee terminal raw mode and alternate screen
    // are restored cleanly even if an unhandled panic occurs anywhere in the application.
    let default_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore_terminal();
        default_panic_hook(panic_info);
    }));

    if let Err(err) = App::run_main() {
        restore_terminal();
        eprintln!("TFSV error: {err}");
        std::process::exit(1);
    }
}
