use std::fmt::Display;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) file: Option<PathBuf>,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Try all configuration files, and no one exists.
    NotFound,

    /// Can not read configuration file because of I/O error.
    Io(std::io::Error),

    /// Secret data in a configuration file may be leaked. File's access
    /// permission must be `400` or `600`.
    Insecured(Permissions),

    /// Can not map content of configuration file into Rust type.
    Deserialization(String),
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn file(&self) -> &Option<PathBuf> {
        &self.file
    }

    fn unwrap_file(&self) -> String {
        self.file.as_ref().unwrap().to_string_lossy().to_string()
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::NotFound => {
                write!(f, "NotFound")
            }
            ErrorKind::Insecured(permissions) => {
                let mode = permissions.mode() as u16;
                let access_permission = mode << 4 >> 4;
                write!(
                    f,
                    "Insecured. permission={:o}; file={}",
                    access_permission,
                    self.unwrap_file()
                )
            }
            ErrorKind::Io(e) => write!(f, "I/O: {}; file={}", e, self.unwrap_file()),
            ErrorKind::Deserialization(description) => {
                write!(
                    f,
                    "Deserialization: {}; file={}",
                    description,
                    self.unwrap_file()
                )
            }
        }
    }
}
