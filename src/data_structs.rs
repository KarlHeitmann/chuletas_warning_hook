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

pub struct Results {
    // data: Option<HashMap<String, Offenses>>
    pub data: HashMap<String, Offenses> // TODO: Review a way to construct that but with a
                                        // constructor, something like that
}

impl Results {
    pub fn is_empty(&self) -> bool {
        // self.data.unwrap().is_empty()
        self.data.is_empty()
    }
    pub fn add_offense(& mut self, file: &str, offense: Offenses) {
        self.data.insert(file.to_string(), offense);
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cadena = format!("OFFENSES FOUND:\n");
        for (file, offense) in &self.data {
            cadena = format!("{}{}\n{}\n", cadena, file, offense)
        }
        write!(f, "{}", cadena)
    }
}


