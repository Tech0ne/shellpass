#[derive(Debug, Clone, PartialEq)]
pub enum State {
    EditEntry {
        profile_index: usize,
        entry_index: Option<usize>,
    },
    EditProfile {
        profile_index: Option<usize>,
    },
    EntryDetail {
        profile_index: usize,
        entry_index: usize,
    },
    EntryList {
        profile_index: usize,
    },
    ProfileList,
    Unlock,
}
