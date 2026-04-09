use crate::bus::can;
use crate::config::Config;
use log::{debug, error};
use ratatui::widgets;
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
    pub table_state: widgets::TableState,
}

impl App {
    pub fn new() -> App {
        let log_file = fs::File::create("Log.log").unwrap();
        env_logger::Builder::new()
            .target(env_logger::Target::Pipe(Box::new(log_file)))
            .filter_level(log::LevelFilter::Debug)
            .init();

        let mut table_state = widgets::TableState::default();
        table_state.select(Some(0));

        App {
            active_screen: ActiveScreen::CanBus,
            edit_window: None,
            can_messages: Vec::new(),
            app_config: Config::load_config_file("config.yaml"),
            table_state,
        }
    }

    pub fn next_message(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.can_messages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_message(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.can_messages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn load_can_messages(&mut self) {
        for file_in_path in
            fs::read_dir(&self.app_config.database_dir).expect("Failed to read directory.")
        {
            let file_path = file_in_path.unwrap();
            let f_path = file_path.path();

            if f_path.is_file() {
                if let Some(f_ext) = f_path.extension() {
                    if f_ext == "yml" || f_ext == "yaml" {
                        if let Some(file_path_str) = f_path.to_str() {
                            debug!("PSA-RET opening file {}.", file_path_str);
                            let can_message = match can::CanMessage::from_yaml_file(&file_path_str)
                            {
                                Ok(msg) => msg,
                                Err(error) => {
                                    error!("File {file_path_str}: {error}");
                                    return;
                                }
                            };
                            self.can_messages.push(can_message);
                        }
                    }
                }
            }
        }
        self.can_messages.sort_by(|a, b| a.id.cmp(&b.id));
    }
}
