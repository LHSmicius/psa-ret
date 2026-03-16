use crate::bus::can;
use crate::config::Config;
use log::debug;
use std::fs;

pub enum ActiveScreen {
    CanBus,
    Editing,
}

pub enum EditWindow {
    NewCanMsg,
    EditCanMsg,
}

pub struct App {
    pub active_screen: ActiveScreen,
    pub edit_window: Option<EditWindow>,
    pub can_messages: Vec<can::CanMessage>,
    pub app_config: Config,
}

impl App {
    pub fn new() -> App {
        let log_file = fs::File::create("Log.log").unwrap();
        env_logger::Builder::new()
            .target(env_logger::Target::Pipe(Box::new(log_file)))
            .filter_level(log::LevelFilter::Debug)
            .init();

        App {
            active_screen: ActiveScreen::CanBus,
            edit_window: None,
            can_messages: Vec::new(),
            app_config: Config::load_config_file("config.yaml"),
        }
    }

    pub fn load_can_messages(&mut self) {
        for file_in_path in
            fs::read_dir(self.app_config.database_dir.clone()).expect("Failed to read directory.")
        {
            let file_path = file_in_path.unwrap();
            let f_path = file_path.path();

            if f_path.is_file() {
                if let Some(f_ext) = f_path.extension() {
                    if f_ext == "yml" || f_ext == "yaml" {
                        if let Some(file_path_str) = f_path.to_str() {
                            debug!("PSA-RET opening file {}.", file_path_str);
                            let can_message = can::CanMessage::from_yaml_file(&file_path_str)
                                .expect("Failed to load CAN message.");
                            self.can_messages.push(can_message);
                        }
                    }
                }
            }
        }
        self.can_messages.sort_by(|a, b| a.id.cmp(&b.id));
    }
}
