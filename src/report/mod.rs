use serde_derive::{Deserialize, Serialize};

use crate::rel_config::{self, Config};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileErrorLine{
    reason: String,
    content: String,
}
impl FileErrorLine {
    pub fn new(reason: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pos{
    line: usize,
    col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLines {
    files_not_found: Vec<FileErrorLine>,
    generic: Vec<String>,
}

impl ErrorLines {
    pub fn new() -> Self {
        Self { files_not_found: vec![], generic: vec![] }
    }

    pub fn add(&mut self, line: impl Into<String>) {
        self.files_not_found.push(FileErrorLine::new("file not found", line.into()));
    }
    
    pub fn add_generic(&mut self, line: impl Into<String>) {
        self.generic.push(line.into());
    }
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResultLine {
    comment: String,
    content: String,
}

impl FileResultLine {
    pub fn new(comment: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            comment: comment.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultLines {
    files_found: Vec<FileResultLine>,
}

impl ResultLines {
    pub fn new() -> Self {
        Self { files_found: vec![] }
    }

    pub fn add_files(&mut self, line: impl Into<String>) {
        self.files_found.push(FileResultLine::new("file found", line.into()));
    }
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    config: Option<rel_config::Config>,
    result_lines: ResultLines,
    error_lines: ErrorLines,
}

impl Report {
    pub fn new() -> Self {
        Self {
            result_lines: ResultLines::new(),
            error_lines: ErrorLines::new(),
            config: None,
        }
    }
    
    pub fn as_toml(&self) -> String {
        toml::to_string_pretty(self).expect("not possible")
    }
    
    pub fn add_found_file(&mut self, line: impl Into<String>) {
        self.result_lines.add_files(line);
    }
    
    pub fn add_not_found_file(&mut self, line: impl Into<String>) {
        self.error_lines.add(line);
    }
    
    pub fn add_config(&mut self, config: &Config) {
        self.config = Some(config.clone());
    }
    
    pub fn add_generic_error(&mut self, line: impl Into<String>) {
        self.error_lines.add_generic(line);
    }

    
}