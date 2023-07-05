use crate::{
    models::{CommandResult, ProfileSaveObj, SilentCmd},
    utils,
};
use mapper_service::{
    shared_constants,
    shared_models::{Profile, ServiceConfig, UIConfig},
};
use std::path::Path;
use std::process::Stdio;
use std::{fs, io, process::Command};
use sysinfo::ProcessExt;
use sysinfo::SystemExt;
use uuid::Uuid;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

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

    if Command::new("cmd") // Doesn't matter that new window spawns, because the process exits right after
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

    std::process::exit(0);
}

#[tauri::command]
pub fn get_service_config() -> CommandResult<ServiceConfig> {
    let Ok(config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to get service config file");
    };

    CommandResult::new_success_with_value(Some(config), None)
}

// Don't run intial_check during development, since it changes values of my installed instance and it's annoying :D
#[cfg(debug_assertions)]
#[tauri::command]
pub fn initial_check() -> CommandResult<()> {
    CommandResult::new_success(None)
}

#[cfg(not(debug_assertions))]
#[tauri::command]
pub fn initial_check() -> CommandResult<()> {
    let Ok(config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to get service config file");
    };
    if !config.start_on_boot {
        return CommandResult::new_success(None);
    }
    return set_registry_value_to_start_after_boot();
}

fn set_registry_value_to_start_after_boot() -> CommandResult<()> {
    let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);
    let hkey_startup = hkey_current_user
        .open_subkey_with_flags(
            Path::new("Software")
                .join("Microsoft")
                .join("Windows")
                .join("CurrentVersion")
                .join("Run"),
            winreg::enums::KEY_ALL_ACCESS,
        )
        .unwrap();

    let reg_value_option: std::result::Result<String, std::io::Error> =
        hkey_startup.get_value(shared_constants::REGISTRY_STARTUP_KEY_NAME);

    let new_registry_value = format!("\"{}\"", shared_constants::get_mapper_path().display());
    let should_set_new_value = match reg_value_option {
        Ok(reg_value) if reg_value != new_registry_value => true,
        Err(_) => true,
        _ => false,
    };

    if should_set_new_value
        && hkey_startup
            .set_value(
                shared_constants::REGISTRY_STARTUP_KEY_NAME,
                &new_registry_value,
            )
            .is_err()
    {
        return CommandResult::new_err(
            "Unable to set registry value that enables mapper to run after login.",
        );
    }

    CommandResult::new_success(None)
}

fn remove_registry_value_from_booting() -> CommandResult<()> {
    let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);
    let hkey_startup = hkey_current_user
        .open_subkey_with_flags(
            Path::new("Software")
                .join("Microsoft")
                .join("Windows")
                .join("CurrentVersion")
                .join("Run"),
            winreg::enums::KEY_ALL_ACCESS,
        )
        .unwrap();

    if hkey_startup
        .set_value(shared_constants::REGISTRY_STARTUP_KEY_NAME, &"")
        .is_err()
    {
        return CommandResult::new_err(
            "Unable to set registry value that disables mapper to run after login.",
        );
    }

    CommandResult::new_success(None)
}

#[tauri::command]
pub fn current_mapper_state() -> bool {
    let Ok(output) = Command::new_silent_cmd()
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

#[tauri::command]
pub fn change_mapper_state(new_state: bool) {
    let mut system = sysinfo::System::new();
    system.refresh_all();

    if !new_state {
        for process in system.processes_by_name("mapper_service.exe") {
            process.kill();
        }
    } else {
        let mapper_path = shared_constants::get_mapper_path();
        let mapper_path_string = mapper_path.display().to_string();

        let mapper_path_prepared = mapper_path_string
            .split('\\')
            .map(|s| match s.contains(' ') {
                true => format!("\"{}\"", s),
                false => s.to_string(),
            })
            .collect::<Vec<String>>()
            .join("\\");

        let mut command = Command::new_silent_cmd();
        command
            .arg("/C")
            .arg(format!(r#"start /B {}"#, mapper_path_prepared));

        let Ok(mut child) = command.spawn() else {
               return;
            };

        let _ = child.wait();
    }
}

#[tauri::command]
pub fn get_ui_config() -> CommandResult<UIConfig> {
    let Ok(config) = utils::get_config::<UIConfig>(shared_constants::ui_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    CommandResult::new_success_with_value(Some(config), None)
}

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

#[tauri::command]
pub fn save_advanced_settings(
    start_on_boot: bool,
    enable_recursive_remapping: bool,
    enable_recursive_shortcuts: bool,
) -> CommandResult<()> {
    let Ok(mut config) = utils::get_config::<ServiceConfig>(shared_constants::service_config_file_path()) else {
        return CommandResult::new_err("Unable to read or parse service config file.");
    };

    if !start_on_boot {
        let answ = remove_registry_value_from_booting();
        if !answ.is_success {
            return answ;
        }
    } else {
        let answ = set_registry_value_to_start_after_boot();
        if !answ.is_success {
            return answ;
        }
    }

    config.start_on_boot = start_on_boot;
    config.enable_recursive_remapping = enable_recursive_remapping;
    config.enable_recursive_shortcuts = enable_recursive_shortcuts;
    if utils::save_config(&shared_constants::service_config_file_path(), &config).is_err() {
        return CommandResult::new_err("Unable to save new service config file.");
    }

    CommandResult::new_success(None)
}
