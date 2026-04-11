pub mod clip_timer;
pub mod entry;
mod notification;
pub mod state;
pub mod vim_mode;

use crate::vault::vault_data::VaultData;
use state::State;
use std::path::PathBuf;

pub struct App {
    pub state: state::State,
    pub mode: vim_mode::Mode,
    pub is_new_vault: bool,
    pub vault_path: PathBuf,
    pub vault: Option<VaultData>,
    pub password_input: String,
    pub show_input: bool,

    pub selected: usize,
    pub scroll: usize,
    pub command_buf: String,

    pub clip_timer: Option<clip_timer::ClipTimer>,
    pub notification: Option<notification::Notification>,

    pub entry_form: Option<entry::Entry>,
    pub profile_name_input: String,

    pub detail_selected: usize,
    pub quit: bool,
    pub dirty: bool,

    // ? need a rework on the vim keybinds
    pub count_buf: String,
    pub pending_z: bool,
}

impl App {
    pub fn new(vault_path: PathBuf, is_new_vault: bool) -> Self {
        Self {
            state: state::State::Unlock,
            mode: vim_mode::Mode::Insert,
            is_new_vault,
            vault_path,
            vault: None,
            password_input: String::new(),
            show_input: false,

            selected: 0,
            scroll: 0,
            command_buf: String::new(),

            clip_timer: None,
            notification: None,

            entry_form: None,
            profile_name_input: String::new(),

            detail_selected: 0,
            quit: false,
            dirty: false,

            count_buf: String::new(),
            pending_z: false,
        }
    }

    pub fn ntfy_info(&mut self, msg: impl Into<String>) {
        self.notification = Some(notification::Notification::info(msg));
    }

    pub fn ntfy_error(&mut self, msg: impl Into<String>) {
        self.notification = Some(notification::Notification::info(msg));
    }

    pub fn current_profile_count(&self) -> usize {
        self.vault.as_ref().map(|v| v.profiles.len()).unwrap_or(0)
    }

    pub fn current_entry_count(&self) -> usize {
        match &self.state {
            State::EntryList { profile_index }
            | State::EntryDetail { profile_index, .. }
            | State::EditEntry { profile_index, .. } => self
                .vault
                .as_ref()
                .and_then(|v| v.profiles.get(*profile_index))
                .map(|p| p.entries.len())
                .unwrap_or(0),
            _ => 0,
        }
    }

    pub fn tick(&mut self) {
        if let Some(n) = &self.notification {
            if n.expired() {
                self.notification = None;
            }
        }

        if let Some(t) = &self.clip_timer {
            if t.expired() {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    if let Err(e) = crate::clipboard::clear_clipboard(&mut clipboard) {
                        self.ntfy_error(format!("Error clearing clipboard: {}", e));
                    } else {
                        self.ntfy_info("Clipboard cleared");
                    }

                    self.clip_timer = None;
                }
            }
        }
    }
}
