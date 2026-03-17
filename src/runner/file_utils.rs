use std::{fs::File, io::Read, path::Path};

pub fn get_file_contents(path: &Path) -> Option<String> {
    if path.is_file() {
        // Open the file
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        // Get the content string of the file
        let mut str = String::new();
        match file.read_to_string(&mut str) {
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
            Ok(_) => Some(str),
        }
    } else {
        None
    }
}
