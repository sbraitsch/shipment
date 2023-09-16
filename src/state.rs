use std::fs;
use std::fs::File;
use std::io::{Read};
use std::process::Command;
use ratatui::style::Color;
use crate::state::Status::EXITED;

#[derive(Clone)]
pub enum CurrentScreen {
    Main,
    Detail(Container),
    Log(Container),
    File(Container)
}

#[derive(Clone)]
pub enum Status {
    UP(usize),
    EXITED(usize),
    ERROR(String)
}

#[derive(Clone)]
pub struct Container {
    pub name: String,
    pub cpu: f32,
    pub mem: f32,
    pub status: Status,
    pub logs: String
}

pub struct Theme {
    pub pastel_blue: Color,
    pub mint: Color
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            pastel_blue: Color::Rgb(137, 180, 250),
            mint: Color::Rgb(106, 151, 153)
        }
    }
}

pub struct Sebulba {
    pub selected_idx: Option<usize>,
    pub all_containers: Vec<Container>,
    pub current_screen: CurrentScreen,
    pub info: Result<(), String>,
    pub theme: Theme,
    pub offset: usize
}

impl Sebulba {
    pub fn new() -> Sebulba {
        let mut sebulba = Sebulba {
            selected_idx: None,
            all_containers: vec![],
            current_screen: CurrentScreen::Main,
            info: Ok(()),
            theme: Theme::new(),
            offset: 0
        };

        sebulba.list_files();
        sebulba
    }

    pub fn select_next(&mut self) {
        let max_idx = self.all_containers.len() -1;
        match self.selected_idx {
            Some(value) if value == max_idx => self.selected_idx = Some(0),
            None => self.selected_idx = Some(0),
            Some(idx) => self.selected_idx = Some(idx + 1),
        }
        self.commit_selection()
    }

    pub fn select_prev(&mut self) {
        let max_idx = self.all_containers.len() -1;
        match self.selected_idx {
            Some(0) | None => self.selected_idx = Some(max_idx),
            Some(idx) => self.selected_idx = Some(idx - 1)
        }
        self.commit_selection()
    }

    pub fn inc_offset(&mut self) {
        self.offset = self.offset + 1
    }

    pub fn dec_offset(&mut self) {
        match self.offset {
            0 => {},
            _ => self.offset = self.offset - 1
        }
    }

    pub fn commit_selection(&mut self) {
        self.offset = 0;
        if let Some(idx) = self.selected_idx {
            let mut container_to_view = self.all_containers[idx].clone();
            match File::open(&container_to_view.name) {
                Ok(mut file) => {
                    let mut content = String::new();
                    match file.read_to_string(&mut content) {
                        Ok(_) => {
                            container_to_view.logs = content.into();
                            self.current_screen = CurrentScreen::Detail(container_to_view);
                        }
                        Err(_) => { self.info = Err("File couldn't be opened.".into()) }
                    }
                }
                Err(_) => { self.info = Err("File couldn't be opened.".into()) }
            }
        }

    }

    pub fn docker_ps_a(&mut self) {
        let output = Command::new("docker")
            .arg("ps")
            .arg("-a")
            .output()
            .expect("Failed to execute command");

        let _output_str = String::from_utf8_lossy(&output.stdout);
    }

    pub fn list_files(&mut self) {
        self.selected_idx = None;
        // Get the current directory (folder)
        let current_dir = std::env::current_dir().unwrap();

        // Read the contents of the directory
        let entries = fs::read_dir(current_dir).unwrap();

        // Iterate over the directory entries and print the file names
        let mut found_containers = vec![];
        entries.for_each(|entry| {
            if let Ok(dir_entry) = entry {
                let path = dir_entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        found_containers.push(Container {
                            name: file_name.to_string_lossy().parse().unwrap(),
                            cpu: 0.0,
                            mem: 0.0,
                            status: EXITED(0),
                            logs: String::new()
                        });
                    }
                }
            }
        });
        self.all_containers = found_containers;
    }
}