use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::terminal_util::{resume_ui, suspend_ui};

#[derive(Serialize, Deserialize)]
pub enum Screen {
    Menu,
    TodayEntry,
    ViewLog,
    HabitSettings,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Habit {
    pub name: String,
    pub threshold: f64,
    pub direction: Direction,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Direction {
    MoreIsBetter,
    LessIsBetter,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub progress: f64,
    pub screen: Screen,
    pub habits: Vec<Habit>,
    pub logs: HashMap<String, HashMap<String, f64>>,

    // hidden from JSON
    #[serde(skip)]
    data_file: PathBuf,
}

impl State {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir().expect("No data dir");
        let app_dir = data_dir.join("hbt");
        fs::create_dir_all(&app_dir).unwrap();

        let data_file = app_dir.join("data.json");

        if let Ok(contents) = fs::read_to_string(&data_file) {
            if let Ok(mut stored) = serde_json::from_str::<State>(&contents) {
                stored.data_file = data_file.clone();
                return stored;
            }
        }

        Self {
            progress: 0.0,
            screen: Screen::Menu,
            habits: Vec::new(),
            logs: HashMap::new(),
            data_file,
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(&self.data_file, json).unwrap();
    }

    pub fn enter_today_values(&mut self) {
        use chrono::Local;
        use std::io::{Write, stdin, stdout};

        suspend_ui().unwrap();

        let today = Local::now().format("%Y-%m-%d").to_string();
        let entry = self.logs.entry(today).or_default();

        for habit in &self.habits {
            print!("Value for {}: ", habit.name);
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if let Ok(val) = input.trim().parse::<f64>() {
                entry.insert(habit.name.clone(), val);
            }
        }

        resume_ui().unwrap();

        self.screen = Screen::Menu;
    }

    pub fn add_habit(&mut self) {
        use std::io::{Write, stdin, stdout};

        suspend_ui().unwrap();

        print!("Habit name: ");
        stdout().flush().unwrap();
        let mut name = String::new();
        stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        print!("Threshold: ");
        stdout().flush().unwrap();
        let mut threshold = String::new();
        stdin().read_line(&mut threshold).unwrap();
        let threshold = threshold.trim().parse::<f64>().unwrap();

        print!("Direction (more/less): ");
        stdout().flush().unwrap();
        let mut dir = String::new();
        stdin().read_line(&mut dir).unwrap();
        let direction = match dir.trim().to_lowercase().as_str() {
            "more" => Direction::MoreIsBetter,
            _ => Direction::LessIsBetter,
        };

        resume_ui().unwrap();

        self.habits.push(Habit {
            name,
            threshold,
            direction,
        });

        self.screen = Screen::Menu;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
