mod application;

use std::io;
use application::Application;

fn main()-> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = Application::new();
    let res = app.run(&mut terminal);
    ratatui::restore();
    res 
}
