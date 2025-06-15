use serde::{Serialize, Deserialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub player_name: String,
    pub rank: String,
    pub reputation: i32,
    pub ship_name: String,
    pub location: String,
    pub hull_status: i32,
    pub shield_status: i32,
    pub mission_log: Vec<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_name: "Unnamed Officer".to_string(),
            rank: "Lieutenant".to_string(),
            reputation: 0,
            ship_name: "Imperial Star Destroyer".to_string(),
            location: "Deep Space".to_string(),
            hull_status: 100,
            shield_status: 100,
            mission_log: vec!["Your mission begins.".to_string()],
        }
    }
}

impl GameState {
    pub fn load(path: &Path) -> std::io::Result<Self> {
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let state = serde_json::from_str(&data)?;
            Ok(state)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)
    }
}
