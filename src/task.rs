use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(title: &str, description: &str) -> Self {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        }
    }
}
