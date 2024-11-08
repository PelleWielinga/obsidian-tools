use std::path::{Path, PathBuf};
use std::{fs, io};

const MARKDOWN_DIR: &str = "/home/pelle/dev/notes/";

fn should_ignore(path: &PathBuf) -> bool {
    match path.file_name() {
        Some(name) => name == ".git" || name == ".direnv",
        None => false,
    }
}

fn list_markdown_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut markdown_files = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();

                        if !should_ignore(&path) {
                            if path.is_dir() {
                                markdown_files.extend(list_markdown_files(&path)?);
                            } else if path.is_file()
                                && path.extension().map(|s| s == "md").unwrap_or(false)
                            {
                                println!("{}", path.display());
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        Err(e) => return Err(e),
    }

    return Ok(markdown_files);
}

fn main() {
    let markdown_path = Path::new(MARKDOWN_DIR);
    match list_markdown_files(markdown_path) {
        Ok(files) => {
            for path in files {
                println!("{}", path.display())
            }
        }
        Err(e) => eprintln!("Error listing markdown files: {}", e),
    }
}
