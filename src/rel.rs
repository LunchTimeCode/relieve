use crate::{
    rel_config::{self, Config},
    report::Report,
};
use walkdir::WalkDir;

pub fn read() -> anyhow::Result<String> {
    let config = rel_config::Config::try_from_file()?;
    let mut report = Report::new();
    report.add_config(&config);
    let _ = files_available(config, &mut report);

    Ok(report.as_toml())
}

fn files_available(config: Config, report: &mut Report) -> anyhow::Result<Vec<String>> {
    let mut found_paths = vec![];

    let mut left_over = config.files_names().clone();

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if config.contains_file(f_name.as_ref()) {
            if let Some(p) = entry.path().to_str() {
                found_paths.push(p.to_string());
                report.add_found_file(p.to_string());
                let index = left_over
                    .iter()
                    .position(|x| f_name.eq_ignore_ascii_case(x))
                    .unwrap();
                left_over.remove(index);
            } else {
                report.add_generic_error("Could not create Path");
            }
        }
    }

    left_over
        .iter()
        .for_each(|f| report.add_not_found_file(f.to_string()));

    Ok(found_paths)
}
