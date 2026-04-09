use log::{debug, warn};
use std::{fmt, fs};
use yaml_rust2::{Yaml, YamlLoader};

#[derive(Debug, Clone, Default)]
pub struct Translation {
    pub en: Option<String>,
    pub fr: Option<String>,
    pub de: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub alt_names: Option<Vec<String>>,
    pub bits: Option<String>,
    pub data_type: Option<String>,
    pub signed: Option<bool>,
    pub factor: Option<f64>,
    pub offset: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub units: Option<String>,
    pub comment: Option<Translation>,
    pub values: Vec<(i64, Option<Translation>)>,
    pub unused: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub enum BusType {
    CAN,
    VAN,
    Kline,
    Error,
    #[default]
    NA,
}

impl fmt::Display for BusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusType::CAN => write!(f, "CAN"),
            BusType::VAN => write!(f, "VAN"),
            BusType::Kline => write!(f, "K-L"),
            BusType::Error => write!(f, "Err"),
            BusType::NA => write!(f, "N/A"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Periodicity {
    Ms(i64),
    Trigger,
    TriggerOrMs(i64),
    Error,
    #[default]
    NA,
}

impl fmt::Display for Periodicity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Periodicity::Ms(ms) => write!(f, "{: >4}ms", ms),
            Periodicity::Trigger => write!(f, " Trig "),
            Periodicity::TriggerOrMs(ms) => write!(f, "T{: >5}", ms),
            Periodicity::Error => write!(f, "Error "),
            Periodicity::NA => write!(f, "N/A   "),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CanMessage {
    pub id: Option<String>,
    pub name: Option<String>,
    pub alt_names: Option<Vec<String>>,
    pub length: Option<i64>,
    pub comment: Option<Translation>,
    pub bus_type: BusType,
    pub periodicity: Periodicity,
    pub senders: Vec<String>,
    pub receivers: Vec<String>,
    pub signals: Vec<(String, Signal)>,
}

impl Translation {
    fn from_yaml(yaml: &Yaml) -> Option<Translation> {
        if let Yaml::Hash(hash) = yaml {
            let mut translation = Translation {
                en: None,
                fr: None,
                de: None,
            };

            for (key, value) in hash {
                if let (Yaml::String(k), Yaml::String(v)) = (key, value) {
                    match k.as_str() {
                        "en" => translation.en = Some(v.clone()),
                        "fr" => translation.fr = Some(v.clone()),
                        "de" => translation.de = Some(v.clone()),
                        _ => {
                            warn!("[WARNING] Unsupported language \"{}\".", k);
                        }
                    }
                } else {
                    warn!("[WARNING] Wrong type for language translation.");
                }
            }

            Some(translation)
        } else {
            None
        }
    }

    pub fn get(&self, language: &str) -> &str {
        match language {
            "en" => self.en.as_deref().unwrap_or(""),
            "fr" => self.fr.as_deref().unwrap_or(""),
            "de" => self.de.as_deref().unwrap_or(""),
            _ => self.en.as_deref().unwrap_or(""),
        }
    }
}

impl Signal {
    fn log_warn_wrong_type(signal_param: &str) {
        warn!("[WARNING] Wrong type for signal's parameter \"{signal_param}\".");
    }

    fn from_yaml(yaml: &Yaml) -> Signal {
        let mut signal = Signal {
            alt_names: None,
            bits: None,
            data_type: None,
            signed: None,
            factor: None,
            offset: None,
            min: None,
            max: None,
            units: None,
            comment: None,
            values: Vec::new(),
            unused: None,
        };

        if let Yaml::Hash(hash) = yaml {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "alt_names" => {
                            if let Yaml::Array(arr) = value {
                                let mut alt_names = Vec::new();
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        alt_names.push(s.clone());
                                    }
                                }
                                if !alt_names.is_empty() {
                                    signal.alt_names = Some(alt_names);
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "bits" => {
                            if let Yaml::String(v) = value {
                                signal.bits = Some(String::from(v));
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "type" => {
                            if let Yaml::String(v) = value {
                                signal.data_type = Some(v.clone());
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "signed" => {
                            if let Yaml::Boolean(v) = value {
                                signal.signed = Some(*v);
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "factor" => {
                            signal.factor = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    Self::log_warn_wrong_type(k);
                                    None
                                }
                            };
                        }
                        "offset" => {
                            signal.offset = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    Self::log_warn_wrong_type(k);
                                    None
                                }
                            };
                        }
                        "min" => {
                            signal.min = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    Self::log_warn_wrong_type(k);
                                    None
                                }
                            };
                        }
                        "max" => {
                            signal.max = match value {
                                Yaml::Real(v) => v.parse().ok(),
                                Yaml::Integer(v) => Some(*v as f64),
                                _ => {
                                    Self::log_warn_wrong_type(k);
                                    None
                                }
                            };
                        }
                        "units" => {
                            if let Yaml::String(v) = value {
                                signal.units = Some(v.clone());
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "comment" => {
                            signal.comment = Translation::from_yaml(&value);
                        }
                        "values" => {
                            if let Yaml::Hash(value_hash) = value {
                                for (value_key, value_val) in value_hash {
                                    if let Yaml::Integer(value_num) = value_key {
                                        let explanation = Translation::from_yaml(value_val);
                                        signal.values.push((*value_num, explanation));
                                    } else {
                                        warn!("[WARNING] Expected integer in field \"values\".");
                                    }
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "unused" => {
                            if let Yaml::Boolean(v) = value {
                                signal.unused = Some(*v);
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        _ => {
                            warn!("[WARNING] Unknown CAN signal parameter \"{}\".", k);
                        }
                    }
                } else {
                    Self::log_warn_wrong_type("NAME");
                }
            }
        }
        signal
    }
}

impl CanMessage {
    fn log_warn_wrong_type(message_param: &str) {
        warn!("[WARNING] Wrong type for message's parameter \"{message_param}\".");
    }

    pub fn from_yaml_file(file_path: &str) -> Result<CanMessage, String> {
        let file_result = fs::read_to_string(file_path);
        let file = match file_result {
            Ok(f) => f,
            Err(_) => return Err(String::from("Can't read file.")),
        };

        let yaml_tree_result = YamlLoader::load_from_str(&file);
        let yaml_tree = match yaml_tree_result {
            Ok(yaml) => yaml,
            Err(_) => return Err(String::from("Can't parse YAML tree.")),
        };
        let yaml_root = &yaml_tree[0];

        let mut message = CanMessage {
            id: None,
            name: None,
            alt_names: None,
            length: None,
            comment: None,
            bus_type: BusType::NA,
            periodicity: Periodicity::NA,
            senders: Vec::new(),
            receivers: Vec::new(),
            signals: Vec::new(),
        };

        debug!("Loading CAN message header.");
        if let Yaml::Hash(hash) = yaml_root {
            for (key, value) in hash {
                if let Yaml::String(k) = key {
                    match k.as_str() {
                        "id" => {
                            message.id = match value {
                                Yaml::String(v) => Some(v.clone()),
                                Yaml::Integer(v) => Some(format!("0x{:X}", v)),
                                _ => None,
                            };
                        }
                        "name" => {
                            if let Yaml::String(v) = value {
                                message.name = Some(v.clone());
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "alt_names" => {
                            if let Yaml::Array(arr) = value {
                                let mut alt_names = Vec::new();
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        alt_names.push(s.clone());
                                    }
                                }
                                if !alt_names.is_empty() {
                                    message.alt_names = Some(alt_names);
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "length" => {
                            if let Yaml::Integer(v) = value {
                                message.length = Some(*v);
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "comment" => {
                            message.comment = Translation::from_yaml(&value);
                        }
                        "type" => {
                            if let Yaml::String(v) = value {
                                message.bus_type = match v.as_str() {
                                    "can" => BusType::CAN,
                                    _ => {
                                        warn!("Unknown BusType {v}");
                                        BusType::Error
                                    }
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "periodicity" => {
                            if let Yaml::Integer(number) = value {
                                message.periodicity = Periodicity::Ms(*number);
                            } else if let Yaml::String(text) = value {
                                if text.eq("trigger") {
                                    message.periodicity = Periodicity::Trigger;
                                } else if text.ends_with(" ms") {
                                    let number: i64 = text.trim_end_matches(" ms").parse().unwrap();
                                    message.periodicity = Periodicity::Ms(number);
                                } else if text.ends_with("ms") {
                                    let number: i64 = text.trim_end_matches("ms").parse().unwrap();
                                    message.periodicity = Periodicity::Ms(number);
                                } else {
                                    warn!("[WARNING] Unable to parse \"periodicity\".");
                                    message.periodicity = Periodicity::Error;
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "senders" => {
                            if let Yaml::Array(arr) = value {
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        message.senders.push(s.clone());
                                    }
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "receivers" => {
                            if let Yaml::Array(arr) = value {
                                for item in arr {
                                    if let Yaml::String(s) = item {
                                        message.receivers.push(s.clone());
                                    }
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        "signals" => {
                            if let Yaml::Hash(signals_hash) = value {
                                for (signal_key, signal_value) in signals_hash {
                                    if let Yaml::String(signal_name) = signal_key {
                                        debug!("Loading CAN signal: {}.", signal_name);
                                        let signal = Signal::from_yaml(signal_value);
                                        message.signals.push((signal_name.clone(), signal));
                                    } else {
                                        Self::log_warn_wrong_type("signal's name");
                                    }
                                }
                            } else {
                                Self::log_warn_wrong_type(k);
                            }
                        }
                        _ => {
                            warn!("[WARNING] Unknown CAN message parameter \"{}\".", k);
                        }
                    }
                } else {
                    Self::log_warn_wrong_type("KEY");
                }
            }
        }
        Ok(message)
    }
}
