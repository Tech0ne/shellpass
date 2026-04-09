use arboard::Clipboard;

use crate::errors::{Error, Result};

pub fn clear_clipboard(clipboard: &mut Clipboard) -> Result<()> {
    clipboard.set_text("")?;
    clipboard.clear()?;
    Ok(())
}

pub fn copy_to_clipboard(clipboard: &mut Clipboard, data: &str) -> Result<()> {
    clipboard.set_text(data).map_err(|e| Error::clipboard(e))
}
