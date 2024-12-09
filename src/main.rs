use colored::Colorize;

mod commands;
mod rel_config;
mod rel;

#[tokio::main]
async fn main() {
    let res = tokio::task::spawn_blocking(commands::figure)
        .await
        .expect("async comp not working")
        .await;

    match res {
        Ok(message) => {
            let raw = message.1;
            let out = message.0;
            let message = if raw { out } else { format!("{}", out.green()) };

            println!("{message}")
        }

        Err(error_message) => {
            let message = format!("{}", error_message.to_string().red());
            println!("{message}");
            std::process::exit(1)
        }
    }
}
