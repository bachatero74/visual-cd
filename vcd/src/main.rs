mod application;
mod errors;
mod filesystem;
mod structures;

use std::process::ExitCode;

use application::Application;
use log::info;

use errors::AppError;
use ratatui::DefaultTerminal;

fn main() -> ExitCode {
    match run() {
        Ok(path) => match path {
            Some(path) => {
                eprintln!("{path}");
                ExitCode::from(0)
            }
            None => ExitCode::from(1),
        },
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<Option<String>, AppError> {
    //setup_logger()?;
    info!("App start");
    let mut terminal = Terminal::new();
    Application::new()?.run(&mut terminal.t)
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

struct Terminal {
    t: DefaultTerminal,
}

impl Terminal {
    fn new() -> Self {
        Terminal { t: ratatui::init() }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
