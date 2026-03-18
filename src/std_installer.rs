use crate::StdLib;
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::{fs, io, path::Path};

pub fn install_std(dest: &Path) -> io::Result<()> {
    let files: Vec<_> = StdLib::iter().collect();
    let total = files.len() as u64;

    println!("  Installing std");

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template("  [{bar:30.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=> "),
    );

    for file in &files {
        pb.set_message(file.to_string());
        let file_path = dest.join(file.as_ref());
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = StdLib::get(file.as_ref()).unwrap();
        fs::write(&file_path, content.data)?;
        pb.inc(1);
    }

    pb.finish_with_message("Done".bright_green().bold().to_string());
    println!(
        "{} Installed std to {}",
        "✓".bright_green().bold(),
        dest.display().to_string().bold()
    );

    Ok(())
}
