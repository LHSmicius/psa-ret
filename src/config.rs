use log::{debug, warn};
use std::fs;
use std::{io::ErrorKind, io::Write};
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader, yaml};

pub struct Config {
    pub database_dir: String,
    pub default_lang: String,
}

impl Config {
    fn generate_default_config() -> Config {
        Config {
            database_dir: String::from("../PSA-RE/buses/AEE2004.full/HS.IS/"),
            default_lang: String::from("en"),
        }
    }

    pub fn load_config_file(file_path: &str) -> Config {
        let mut generate_config_file = false;
        let config_file = fs::read_to_string(file_path);
        let config_file = match config_file {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    debug!("Config file not found. Will create one.");
                    generate_config_file = true;
                    String::new()
                }
                _ => panic!("Failed to open file: {error:?}"),
            },
        };
        let config = YamlLoader::load_from_str(&config_file);
        let config = match config {
            Ok(file) => file,
            Err(error) => panic!("Failed to parse YAML content of configuration file: {error:?}"),
        };

        let mut new_config = Self::generate_default_config();
        if generate_config_file {
            Self::save_config(file_path, &new_config);
            return new_config;
        }

        let config_root = &config[0];
        if let Yaml::Hash(hash) = config_root {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "database_dir" => {
                            if let Yaml::String(v) = value {
                                new_config.database_dir = v.clone();
                            } else {
                                warn!("[WARNING] Wrong type for \"database_dir\".");
                            }
                        }
                        "default_lang" => {
                            if let Yaml::String(v) = value {
                                new_config.default_lang = v.clone();
                            } else {
                                warn!("[WARNING] Wrong type for \"default_lang\".");
                            }
                        }
                        _ => {
                            warn!("[WARNING] Unknown configuration parameter \"{}\".", k);
                        }
                    }
                }
            }
        }
        new_config
    }

    fn save_config(file_path: &str, config: &Config) {
        let mut hash = yaml::Hash::new();
        hash.insert(
            Yaml::String("database_dir".into()),
            Yaml::String(config.database_dir.clone()),
        );
        hash.insert(
            Yaml::String("default_lang".into()),
            Yaml::String(config.default_lang.clone()),
        );

        let yaml_doc = Yaml::Hash(hash);
        let mut file_str = String::new();
        YamlEmitter::new(&mut file_str)
            .dump(&yaml_doc)
            .expect("Failed to emit YAML.");

        let mut new_config_file = fs::File::create(file_path).unwrap();
        new_config_file
            .write_all(file_str.as_bytes())
            .expect("Failed to write to file.");
    }
}
