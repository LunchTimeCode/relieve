use crate::rel_config;
use walkdir::WalkDir;

pub struct ErrorLines {
    pub lines: Vec<String>,
}

impl ErrorLines {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }

    pub fn add(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }
}

pub fn read() -> anyhow::Result<String> {
    let config = rel_config::Config::try_from_file()?;
    let mut errs = ErrorLines::new();
    let found = files_available(config.files(), &mut errs);

    Ok(format!(
        "Used config: {} | found {} | t find {}",
        config,
        found?.join(", "),
        errs.lines.join(", ")
    ))
}

fn files_available(
    files: Vec<String>,
    error_lines: &mut ErrorLines,
) -> anyhow::Result<Vec<String>> {
    let mut found_paths = vec![];

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if files.contains(&f_name.to_string()) && sec.elapsed()?.as_secs() < 86400 {
            if let Some(p) = entry.path().to_str() {
                found_paths.push(p.to_string());
            } else {
                error_lines.add("Could not create Path");
            }
        } else {
            error_lines.add(format!("Could not find {f_name}"));
        }
    }

    Ok(found_paths)
}
