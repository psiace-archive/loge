use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileWriter {
    /// The path to the logging file.
    path: PathBuf,
    writer: BufWriter<File>,
}

impl FileWriter {
    pub fn new(path: PathBuf) -> FileWriter {
        let file =
            OpenOptions::new().write(true).append(true).create(true).open(path.as_path()).unwrap();

        FileWriter { path, writer: BufWriter::new(file) }
    }

    pub fn write(&mut self, record: String) -> io::Result<()> {
        let writer = self.writer.get_mut();
        let result = writeln!(writer, "{}", record);
        if result.is_err() {
            return result;
        }
        Ok(())
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}
