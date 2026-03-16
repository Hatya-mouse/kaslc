use std::{fs, io, path::Path};

use crate::StdLib;

pub fn install_std(dest: &Path) -> io::Result<()> {
    for file in StdLib::iter() {
        let file_path = dest.join(file.as_ref());
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = StdLib::get(file.as_ref()).unwrap();
        fs::write(&file_path, content.data)?;
    }
    Ok(())
}
