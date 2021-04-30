use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fmt, fs, io};

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::components::tree::TreeNode;

#[derive(Serialize, Deserialize, Clone, Lens, Debug)]
pub struct FileEntry {
    pub name: String,
    pub ext: String,
    pub is_dir: bool,
    pub path: String,
    pub children: Vec<FileEntry>,
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: "".to_string(),
            ext: "".to_string(),
            is_dir: false,
            path: "".to_string(),
            children: vec![],
        }
    }
}

impl FileEntry {
    pub fn from_path(path: PathBuf) -> Self {
        let file_name = path.file_name().unwrap();
        let name = match file_name.to_str() {
            None => "".to_string(),
            Some(na) => na.to_string(),
        };
        let ext = match path.extension() {
            None => "".to_string(),
            Some(ext) => ext.to_str().unwrap().to_string(),
        };

        let path = format!("{}", path.display());

        FileEntry {
            name,
            ext,
            is_dir: false,
            path,
            children: vec![],
        }
    }
    pub fn new(name: String) -> Self {
        FileEntry {
            name: name,
            ext: "".to_string(),
            is_dir: false,
            path: "".to_string(),
            children: vec![],
        }
    }

    pub fn add_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        if !entry.path().is_dir() {
            return entry
                .file_name()
                .to_str()
                .map(|s| s == ".DS_Store")
                .unwrap_or(false);
        }

        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    pub fn from_dir(title: String, dir: &Arc<Path>) -> FileEntry {
        let mut root = FileEntry::new(title);
        let _result = FileEntry::visit_dirs(dir, 0, &mut root, dir);
        root
    }

    fn visit_dirs(
        dir: &Path,
        depth: usize,
        node: &mut FileEntry,
        base_dir: &Path,
    ) -> io::Result<()> {
        if dir.is_dir() {
            let entry_set = fs::read_dir(dir)?; // contains DirEntry
            let mut entries = entry_set
                .filter_map(|v| match v {
                    Ok(dir) => {
                        if FileEntry::is_hidden(&dir) {
                            return None;
                        }
                        Some(dir)
                    }
                    Err(_) => None,
                })
                .collect::<Vec<_>>();

            entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

            for (_index, entry) in entries.iter().enumerate() {
                let path = entry.path();

                if path.is_dir() {
                    let depth = depth + 1;
                    let relative_path = path.strip_prefix(base_dir).unwrap();
                    let entry = &mut FileEntry::new(format!("{}", relative_path.display()));
                    entry.is_dir = true;
                    FileEntry::visit_dirs(&path, depth, entry, base_dir)?;
                    node.children.push(entry.to_owned());
                } else {
                    let entry1 = FileEntry::from_path(path);
                    node.children.push(entry1);
                }
            }
        }
        Ok(())
    }
}

impl Data for FileEntry {
    fn same(&self, other: &Self) -> bool {
        self.name.same(&other.name)
            && self.children.len() == other.children.len()
            && self
                .children
                .iter()
                .zip(other.children.iter())
                .all(|(a, b)| a.same(b))
    }
}

impl TreeNode for FileEntry {
    fn children_count(&self) -> usize {
        self.children.len()
    }

    fn get_child(&self, index: usize) -> &FileEntry {
        &self.children[index]
    }

    fn get_child_mut(&mut self, index: usize) -> &mut FileEntry {
        &mut self.children[index]
    }
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}
