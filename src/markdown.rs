use regex::Regex;
use serde::Deserialize;
use std::fmt::{self};
use std::fs::{self};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const MARKDOWN_DIR: &str = "/home/pelle/dev/notes/";

#[derive(Debug)]
struct MarkdownFile {
    path: PathBuf,
    frontmatter: Frontmatter,
}

#[derive(Debug, Deserialize, Default)]
struct Frontmatter {
    id: Option<String>,
}

impl fmt::Display for Frontmatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.id {
            Some(id) => write!(f, "ID: {}", id),
            None => write!(f, "ID: None"),
        }
    }
}

impl fmt::Display for MarkdownFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Path: {}\nFrontmatter: {}",
            self.path.display(),
            self.frontmatter
        )
    }
}

fn should_ignore(path: &PathBuf) -> bool {
    match path.file_name() {
        Some(name) => name == ".git" || name == ".direnv",
        None => false,
    }
}
fn parse_markdown_file(path: &PathBuf) -> io::Result<MarkdownFile> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let frontmatter = extract_frontmatter(&content).unwrap_or_default();

    Ok(MarkdownFile {
        path: path.to_path_buf(),
        frontmatter,
    })
}

fn extract_frontmatter(content: &str) -> Option<Frontmatter> {
    let re = Regex::new(r"(?s)^---\n(.*?)\n---").unwrap();
    if let Some(captures) = re.captures(content) {
        let frontmatter_str = captures.get(1).unwrap().as_str();
        serde_yaml::from_str(frontmatter_str).ok()
    } else {
        None
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
                                markdown_files.push(path);
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
