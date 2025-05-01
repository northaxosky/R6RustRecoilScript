use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Represents a Rainbow Six Siege operator with recoil strengths
pub struct Operator {
    pub name: String,
    pub default_strength: i32,
    pub current_strength: i32,
}

impl Operator {
    // Reset current_strength back to default_strength
    pub fn reset(&mut self) {
        self.current_strength = self.default_strength;
    }
}

// Load operators from a JSON file at the given path
pub fn load_operators(path: &str) -> io::Result<Vec<Operator>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let operators: Vec<Operator> = serde_json::from_str(&content)?;
    Ok(operators)
}

/// Save operators to a JSON file at the given path
pub fn save_operators(path: &str, operators: &[Operator]) -> io::Result<()> {
    let content = serde_json::to_string_pretty(operators)?;
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}