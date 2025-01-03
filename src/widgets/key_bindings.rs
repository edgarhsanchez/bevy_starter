use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyModifiers};
use anyhow::Result;
use bevy::prelude::*;
use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Position, Rect}, style::{Color, Style}, widgets::{BorderType, Borders, WidgetRef}};
use crate::widgets::mouse_context::MouseContext;

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub defined_action_description: String,
    pub user_key: crossterm::event::KeyCode,
    pub user_key_modifiers: crossterm::event::KeyModifiers,
    pub system_key: crossterm::event::KeyCode,
    pub system_key_modifiers: crossterm::event::KeyModifiers,
    pub is_customizable: bool,
}

#[derive(Debug, Default, Resource, Deref, DerefMut, Clone)]
pub struct KeyBindings {
    pub key_bindings: HashMap<(crossterm::event::KeyCode, crossterm::event::KeyModifiers), KeyBinding>,
    pub rect: Rect,
    #[deref]
    pub id: uuid::Uuid,
}

pub trait KeyBindingsTrait {
    fn convert_to_system_key_binding(&self, user_key: crossterm::event::KeyCode, user_key_modifiers: crossterm::event::KeyModifiers) -> Result<(crossterm::event::KeyCode, crossterm::event::KeyModifiers), anyhow::Error> ;
    fn add_custom_key_binding(&mut self, key_binding: KeyBinding) -> Result<(), anyhow::Error>;
}


impl KeyBindingsTrait for KeyBindings {
    fn convert_to_system_key_binding(&self, user_key: crossterm::event::KeyCode, user_key_modifiers: crossterm::event::KeyModifiers) -> Result<(crossterm::event::KeyCode, crossterm::event::KeyModifiers), anyhow::Error> {
        // match user_key and modifiers to system_key and modifiers
        let matched = self.key_bindings.get(&(user_key, user_key_modifiers));
        match matched {
            Some(key_binding) => {
                Ok((key_binding.system_key, key_binding.system_key_modifiers))
            },
            None => {
                Err(anyhow::Error::msg("No matching key binding found"))
            }
        }
    }

    fn add_custom_key_binding(&mut self, key_binding: KeyBinding) -> Result<(), anyhow::Error> {
        // if key binding already exists, return error
        if self.key_bindings.contains_key(&(key_binding.user_key, key_binding.user_key_modifiers)) {
            return Err(anyhow::Error::msg("Key binding already exists"));
        }
        // add custom key binding
        self.key_bindings.insert((key_binding.user_key, key_binding.user_key_modifiers), key_binding);
        Ok(())
    }
}

impl MouseContext for KeyBindings {
    fn is_over(&self, position: Position) -> bool {
        self.rect.contains(position)
    }

    fn set_context(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl WidgetRef for KeyBindings {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        // create block for key bindings popup
        let block = ratatui::widgets::Block::default()
            .title("Key Bindings")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);
        buf.set_style(
            area,
            Style::default()
            .bg(Color::DarkGray)
            .fg(Color::White)
        );

        // add table rectangle for key bindings
        let l0 = Layout::horizontal([
            Constraint::Percentage(100),
        ]).flex(Flex::Legacy)
        .split(area);

        // add 
        


        block.render_ref(area, buf);
    }
}