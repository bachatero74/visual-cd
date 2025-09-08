use std::{
    env,
    ffi::OsString,
    path::{Component, Components, PathBuf},
    rc::Rc,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use log::error;
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

const V_MARGIN: isize = 3;
const COLLAPSED_DIR: &str = "📁";
const EXPANDED_DIR: &str = "📂";

// const COLLAPSED_DIR: &str = "⊞ ";
// const EXPANDED_DIR: &str = "⊟";

// const COLLAPSED_DIR: &str = "▸";
// const EXPANDED_DIR: &str = " ▾";

// const COLLAPSED_DIR: &str = "+";
// const EXPANDED_DIR: &str = "-";

pub struct Application {
    root: (Option<OsString>, Rc<TreeNode>),
    tv_items: Vec<TVItem>,
    cursor: isize,
    display_offset: isize,
}

impl Application {
    pub fn new() -> Result<Self, AppError> {
        let (prefix, root_name) = get_current_root()?;
        let root = Rc::new(TreeNode::new(FileNode { name: root_name }));

        Ok(Application {
            root: (prefix, root),
            tv_items: Vec::new(),
            cursor: 0,
            display_offset: 0,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<Option<String>, AppError> {
        self.root.1.load();

        match {
            let start_dir = env::args().nth(1).map_or(env::current_dir()?, PathBuf::from);
            let found = self.find(&mut start_dir.components());
            self.render_tree_view();
            found
        } {
            Ok(node) => self.goto(&node),
            Err(e) => error!("Cannot navigate to current dir: {e}"),
        }

        let path: Option<String> = loop {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Up => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                                self.display_offset -= 1;
                            } else {
                                if self.cursor > 0 {
                                    self.cursor -= 1;
                                }
                            }
                        }

                        KeyCode::Down => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                                self.display_offset += 1;
                            } else {
                                if self.cursor < self.tv_items.len() as isize - 1 {
                                    self.cursor += 1;
                                }
                            }
                        }

                        KeyCode::Left => {
                            if let Some(tvi) = self.tv_items.get(self.cursor as usize) {
                                if tvi.tree_node.is_loaded() {
                                    tvi.tree_node.unload();
                                } else {
                                    if let Some(parent) = tvi.tree_node.parent.upgrade() {
                                        self.goto(&parent);
                                    }
                                }
                                self.render_tree_view();
                            }
                        }

                        KeyCode::Right => {
                            if let Some(tvi) = self.tv_items.get(self.cursor as usize) {
                                if tvi.tree_node.is_loaded() {
                                    let first = tvi
                                        .tree_node
                                        .subnodes
                                        .borrow()
                                        .as_ref()
                                        .and_then(|children| children.first().cloned());

                                    if let Some(child) = first {
                                        self.goto(&child);
                                    }
                                } else {
                                    tvi.tree_node.load();
                                    self.render_tree_view();
                                }
                            }
                        }

                        // KeyCode::Backspace => {
                        //     if let Some(tvi) = self.tv_items.get(self.cursor as usize) {
                        //         if let Some(parent) = tvi.tree_node.parent.upgrade() {
                        //             self.goto(&parent);
                        //             self.render_tree_view();
                        //         }
                        //     }
                        // }
                        KeyCode::Enter => {
                            let tvi = self.tv_items.get(self.cursor as usize).ok_or_else(|| {
                                AppError::Str(format!("Nieprawidłowy indeks {}", self.cursor))
                            })?;

                            break Some(tvi.tree_node.get_path().to_string_lossy().to_string());
                        }
                        KeyCode::Esc => break None,

                        code if code >= KeyCode::Char('a') && code <= KeyCode::Char('z') => {
                            if let Some(chr) = code.as_char() {
                                if let Some(tvi) = self.tv_items.get(self.cursor as usize) {
                                    if let Some(next) = &tvi.tree_node.find_next(chr) {
                                        self.goto(next);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            };
        };
        Ok(path)
    }

    fn render_tree_view(&mut self) {
        fn add_node(
            list: &mut Vec<TVItem>,
            node: &Rc<TreeNode>,
            prevs_stack: &mut Vec<bool>,
            not_last: Option<bool>,
        ) {
            let lead: String = prevs_stack
                .iter()
                .map(|b| if *b { "│   " } else { "    " })
                .collect();

            let link = match not_last {
                Some(tbc) => match tbc {
                    true => "├───",
                    false => "└───",
                },
                None => "",
            };
            list.push(TVItem {
                tree_node: Rc::clone(node),
                drawing: format!("{lead}{link}"),
            });
            if let Some(not_last) = not_last {
                prevs_stack.push(not_last);
            }
            if let Some(ref sns) = *node.subnodes.borrow() {
                for (i, subn) in sns.iter().enumerate() {
                    add_node(list, subn, prevs_stack, Some(i < sns.len() - 1));
                }
            }
            if not_last.is_some() {
                prevs_stack.pop();
            }
        }

        self.tv_items.clear();
        let mut prevs_stack = Vec::new();
        add_node(&mut self.tv_items, &self.root.1, &mut prevs_stack, None);

        if self.cursor >= self.tv_items.len() as isize {
            self.cursor = (self.tv_items.len() as isize - 1).max(0);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut items: Vec<Line> = self
            .tv_items
            .iter()
            .map(|tvi| {
                Line::from(format!(
                    "{}{} {}",
                    tvi.drawing,
                    if tvi.tree_node.subnodes.borrow().is_some() {
                        EXPANDED_DIR
                    } else {
                        COLLAPSED_DIR
                    },
                    tvi.tree_node.file_node.name.to_string_lossy(),
                ))
            })
            .collect();

        if self.cursor >= 0 && self.cursor < items.len() as isize {
            items[self.cursor as usize] = items[self.cursor as usize].clone().bg(Color::Blue);
        }

        let current_path = self
            .tv_items
            .get(self.cursor as usize)
            .map_or(String::from("?"), |tvi| {
                tvi.tree_node.get_path().to_string_lossy().to_string()
            });

        self.display_offset = self.calc_offset(frame);

        let par = Paragraph::new(items)
            .block(
                Block::default()
                    .title("Visual cd")
                    .title_bottom(current_path)
                    .title_style(
                        Style::default()
                            .add_modifier(Modifier::REVERSED)
                            .fg(Color::Cyan),
                    )
                    .borders(Borders::ALL)
                    .border_style(Style::new().cyan()),
            )
            .scroll((self.display_offset as u16, 0));
        frame.render_widget(par, frame.area());

        let len = self.tv_items.len() as isize - (frame.area().height as isize - 2);
        if len > 0 {
            let mut scrollbar_state =
                ScrollbarState::new(len as usize).position(self.display_offset as usize);
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

    fn calc_offset(&self, frame: &Frame) -> isize {
        let height = frame.area().height as isize - 2;
        if height > 0 {
            let location = self.cursor - self.display_offset;
            let margin = V_MARGIN.min(height / 2);
            let max_offs = (self.tv_items.len() as isize - height).max(0);
            {
                if location < margin {
                    self.cursor - margin
                } else if location >= height - margin {
                    self.cursor - height + margin + 1
                } else {
                    self.display_offset
                }
            }
            .clamp(0, max_offs)
        } else {
            self.display_offset
        }
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
            .unwrap_or(self.cursor as usize) as isize;
    }
}
