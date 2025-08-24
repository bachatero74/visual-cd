mod application;
mod errors;
mod filesystem;
mod structures;

use application::Application;
use log::{info, warn};

use errors::AppError;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}
fn run() -> Result<(), AppError> {
    setup_logger()?;
    info!("App start");
    let mut terminal = ratatui::init();
    let res = Application::new().run(&mut terminal);
    ratatui::restore();
    res
}

fn setup_logger() -> Result<(), AppError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("output.log").map_err(|_| AppError::StatStr("Cannot open log file"))?)
        .apply()?;
    Ok(())
}
