#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FocusedField {
    Username,
    Password,
    Website,
    RawData,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub username: String,
    pub password: String,
    pub website: String,
    pub raw_data: String,
    pub focused_field: FocusedField,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            website: String::new(),
            raw_data: String::new(),
            focused_field: FocusedField::Username,
        }
    }

    pub fn active_value_mut(&mut self) -> &mut String {
        match self.focused_field {
            FocusedField::Username => &mut self.username,
            FocusedField::Password => &mut self.password,
            FocusedField::Website => &mut self.website,
            FocusedField::RawData => &mut self.raw_data,
        }
    }
}

impl From<&crate::vault::vault_data::profile::entry::Entry> for Entry {
    fn from(value: &crate::vault::vault_data::profile::entry::Entry) -> Self {
        Self {
            username: value.username.clone(),
            password: value.password.clone(),
            website: value.website.clone(),
            raw_data: value.raw_data.clone(),
            focused_field: FocusedField::Username,
        }
    }
}
