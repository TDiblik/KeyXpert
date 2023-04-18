use std::{path::PathBuf, time::SystemTime};

use chrono::{DateTime, Utc};

#[cfg(target_os = "windows")]
pub const MAPPER_EXECUTABLE_NAME: &str = "mapper_service.exe";
#[cfg(target_family = "unix")]
pub const MAPPER_EXECUTABLE_NAME: &str = "mapper_service";

#[cfg(target_os = "windows")]
pub const REGISTRY_STARTUP_KEY_NAME: &str = "KeyXpert_start_mapper_after_startup";

/// Following format (ends with separator): C:\\Users\UserName\
pub fn parsed_home_path() -> String {
    format!(
        "{}{}",
        home::home_dir()
            .expect("Home library unable to get home directory.")
            .display(),
        std::path::MAIN_SEPARATOR
    )
}

pub fn get_mapper_path() -> PathBuf {
    let mut mapper_path = std::env::current_exe().unwrap();
    mapper_path.pop();
    mapper_path.push(MAPPER_EXECUTABLE_NAME);
    return mapper_path;
}

pub fn config_dir_path() -> String {
    format!(
        "{}.config{}KeyXpert{}",
        parsed_home_path(),
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
    let now: DateTime<Utc> = SystemTime::now().into();
    let error_formated = format!("[{}]: {}", now.format("%d.%m.%Y %H:%M:%S"), err);

    #[cfg(not(debug_assertions))]
    append_to_file(format!("{}error.log", config_dir_path()), &error_formated).unwrap_or_else(
        |_| {
            println!(
                "Unable to write into error log --- dumping into console: {}",
                error_formated
            );
        },
    );

    #[cfg(debug_assertions)]
    println!("debug log err: {}", error_formated);
}

#[cfg(not(debug_assertions))]
fn append_to_file(raw_path: String, contents: &String) -> anyhow::Result<()> {
    use std::{
        fs::OpenOptions,
        io::{BufWriter, Write},
    };

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .truncate(false)
        .open(raw_path)?;

    let mut file_buffer = BufWriter::new(file);
    writeln!(file_buffer, "{}", contents)?;

    Ok(())
}
