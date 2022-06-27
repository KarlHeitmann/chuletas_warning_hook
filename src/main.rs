use std::collections::HashMap;
use std::fmt;

mod io;

struct Offenses {
    data: HashMap<String, i32>
}

struct Results {
    // data: Option<HashMap<String, Offenses>>
    data: HashMap<String, Offenses>
}

impl Results {
    fn is_empty(&self) -> bool {
        // self.data.unwrap().is_empty()
        self.data.is_empty()
    }
    fn add_offense(& mut self, file: &str, offense: Offenses) {
        self.data.insert(file.to_string(), offense);
    }
}

impl Offenses {
    fn add_offense(& mut self, offense: &str) {
        if self.data.contains_key(&offense.to_string()) {
            self.data.insert(offense.to_string(), self.data[&offense.to_string()] + 1);
        } else {
            self.data.insert(offense.to_string(), 1);
        }
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
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

impl fmt::Display for Offenses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cadena = format!("");
        for (offense, times) in &self.data {
            cadena = format!("{}  {} - {}\n", cadena, offense, times)
        }
        write!(f, "{}", cadena)
    }
}

fn analyze(diffs:Vec<&str>) -> Offenses {
//fn analyze(diffs:Vec<&str>) -> HashMap<&&str, i32> {
    let offensive_words = vec!["XXX_ME", "HACK_ME", "puts", "binding.pry"];
    let mut offenses = Offenses { data: HashMap::new() };

    let (possible_new, _) = diffs[1].split_once(' ').unwrap();
    let start_point = if possible_new == "new" {
                          6
                      } else {
                          5
                      };

    let diffs_text = &diffs[start_point..];
    for diff in diffs_text {
        if diff.chars().nth(0).unwrap() != '+' {
            println!("skipping because the diff was not added");
            continue;
        }

        for offense in &offensive_words {
            if diff.contains(offense) {
                offenses.add_offense(offense);
            }
        }
    }

    offenses
}

fn loop_files(files:Vec<&str>) -> Results {
//fn loop_files(files:Vec<&str>) -> HashMap<&str, HashMap<&&str, i32>> {
    let mut results = Results { data: HashMap::new() };
    for file in files {
        let (_, extension) = file.rsplit_once('.').unwrap();
        if extension != "rb" {
            println!("Continue because {} it is not a ruby file", file);
            continue;
        } // use matcher
        let diff_data = io::get_diff_file(file);

        let r = analyze(diff_data.split("\n").collect::<Vec<&str>>());
        if r.is_empty() { continue ; } // use matcher

        results.add_offense(file, r);
    }

    results
}

fn show_offenses(results: Results) {
    if results.is_empty() { return ; } // use matcher
    println!("{}", results);
}

fn main() {
    let s = io::get_diff_files();
    if s == "" { return } // use matcher

    let res = loop_files(s.split("\n").collect::<Vec<&str>>());
    show_offenses(res);
    std::process::exit(1);
}












