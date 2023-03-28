use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::constants;

pub fn get_config<T>(config_path_raw: &str) -> anyhow::Result<T>
where
    T: Default + DeserializeOwned + Serialize,
{
    let config_dir_path = Path::new(constants::CONFIG_DIR_PATH);
    if !config_dir_path.exists() {
        fs::create_dir_all(config_dir_path)?
    }

    let config_path = Path::new(config_path_raw);
    if !config_path.exists() {
        save_config(config_path_raw, &T::default())?;
    };

    let config_file = File::open(config_path)?;
    let config_buf = BufReader::new(config_file);

    Ok(serde_json::from_reader(config_buf)?)
}

pub fn save_config<T>(config_path: &str, new_config: &T) -> std::io::Result<()>
where
    T: Serialize,
{
    fs::write(config_path, serde_json::to_string(new_config)?.as_bytes())
}
