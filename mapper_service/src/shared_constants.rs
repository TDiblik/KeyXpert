use std::fs;

pub fn config_dir_path() -> String {
    format!(
        "{}{}.config{}KeyXpert{}",
        home::home_dir()
            .expect("Home library unable to get home directory.")
            .display(),
        std::path::MAIN_SEPARATOR,
        std::path::MAIN_SEPARATOR,
        std::path::MAIN_SEPARATOR
    )
}

pub fn ui_config_file_path() -> String {
    format!("{}config_ui.json", config_dir_path())
}

pub fn service_config_file_path() -> String {
    format!("{}config.json", config_dir_path())
}

pub fn log_error(err: &anyhow::Error) {
    let path_to_error_log = format!("{}error.log", config_dir_path());
    fs::write(path_to_error_log, err.to_string()).unwrap_or_else(|_| {
        println!(
            "Unable to write into error log --- dumping into console: {}",
            err
        )
    });

    #[cfg(debug_assertions)]
    println!("debug log err: {}", err)
}
