use anyhow::anyhow;
use std::{fs::File, io::BufReader, path::Path};

use mapper_service::{
    shared_constants,
    shared_models::{Profile, ServiceConfig},
};

pub fn get_service_config(config_path_raw: String) -> anyhow::Result<ServiceConfig> {
    let config_buf = BufReader::new(File::open(Path::new(&config_path_raw))?);
    Ok(serde_json::from_reader(config_buf)?)
}

pub fn get_active_profile() -> anyhow::Result<Profile> {
    let Ok(service_config) = get_service_config(shared_constants::service_config_file_path()) else {
        return Err(anyhow!("Unable to get/parse service config file."));
    };
    let Some(active_profile_id) = service_config.active_profile else {
        return Err(anyhow!("No profile active, shutting down."));
    };

    let Some(active_profile) = service_config.profiles.iter().find(|s| s.id == active_profile_id) else {
        return Err(
            anyhow!(
                format!("Profiles ({}) do not include active profile id ({})", 
                    service_config.profiles.iter().map(|s| s.id.to_string()).collect::<Vec<String>>().join(", "), 
                    active_profile_id
                )
            )
        );
    };

    Ok(active_profile.clone())
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*)
    };
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn is_sys_key(key: u8) -> bool {
    *crate::SYS_KEYS_TABLE.get(key as usize).unwrap_or(&false)
}

#[macro_export]
macro_rules! call_next_hook {
    ($n_code:expr, $w_param:expr, $l_param:expr) => {
        return CallNextHookEx(WINDOW_HHOOK, $n_code, $w_param, $l_param)
    };
}

#[macro_export]
macro_rules! event_handled {
    () => {
        return 1;
    };
}

#[macro_export]
macro_rules! map_virtual_key {
    ($key:expr) => {
        MapVirtualKeyW($key as u32, MAPVK_VK_TO_VSC) as u8
    };
}

#[macro_export]
macro_rules! keybd_trigger_key_up {
    ($key:expr, $scan_code:expr) => {
        keybd_event(
            $key as u8,
            $scan_code,
            KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
            0,
        );
    };
}

#[macro_export]
macro_rules! keybd_trigger_key_down {
    ($key:expr, $scan_code:expr) => {
        keybd_event($key as u8, $scan_code, KEYEVENTF_EXTENDEDKEY, 0);
    };
}
