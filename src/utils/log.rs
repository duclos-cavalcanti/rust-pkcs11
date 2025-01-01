use std::fmt;
use std::error::Error;
use std::fs::File;
use std::io::{Write};

use colored::Colorize;

pub enum Level {
    INFO, 
    EVENT, 
    URGENT
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Level::INFO => "INFO",
            Level::EVENT => "EVENT",
            Level::URGENT => "URGENT",
        };
        write!(f, "{}", text)
    }
}

pub struct Logger {
    writer: Box<dyn Writer + Send>,
}

impl Logger {
    pub fn new(fpath: Option<&str>) -> Result<Self, Box<dyn Error>> {
        let writer: Box<dyn Writer + Send> = match fpath {
            Some(f) => Box::new(FileWriter::new(f)?),
            None => Box::new(StdoutWriter::new())
        };

        Ok(Self { writer: writer })
    }

    pub fn log(&mut self, text: String, level:Option<Level>) -> Result<(), Box<dyn Error>> {
        let level = level.unwrap_or(Level::INFO);
        let ltext = format!("{}", level);
        let text = match level {
            Level::INFO     => format!("[{}]: {}", ltext, text),
            Level::EVENT    => format!("[{}]: {}", ltext.blue(), text),
            Level::URGENT   => format!("[{}]: {}", ltext.red(), text),
        };

        self.writer.write(&text)?;
        Ok(())
    }
}

trait Writer {
    fn write(&mut self, text: &str) -> Result<(), Box<dyn Error>>;
}

struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new(fpath: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::create(fpath)?;
        Ok(Self { file })
    }
}

impl Writer for FileWriter {
    fn write(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        writeln!(self.file, "{}",text)?;
        Ok(())
    }
}

struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Writer for StdoutWriter {
    fn write(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        println!("{}", text);
        Ok(())
    }
}

