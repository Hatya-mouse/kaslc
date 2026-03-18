use std::{fs::File, io::Read, path::Path};

pub fn get_file_contents(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("The file {} does not exist", path.display()));
    }

    if path.is_file() {
        // Open the file
        let mut file = match File::open(path) {
            Err(why) => return Err(format!("couldn't open {}: {}", path.display(), why)),
            Ok(file) => file,
        };

        // Get the content string of the file
        let mut str = String::new();
        match file.read_to_string(&mut str) {
            Err(why) => Err(format!("couldn't read {}: {}", path.display(), why)),
            Ok(_) => Ok(str),
        }
    } else {
        Err(format!("{} is not a file", path.display()))
    }
}
