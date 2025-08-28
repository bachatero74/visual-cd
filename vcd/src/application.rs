use std::{
    env,
    ffi::OsString,
    path::{Component, Components},
    rc::Rc,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use log::{error, info};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Margin,
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::{
    errors::AppError,
    filesystem::get_current_root,
    structures::{FileNode, TVItem, TreeNode},
};

pub struct Application {
    root: (Option<OsString>, Rc<TreeNode>),
    tv_items: Vec<TVItem>,
    cursor: usize,
}

impl Application {
    pub fn new() -> Result<Self, AppError> {
        let (prefix, root_name) = get_current_root()?;
        let root = Rc::new(TreeNode::new(FileNode { name: root_name }));

        Ok(Application {
            root: (prefix, root),
            tv_items: Vec::new(),
            cursor: 0,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        self.root.1.load();

        match {
            let found = self.find(&mut env::current_dir()?.components());
            self.render_tree_view();
            found
        } {
            Ok(node) => self.goto(&node),
            Err(e) => error!("Cannot navigate to current dir: {e}"),
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
        fn add_node(list: &mut Vec<TVItem>, node: &Rc<TreeNode>, level: usize) {
            list.push(TVItem {
                tree_node: Rc::clone(node),
                drawing: " ".repeat(4 * level),
            });
            if let Some(ref sns) = *node.subnodes.borrow() {
                for subn in sns {
                    add_node(list, subn, level + 1);
                }
            }
        }

        self.tv_items.clear();
        add_node(&mut self.tv_items, &self.root.1, 0);
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut items: Vec<Line> = self
            .tv_items
            .iter()
            .map(|tvi| {
                Line::from(format!(
                    "{} {} {}",
                    tvi.drawing,
                    "▸",
                    tvi.tree_node.file_node.name.to_string_lossy(),
                ))
            })
            .collect();

        if self.cursor < items.len() {
            items[self.cursor] = items[self.cursor].clone().bg(Color::Blue);
        }

        let current_path = self
            .tv_items
            .get(self.cursor)
            .map_or(String::from("?"), |tvi| {
                tvi.tree_node.get_path().to_string_lossy().to_string()
            });

        let par = Paragraph::new(items)
            .block(
                Block::default()
                    .title("Visual cd")
                    .title_bottom(current_path)
                    .title_style(Style::default().add_modifier(Modifier::REVERSED))
                    .borders(Borders::ALL)
                    .border_style(Style::new().gray()),
            )
            .scroll((0, 0));
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

    fn find(&self, components: &mut Components) -> Result<Rc<TreeNode>, AppError> {
        let msg = "Cannot find specified path";
        let c = components.next().ok_or(AppError::StatStr(msg))?;
        match c {
            Component::Prefix(p) => {
                let prefix = self.root.0.as_ref().ok_or(AppError::StatStr(msg))?;
                if p.as_os_str() == prefix {
                    match components.next().ok_or(AppError::StatStr(msg))? {
                        Component::RootDir => self.root.1.find(components),
                        _ => Err(AppError::StatStr(msg)),
                    }
                } else {
                    Err(AppError::StatStr(msg))
                }
            }
            Component::RootDir => self.root.1.find(components),
            _ => Err(AppError::StatStr(msg)),
        }
    }

    fn goto(&mut self, node: &Rc<TreeNode>) {
        self.cursor = self
            .tv_items
            .iter()
            .position(|tvi| Rc::ptr_eq(&tvi.tree_node, node))
            .unwrap_or(self.cursor);
    }
}
