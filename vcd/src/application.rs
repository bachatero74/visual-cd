use std::rc::Rc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Margin,
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::{
    errors::AppError,
    filesystem::load_tree,
    structures::{FileNode, TVItem, TreeNode},
};

pub struct Application {
    root: Rc<TreeNode>,
    tv_items: Vec<TVItem>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            root: load_tree(),
            tv_items: Vec::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        self.render_tree_view();
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Enter => break,
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn render_tree_view(&mut self){
        self.tv_items.clear();
    }

    fn draw(&mut self, frame: &mut Frame) {
        let items: Vec<Line> = (1..100)
            .map(|i| {
                let line = Line::from(format!(
                    "├─────────────────┬──┐ ▸ Baaaaaaaaaardzo dłuuuuuuga linia {}",
                    i
                ));

                if i == 65 { line.bg(Color::Blue) } else { line }
            })
            .collect();

        let par = Paragraph::new(items)
            .block(
                Block::default()
                    .title("Directory navigator")
                    .title_bottom("/home/jacek")
                    .title_style(Style::default().add_modifier(Modifier::REVERSED))
                    .borders(Borders::ALL)
                    .border_style(Style::new().gray()),
            )
            .scroll((50, 0));
        frame.render_widget(par, frame.area());

        let mut scrollbar_state = ScrollbarState::new(99).position(50);
        // .viewport_content_length(50);

        let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(
            scroll,
            frame.area().inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}
