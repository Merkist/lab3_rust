use crate::task::Task;
use std::fs;
use std::io::{self, Write};
use rfd::FileDialog;

const FILE_NAME: &str = "data.json";

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let json = serde_json::to_string(tasks)?;
    let mut file = fs::File::create(FILE_NAME)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let data = fs::read_to_string(FILE_NAME).unwrap_or_else(|_| "[]".to_string());
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save_tasks_to(tasks: &Vec<Task>) -> io::Result<()> {
    let json = serde_json::to_string(tasks)?;

    if let Some(path) = FileDialog::new()
        .add_filter("JSON files", &["json"])
        .set_file_name("tasks.json")
        .save_file()
    {
        let mut file = fs::File::create(&path)?;
        file.write_all(json.as_bytes())?;
    }
    Ok(())
}

pub fn load_tasks_from() -> io::Result<Vec<Task>> {
    if let Some(file) = FileDialog::new().pick_file() {
        let data = fs::read_to_string(file.display().to_string()).unwrap_or_else(|_| "[]".to_string());
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    }
    else {
        Ok(Vec::new())
    }
}



