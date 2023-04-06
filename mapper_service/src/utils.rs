use std::{fs::File, io::BufReader, path::Path};

use mapper_service::shared_models::ServiceConfig;

pub fn get_service_config(config_path_raw: String) -> anyhow::Result<ServiceConfig> {
    let config_buf = BufReader::new(File::open(Path::new(&config_path_raw))?);
    Ok(serde_json::from_reader(config_buf)?)
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*)
    };
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

#[allow(clippy::missing_safety_doc)]
pub unsafe fn is_sys_key(key: u8) -> bool {
    *crate::SYS_KEYS_TABLE.get(key as usize).unwrap_or(&false)
}
