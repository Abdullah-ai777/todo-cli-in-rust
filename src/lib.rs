use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use uuid::Uuid;

pub fn time() -> String {
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d").to_string();
    formatted_time
}
pub fn input() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("please enter right input");
    s
}

pub fn uuid() -> String {
    let id = Uuid::new_v4();
    id.to_string()
}

pub fn clear_screen() {
    // Use OS-specific commands to clear the screen
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub text: String,
    pub date: String,
}

pub fn save_to_json(data: &Vec<Task>) {
    let file = OpenOptions::new()
        .create(true) // Create file if it doesn't exist
        .write(true) // Write to file
        .truncate(true) // Clear file before writing
        .open("tasks.json")
        .unwrap();

    serde_json::to_writer(file, &data).unwrap();
}

pub fn load_from_json() -> Vec<Task> {
    let file = std::fs::File::open("tasks.json");
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap()
        }
        Err(_) => Vec::new(), // If file doesn't exist, return empty vector
    }
}
