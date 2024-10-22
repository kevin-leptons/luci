use std::fmt::Display;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// There is no existing configuration file.
    NotFound,

    /// Can not read configuration file because of I/O error.
    Io(PathBuf, std::io::Error),

    /// Data in a configuration file may be leaked. File's access
    /// permission must be `400` or `600`.
    Insecured(PathBuf, Permissions),

    /// Cannot map configuration from file into Rust type.
    Deserialization(PathBuf, String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => {
                write!(f, "NotFound")
            }
            Self::Insecured(file, permissions) => {
                let mode = permissions.mode() as u16;
                let access_permission = mode << 4 >> 4;
                write!(
                    f,
                    "Insecured. permission={:o}; file={}",
                    access_permission,
                    file.to_string_lossy()
                )
            }
            Self::Io(file, error) => write!(f, "I/O: {}; file={}", error, file.to_string_lossy()),
            Self::Deserialization(file, description) => {
                write!(
                    f,
                    "Deserialization: {}; file={}",
                    description,
                    file.to_string_lossy()
                )
            }
        }
    }
}
