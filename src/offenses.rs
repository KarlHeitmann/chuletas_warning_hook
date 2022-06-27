use std::collections::HashMap;
use std::fmt;

pub struct Offenses {
    pub data: HashMap<String, i32>
}

impl Offenses {
    pub fn add_offense(& mut self, offense: &str) {
        if self.data.contains_key(&offense.to_string()) {
            self.data.insert(offense.to_string(), self.data[&offense.to_string()] + 1);
        } else {
            self.data.insert(offense.to_string(), 1);
        }
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl fmt::Display for Offenses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cadena = format!("");
        for (offense, times) in &self.data {
            cadena = format!("{}  {} - {}\n", cadena, offense, times)
        }
        write!(f, "{}", cadena)
    }
}


