use crate::rel_config;



pub fn read()-> anyhow::Result<String>{
    let config = rel_config::Config::try_from_file()?;
    Ok(format!("Used {}", config))
}