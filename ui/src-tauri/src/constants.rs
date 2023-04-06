pub fn config_dir_path() -> String {
    format!(
        "{}/.config/KeyXpert/",
        home::home_dir()
            .expect("Home library unable to get home direcotry.")
            .display()
    )
}

pub fn ui_config_file_path() -> String {
    format!("{}config_ui.json", config_dir_path())
}

pub fn service_config_file_path() -> String {
    format!("{}config.json", config_dir_path())
}
