use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub trait Log {
    fn log<S: Into<String>>(&mut self, message: S);
}

#[derive(Debug)]
pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new<S1: Into<String>, S2: Into<String>>(dir: S1, filename: S2) -> Logger
        where S1: std::convert::AsRef<std::ffi::OsStr> {
        let dir_path = Path::new(&dir);
        let file_path = dir_path.join(filename.into());
        let file = {
            if !file_path.exists() {
                File::create(file_path).unwrap()
            } else {
                OpenOptions::new().append(true).open(file_path).unwrap()
            }
        };
        Logger {
            file,
        }
    }
}

impl Log for Logger {
    fn log<S: Into<String>>(&mut self, message: S) {
        writeln!(self.file, "{}", message.into()).unwrap();
        self.file.sync_all();
    }
}
