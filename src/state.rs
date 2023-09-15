use std::fs;
use std::process::Command;
use crate::state::Status::EXITED;

pub enum CurrentScreen {
    Main,
    Detail,
    Log
}

pub enum Status {
    UP(usize),
    EXITED(usize),
    ERROR(String)
}

pub struct Container {
    pub name: String,
    pub cpu: f32,
    pub mem: f32,
    pub status: Status
}

pub struct Sebulba {
    pub selected_container: String,
    pub all_containers: Vec<Container>,
    pub current_screen: CurrentScreen
}

impl Sebulba {
    pub fn new() -> Sebulba {
        let mut sebulba = Sebulba {
            selected_container: "".to_string(),
            all_containers: vec![],
            current_screen: CurrentScreen::Main,
        };

        sebulba.list_files();
        sebulba
    }

    pub fn docker_ps_a(&mut self) {
        let output = Command::new("docker")
            .arg("ps")
            .arg("-a")
            .output()
            .expect("Failed to execute command");

        // Check if the command was successful
        if output.status.success() {
            // Convert the output bytes to a string
            let output_str = String::from_utf8_lossy(&output.stdout);
            self.selected_container = output_str.parse().unwrap();
        } else {
            // Handle the case where the command failed
            let error_str = String::from_utf8_lossy(&output.stderr);
            self.selected_container = error_str.parse().unwrap();
        }
    }

    pub fn list_files(&mut self) {
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
                        });
                    }
                }
            }
        });
        self.all_containers = found_containers;

    }
}