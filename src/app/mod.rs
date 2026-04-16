pub mod clip_timer;
pub mod edit_mode;
pub mod entry;
mod layout;
mod notification;
pub mod state;

use crate::{errors::Result, vault::vault_data::VaultData};
use arboard::Clipboard;
use state::State;
use std::{path::PathBuf, time::Instant};

const DOUBLE_CLICK_TIMING: u128 = 500;

pub struct App {
    pub state: state::State,
    pub mode: edit_mode::Mode,
    pub is_new_vault: bool,
    pub vault_path: PathBuf,
    pub vault_pass: String,
    pub vault: Option<VaultData>,
    pub show_input: bool,

    pub clipboard: Option<Clipboard>,

    pub selected: usize,
    pub scroll: usize,

    pub clip_timer: Option<clip_timer::ClipTimer>,
    pub notification: Option<notification::Notification>,

    pub entry_form: Option<entry::Entry>,
    pub profile_name_input: String,

    pub layout: layout::LayoutCache,
    pub last_click: Option<(u16, u16, Instant)>,

    pub quit: bool,
    pub dirty: bool,
}

impl App {
    pub fn new(vault_path: PathBuf, is_new_vault: bool) -> Result<Self> {
        Ok(Self {
            state: state::State::Unlock,
            mode: edit_mode::Mode::Insert,
            is_new_vault,
            vault_path,
            vault_pass: String::new(),
            vault: None,
            show_input: false,

            clipboard: Clipboard::new().ok(),

            selected: 0,
            scroll: 0,

            clip_timer: None,
            notification: None,

            entry_form: None,
            profile_name_input: String::new(),

            layout: layout::LayoutCache::default(),
            last_click: None,

            quit: false,
            dirty: false,
        })
    }

    pub fn ntfy_info(&mut self, msg: impl Into<String>) {
        self.notification = Some(notification::Notification::info(msg));
    }

    pub fn ntfy_error(&mut self, msg: impl Into<String>) {
        self.notification = Some(notification::Notification::error(msg));
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
                if let Some(clipboard) = &mut self.clipboard {
                    if let Err(e) = crate::clipboard::clear_clipboard(clipboard) {
                        self.ntfy_error(format!("Error clearing clipboard: {}", e));
                    } else {
                        self.ntfy_info("Clipboard cleared");
                    }
                }

                self.clip_timer = None;
            }
        }
    }

    pub fn is_double_click(&mut self, col: u16, row: u16) -> bool {
        let now = Instant::now();
        let is_double = self
            .last_click
            .as_ref()
            .map(|(lc, lr, lt)| {
                *lc == col && *lr == row && lt.elapsed().as_millis() < DOUBLE_CLICK_TIMING
            })
            .unwrap_or(false);
        self.last_click = Some((col, row, now));
        is_double
    }
}
