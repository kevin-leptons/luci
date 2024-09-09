use libc::{S_IRWXG, S_IRWXO, S_IXUSR};
use serde::de::{self, DeserializeOwned};
use std::fs::File;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::error::{Error, ErrorKind};
use crate::result::Result;

/// Go through a list of `JSONC` files. For each file, try to map it into data
/// structure `T`. Return result on the first success or failure. The
/// configuration file must be secured by `mode=600`.
///
/// # Examples
///
/// ```no_run
#[doc = include_str!("../examples/read_jsonc.rs")]
///
pub fn read_jsonc<T, P>(files: impl Iterator<Item = P>) -> Result<(T, PathBuf)>
where
    T: de::DeserializeOwned,
    P: AsRef<Path>,
{
    for file in files {
        match read_jsonc_file(file.as_ref()) {
            Ok(v) => {
                return Ok((v, file.as_ref().to_path_buf()));
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {}
                _ => return Err(e),
            },
        }
    }
    let error = Error {
        kind: ErrorKind::NotFound,
        file: None,
    };
    Err(error)
}

fn read_jsonc_file<T, P>(path: P) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let data = read_secured_file(path.as_ref())?;
    let config = parse_json(data, path)?;
    return Ok(config);
}

fn read_secured_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    assert_secured_file(path.as_ref())?;
    match fs::read_to_string(path.as_ref()) {
        Ok(v) => Ok(v),
        Err(e) => Err(from_io_error(e, path)),
    }
}

fn assert_secured_file<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file = match File::open(path.as_ref()) {
        Ok(v) => v,
        Err(e) => return Err(from_io_error(e, path)),
    };
    let metadata = match file.metadata() {
        Ok(v) => v,
        Err(e) => return Err(from_io_error(e, path)),
    };
    let mode = metadata.permissions().mode();
    if (mode & S_IXUSR > 0) || (mode & S_IRWXO > 0) || (mode & S_IRWXG > 0) {
        let error = Error {
            kind: ErrorKind::Insecured(metadata.permissions()),
            file: Some(path.as_ref().to_path_buf()),
        };
        return Err(error);
    }
    return Ok(());
}

fn parse_json<T, P>(value: String, file: P) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    match serde_hjson::from_str(&value) {
        Ok(v) => Ok(v),
        Err(e) => {
            let error = Error {
                kind: ErrorKind::Deserialization(e.to_string()),
                file: Some(file.as_ref().to_path_buf()),
            };
            Err(error)
        }
    }
}

fn from_io_error<P>(error: io::Error, path: P) -> Error
where
    P: AsRef<Path>,
{
    match error.kind() {
        io::ErrorKind::NotFound => Error {
            kind: ErrorKind::NotFound,
            file: None,
        },
        _ => Error {
            kind: ErrorKind::Io(error),
            file: Some(path.as_ref().to_path_buf()),
        },
    }
}
