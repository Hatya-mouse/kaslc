use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
    path::Path,
};

pub fn get_file_contents(path: &Path) -> Result<String, FileLoadError> {
    if !path.exists() {
        return Err(FileLoadError::FileNotFound);
    }

    if path.is_file() {
        // Open the file
        let mut file = match File::open(path) {
            Err(why) => return Err(FileLoadError::CouldNotOpen(why)),
            Ok(file) => file,
        };

        // Get the content string of the file
        let mut str = String::new();
        match file.read_to_string(&mut str) {
            Err(why) => Err(FileLoadError::CouldNotRead(why)),
            Ok(_) => Ok(str),
        }
    } else {
        Err(FileLoadError::NotAFile)
    }
}

pub enum FileLoadError {
    FileNotFound,
    CouldNotOpen(std::io::Error),
    CouldNotRead(std::io::Error),
    NotAFile,
}

impl Display for FileLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileLoadError::FileNotFound => write!(f, "File not found"),
            FileLoadError::CouldNotOpen(why) => write!(f, "Could not open file: {}", why),
            FileLoadError::CouldNotRead(why) => write!(f, "Could not read file: {}", why),
            FileLoadError::NotAFile => write!(f, "Not a file",),
        }
    }
}
