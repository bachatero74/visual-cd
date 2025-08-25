use std::{ffi::OsString, rc::Rc};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use log::info;
use ratatui::{
    DefaultTerminal, Frame,
    layout::Margin,
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::{
    errors::AppError,
    structures::{FileNode, TVItem, TreeNode},
};

pub struct Application {
    root: Rc<TreeNode>,
    tv_items: Vec<TVItem>,
    cursor: Option<usize>,
}

impl Application {
    pub fn new() -> Self {
        let root = Rc::new(TreeNode::new(FileNode {
            name: OsString::from("/"),
        }));

        Application {
            root,
            tv_items: Vec::new(),
            cursor: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        self.root.load();
        self.render_tree_view();
        if self.tv_items.len() > 0 {
            self.cursor = Some(0);
        }

        loop {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Down => info!("Down"),
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

    fn render_tree_view(&mut self) {
        self.tv_items.clear();
        self.tv_items.push(TVItem {
            tree_node: Rc::clone(&self.root),
            drawing: "".to_string(),
        });
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut items: Vec<Line> = (1..100)
            .map(|i| {
                Line::from(format!(
                    "├─────────────────┬──┐ ▸ Baaaaaaaaaardzo dłuuuuuuga linia {}",
                    i
                ))
            })
            .collect();

        items[65] = items[65].clone().bg(Color::Blue);

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
