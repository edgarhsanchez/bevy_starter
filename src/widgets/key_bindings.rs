use crate::mini_salsa::theme::THEME;
use anyhow::Result;
use bevy::{input::keyboard::Key, prelude::*, utils::tracing::span};
use crossterm::event::{KeyCode, KeyModifiers};
use rat_ftable::{
    selection::RowSelection,
    textdata::{Cell, Row},
    Table, TableContext, TableData, TableSelection, TableState,
};
use rat_scrolled::Scroll;
use ratatui::{widgets::Widget,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, StatefulWidgetRef, StatefulWidget, WidgetRef, block},
};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub defined_action_description: String,
    pub user_key: crossterm::event::KeyCode,
    pub user_key_modifiers: crossterm::event::KeyModifiers,
    pub system_key: crossterm::event::KeyCode,
    pub system_key_modifiers: crossterm::event::KeyModifiers,
    pub is_customizable: bool,
}

impl Default for KeyBinding {
    fn default() -> Self {
        Self {
            defined_action_description: String::new(),
            user_key: crossterm::event::KeyCode::Null,
            user_key_modifiers: crossterm::event::KeyModifiers::empty(),
            system_key: crossterm::event::KeyCode::Null,
            system_key_modifiers: crossterm::event::KeyModifiers::empty(),
            is_customizable: false,
        }
    }
}

#[derive(Debug, Default, Resource, Deref, DerefMut, Clone)]
pub struct KeyBindings {
    pub key_bindings:
        HashMap<(crossterm::event::KeyCode, crossterm::event::KeyModifiers), KeyBinding>,
    pub rect: Rect,
    #[deref]
    pub id: uuid::Uuid,
    pub lead_row: Option<usize>,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self {
            key_bindings: HashMap::new(),
            rect: Rect::default(),
            id: uuid::Uuid::new_v4(),
            lead_row: None,
        }
    }

    pub fn select(&mut self, row: usize) {
        self.lead_row = Some(row);
    }
}

impl TableSelection for KeyBindings {
    fn is_selected_row(&self, row: usize) -> bool {
        self.lead_row == Some(row)
    }

    fn is_selected_column(&self, _column: usize) -> bool {
        false
    }

    fn is_selected_cell(&self, _column: usize, _row: usize) -> bool {
        false
    }

    fn lead_selection(&self) -> Option<(usize, usize)> {
        self.lead_row.map(|v| (0, v))
    }
}

fn convert_key_code_and_modifiers_to_string(
    key_code: KeyCode,
    key_modifiers: KeyModifiers,
) -> String {
    let key_code_string: String = match key_code {
        KeyCode::Null => "Null".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::F(n) => format!("F{}", n),
        KeyCode::Char(c) => format!("{}", c),
        KeyCode::NumLock => "NumLock".to_string(),
        KeyCode::ScrollLock => "ScrollLock".to_string(),
        KeyCode::CapsLock => "CapsLock".to_string(),
        KeyCode::PrintScreen => "PrintScreen".to_string(),
        KeyCode::Pause => "Pause".to_string(),
        KeyCode::Menu => "Menu".to_string(),
        KeyCode::KeypadBegin => "KeypadBegin".to_string(),
        KeyCode::Media(c) => format!("Media({})", c),
        KeyCode::Modifier(c) => format!("Modifier({})", c),
    };

    let mut key_modifiers_string = "";
    if key_modifiers.contains(KeyModifiers::CONTROL) {
        key_modifiers_string = "Ctrl";
    }
    if key_modifiers.contains(KeyModifiers::ALT) {
        key_modifiers_string = "Alt";
    }
    if key_modifiers.contains(KeyModifiers::SHIFT) {
        key_modifiers_string = "Shift";
    }
    if key_modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT) {
        key_modifiers_string = "Ctrl+Alt";
    }
    if key_modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) {
        key_modifiers_string = "Ctrl+Shift";
    }
    if key_modifiers.contains(KeyModifiers::ALT | KeyModifiers::SHIFT) {
        key_modifiers_string = "Alt+Shift";
    }
    if key_modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT) {
        key_modifiers_string = "Ctrl+Alt+Shift";
    }

    format!("{}+{}", key_code_string, key_modifiers_string)
}

#[derive(Debug, Default, Clone)]
pub struct KeyBindingsState {
    table_state: TableState<RowSelection>,
}

impl KeyBindingsState {
    pub fn new() -> Self {
        Self {
            table_state: TableState::default(),
        }
    }

    pub fn select(&mut self, id: usize) {
        self.table_state.select(Some(id));
    }

    pub fn selected(&mut self) -> Option<usize> {
        self.table_state.selected()
    }

    pub fn select_at_mouse(&mut self, position: Position) {
        let item_index = self.table_state.row_at_clicked((position.x, position.y));
        self.table_state.select(item_index);
    }
}

pub trait KeyBindingsTrait {
    fn convert_to_system_key_binding(
        &self,
        user_key: crossterm::event::KeyCode,
        user_key_modifiers: crossterm::event::KeyModifiers,
    ) -> Result<(crossterm::event::KeyCode, crossterm::event::KeyModifiers), anyhow::Error>;
    fn add_custom_key_binding(&mut self, key_binding: KeyBinding) -> Result<(), anyhow::Error>;
}

impl KeyBindingsTrait for KeyBindings {
    fn convert_to_system_key_binding(
        &self,
        user_key: crossterm::event::KeyCode,
        user_key_modifiers: crossterm::event::KeyModifiers,
    ) -> Result<(crossterm::event::KeyCode, crossterm::event::KeyModifiers), anyhow::Error> {
        // match user_key and modifiers to system_key and modifiers
        let matched = self.key_bindings.get(&(user_key, user_key_modifiers));
        match matched {
            Some(key_binding) => Ok((key_binding.system_key, key_binding.system_key_modifiers)),
            None => Err(anyhow::Error::msg("No matching key binding found")),
        }
    }

    fn add_custom_key_binding(&mut self, key_binding: KeyBinding) -> Result<(), anyhow::Error> {
        // if key binding already exists, return error
        if self
            .key_bindings
            .contains_key(&(key_binding.user_key, key_binding.user_key_modifiers))
        {
            return Err(anyhow::Error::msg("Key binding already exists"));
        }
        // add custom key binding
        self.key_bindings.insert(
            (key_binding.user_key, key_binding.user_key_modifiers),
            key_binding,
        );
        Ok(())
    }
}

impl StatefulWidgetRef for KeyBindings {
    type State = KeyBindingsState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        // chunk in the middle of the screen
        let sub_area = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .split(area)[0];

        // add table rectangle for key bindings
        let l0 = Layout::horizontal([Constraint::Percentage(100)])
            .flex(Flex::Legacy)
            .split(sub_area);

        // prepare dataslice for keybindings
        struct DataSlice<'a>(&'a [KeyBinding]);

        impl<'a> TableData<'a> for DataSlice<'a> {
            fn rows(&self) -> usize {
                self.0.len()
            }

            fn row_style(&self, _row: usize) -> Option<Style> {
                self.0.get(_row).map(|d| {
                    if d.is_customizable {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::White)
                    }
                })
            }

            fn render_cell(
                &self,
                _ctx: &TableContext,
                column: usize,
                row: usize,
                area: Rect,
                buf: &mut Buffer,
            ) {
                if let Some(d) = self.0.get(row) {
                    match column {
                        0 => {
                            let span = Span::from(d.defined_action_description.clone());
                            span.render(area, buf);
                        }
                        1 => {
                            // convert KeyBinding KeyCode and KeyModifiers to string
                            let system_key_binding = convert_key_code_and_modifiers_to_string(
                                d.system_key,
                                d.system_key_modifiers,
                            );
                            let user_key_binding = convert_key_code_and_modifiers_to_string(
                                d.user_key,
                                d.user_key_modifiers,
                            );
                            if user_key_binding.is_empty() {
                                let span = Span::from(system_key_binding);
                                span.style(Style::default().fg(Color::Gray))
                                    .render(area, buf);
                            } else {
                                let span = Span::from(user_key_binding);
                                span.style(Style::default().fg(Color::White))
                                    .render(area, buf);
                            }
                        }
                        _ => {
                            // not a defined column
                        }
                    }
                }
            }
        }

        // sort key_binding in key_bindings by is_customizable
        let mut key_bindings: Vec<KeyBinding> = self.key_bindings.values().cloned().collect();
        key_bindings.sort_by(|a, b| a.is_customizable.cmp(&b.is_customizable));

        // define table
        Table::default()
            .data(DataSlice(key_bindings.as_slice()))
            .widths([Constraint::Percentage(70), Constraint::Percentage(30)])
            .column_spacing(1)
            .header(Row::new([Cell::from("Action"), Cell::from("Key Binding")]))
            .footer(Row::new([Cell::from(""), Cell::from("")]))
            .block(
                Block::bordered()
                .border_type(block::BorderType::Rounded)
                .border_style(THEME.block())
                .title_style(THEME.block_title())
                .title("Key Bindings"),
            )
            .vscroll(Scroll::new())
            .styles(THEME.table_style())
            .render(l0[0], buf, &mut state.table_state);
    }
}
