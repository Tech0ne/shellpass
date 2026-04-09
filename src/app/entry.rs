#[derive(Debug, Clone)]
pub struct Entry {
    pub username: String,
    pub password: String,
    pub website: String,
    pub custom_fields: Vec<(String, String)>,
    pub focused_field: usize,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            website: String::new(),
            custom_fields: Vec::new(),
            focused_field: 0,
        }
    }

    pub fn field_count(&self) -> usize {
        3 + self.custom_fields.len() * 2
    }

    pub fn active_value_mut(&mut self) -> Option<&mut String> {
        match self.focused_field {
            0 => Some(&mut self.username),
            1 => Some(&mut self.password),
            2 => Some(&mut self.website),
            n => {
                let cf_index = (n - 3) / 2;
                let is_val = (n - 3) % 2 == 1;

                if is_val {
                    Some(&mut self.custom_fields.get_mut(cf_index)?.1)
                } else {
                    Some(&mut self.custom_fields.get_mut(cf_index)?.0)
                }
            }
        }
    }
}

impl From<&crate::vault::vault_data::profile::entry::Entry> for Entry {
    fn from(value: &crate::vault::vault_data::profile::entry::Entry) -> Self {
        Self {
            username: value.username.clone(),
            password: value.password.clone(),
            website: value.website.clone(),
            custom_fields: value
                .custom_fields
                .iter()
                .map(|f| (f.key.clone(), f.val.clone()))
                .collect(),
            focused_field: 0,
        }
    }
}
