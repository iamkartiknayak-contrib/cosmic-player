// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget;
use std::{
    cmp::Ordering,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectNode {
    Folder {
        name: String,
        path: PathBuf,
        open: bool,
        root: bool,
    },
    File {
        name: String,
        path: PathBuf,
    },
}

impl ProjectNode {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let name = path
            .file_name()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                format!("path {:?} has no file name", path),
            ))?
            .to_str()
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                format!("path {:?} is not valid UTF-8", path),
            ))?
            .to_string();
        Ok(if path.is_dir() {
            Self::Folder {
                path,
                name,
                open: false,
                root: false,
            }
        } else {
            Self::File { path, name }
        })
    }

    pub fn icon(&self, size: u16) -> Option<widget::icon::Icon> {
        match self {
            //TODO: different icon for project root?
            Self::Folder { open, .. } => Some(if *open {
                widget::icon::from_name("go-down-symbolic")
                    .size(size)
                    .into()
            } else {
                widget::icon::from_name("go-next-symbolic")
                    .size(size)
                    .into()
            }),
            Self::File { .. } => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Folder { name, .. } => name,
            Self::File { name, .. } => name,
        }
    }
}

impl Ord for ProjectNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            // Folders are always before files
            Self::Folder { .. } => {
                if let Self::File { .. } = other {
                    return Ordering::Less;
                }
            }
            // Files are always after folders
            Self::File { .. } => {
                if let Self::Folder { .. } = other {
                    return Ordering::Greater;
                }
            }
        }
        crate::localize::sorter().compare(self.name(), other.name())
    }
}

impl PartialOrd for ProjectNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
