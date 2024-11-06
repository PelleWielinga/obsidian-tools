use std::fs;
use std::path::{Path, PathBuf};

const MARKDOWN_DIR: &str = "/Users/pelle/dev/notes/";

fn should_ignore(path: &PathBuf) -> bool {
    match path.file_name() {
        Some(name) => name == ".git" || name == ".direnv",
        None => false,
    }
}

fn list_markdown_files(dir: &Path) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path(); 

                        if !should_ignore(&path) {
                            if path.is_dir() {
                                list_markdown_files(&path);
                            } else if path.is_file() && path.extension().map(|s| s == "md").unwrap_or(false) {
                                println!("{}", path.display());
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}

fn main() {
    let markdown_path = Path::new(MARKDOWN_DIR);
    list_markdown_files(markdown_path);
}
