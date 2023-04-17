use std::{fs, io};

use mapper_service::{
    shared_constants,
    shared_models::{Profile, ServiceConfig},
};
use uuid::Uuid;

use crate::{
    models::{CommandResult, ProfileSaveObj},
    utils,
};

#[tauri::command]
pub fn download_and_install_update(url_path: String, expected_installer_name: String) -> bool {
    let Some(mut new_installer_path) = tauri::api::path::download_dir() else {
        return false
    };
    new_installer_path.push(expected_installer_name);

    let Ok(response) = reqwest::blocking::get(url_path) else {
        return false
    };
    let Ok(response_bytes) = response.bytes() else {
        return false
    };
    let mut contents = io::Cursor::new(response_bytes);

    let Ok(mut new_installer) = fs::File::create(new_installer_path.clone()) else {
        return false;
    };

    if io::copy(&mut contents, &mut new_installer).is_err() {
        return false;
    }
    if new_installer.sync_all().is_err() {
        return false;
    }
    drop(new_installer);

    // TODO: If windows
    if std::process::Command::new("cmd")
        .args([
            "/C",
            "start",
            "/B",
            new_installer_path.display().to_string().as_str(),
        ])
        .spawn()
        .is_err()
    {
        return false;
    }

    std::process::exit(0); // TODO: It would be better to use https://docs.rs/tauri/1.2.4/tauri/struct.AppHandle.html#method.exit but I was unable to get it working
}

#[tauri::command]
pub fn get_service_config() -> CommandResult<ServiceConfig> {
    let Ok(config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to get service config file");
    };

    CommandResult::new_success_with_value(Some(config), None)
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn initial_check() -> CommandResult<String> {
    use std::path::Path;

    // TODO: It would be best for installer to register on_startup start, but I was unable to get that working
    let startup_script_path = Path::new(&shared_constants::parsed_home_path())
        .join("AppData")
        .join("Roaming")
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup")
        .join("start_mapper_service_on_startup.bat");
    if !startup_script_path.exists() {
        // TODO: If unable to create, let users know
        let _ = fs::write(
            startup_script_path,
            format!(
                r#"
                    @echo off
                    start powershell -Command "Start-Process -FilePath \"{}\" -WindowStyle Hidden"
                "#,
                shared_constants::get_mapper_path().display()
            ),
        );
    }

    CommandResult::new_success(None)
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn current_mapper_state() -> bool {
    use std::process::{Command, Stdio};

    // cmd /C tasklist | find /I "mapper_service.exe" > nul && echo true || echo false
    let Ok(output) = Command::new("cmd")
        .args([
            "/C",
            "tasklist",
            "/fi",
            "ImageName eq mapper_service.exe",
            "/fo",
            "LIST",
        ])
        .stdout(Stdio::piped())
        .output() else {
            return false
        };

    if !output.status.success() {
        return false;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let Some(first_line) = stdout.trim_start().split('\n').next() else {
        return false;
    };

    first_line.contains("Image Name:")
        && first_line.contains(shared_constants::MAPPER_EXECUTABLE_NAME)
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub fn change_mapper_state(new_state: bool) {
    use std::process::Command;

    let mapper_path = shared_constants::get_mapper_path();

    let mut command = Command::new("cmd");
    match new_state {
        true => command.args([
            "/C",
            "start",
            "/B",
            mapper_path.display().to_string().as_str(),
        ]),
        false => command.args([
            "/C",
            "taskkill",
            "/IM",
            shared_constants::MAPPER_EXECUTABLE_NAME,
            "/F",
        ]),
    };

    let Ok(mut child) = command.spawn() else {
        return;
    };

    let _ = child.wait();
}

// #[tauri::command]
// fn get_ui_config() -> ServiceConfig {
//     get_config::<ServiceConfig>(constants::UI_CONFIG_FILE_PATH)
//         .expect("Unable to get ui config file")
// }

#[tauri::command]
pub fn create_profile() -> CommandResult<Uuid> {
    let Ok(mut config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    let new_profile = Profile::default();
    let new_uuid = new_profile.id;
    config.profiles.push(new_profile);

    if utils::save_config(&shared_constants::service_config_file_path(), &config).is_err() {
        return CommandResult::new_err("Unable to add profile to service config file");
    }

    CommandResult::new_success_with_value(Some(new_uuid), None)
}

#[tauri::command]
pub fn delete_profile(id_to_delete: Uuid) -> CommandResult<()> {
    let Ok(mut config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    if let Some(position_to_delete) = config.profiles.iter().position(|s| s.id == id_to_delete) {
        let deleted_profile = config.profiles.remove(position_to_delete);

        // TODO: Rewrite this nested monstrosity as soon as if let chains are stable. (https://github.com/rust-lang/rust/issues/53667)
        if let Some(active_profile) = config.active_profile {
            if deleted_profile.id == active_profile {
                config.active_profile = None;
            }
        }
    } else {
        return CommandResult::new_err(
            "Unable to determine position where in config to delete based on id.",
        );
    }

    if utils::save_config(&shared_constants::service_config_file_path(), &config).is_err() {
        return CommandResult::new_err("Unable to remove profile from service config file.");
    }

    CommandResult::new_success(None)
}

#[tauri::command]
pub fn save_profile(profile: ProfileSaveObj) -> CommandResult<()> {
    let Ok(mut config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    if let Some(active_profile) = config.active_profile {
        if active_profile == profile.id {
            config.active_profile = None;
        }
    }

    if profile.use_this_profile {
        config.active_profile = Some(profile.id);
    }

    if let Some(profile_to_change) = config.profiles.iter_mut().find(|s| s.id == profile.id) {
        *profile_to_change = profile.into();
    } else {
        return CommandResult::new_err(
            "Unable to find profile in saved profiles -- unable to save.",
        );
    }

    if utils::save_config(&shared_constants::service_config_file_path(), &config).is_err() {
        return CommandResult::new_err("Unable to save new service config file");
    }

    CommandResult::new_success(None)
}
