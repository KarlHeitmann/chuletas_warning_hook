use std::collections::HashMap;

mod io;

fn analyze(diffs:Vec<&str>) -> HashMap<String, i32> {
//fn analyze(diffs:Vec<&str>) -> HashMap<&&str, i32> {
    let mut offenses = HashMap::new();
    let offensive_words = vec!["XXX_ME", "HACK_ME", "puts", "binding.pry"];

    /*
    offenses.insert(String::from("Blue"), 10);
    offenses.insert(String::from("Yellow"), 50);
    */

    /*
    println!("{:?}", offenses);
    println!("{:?}", diffs);
    println!("{}", diffs[1]);
    */
    let (possible_new, _) = diffs[1].split_once(' ').unwrap();

    let start_point = if possible_new == "new" {
                          6
                      } else {
                          5
                      };
    let diffs_text = &diffs[start_point..];
    //println!("XXX{:?}XXX", diffs_text);
    for diff in diffs_text {
        // println!("{}", diff);
        if diff.chars().nth(0).unwrap() != '+' {
            println!("skipping because the diff was not added");
            continue;
        }
        for offense in &offensive_words {
            if diff.contains(offense) {
                /*
                if offenses.contains_key(offense) {
                    offenses.insert(offense, offenses[offense] + 1);
                } else {
                    offenses.insert(offense, 1);
                }
                */
                if offenses.contains_key(&offense.to_string()) {
                    offenses.insert(offense.to_string(), offenses[&offense.to_string()] + 1);
                } else {
                    offenses.insert(offense.to_string(), 1);
                }
            }
        }
    }
    // println!("{:?}", offenses);
    offenses
}

fn loop_files(files:Vec<&str>) -> HashMap<&str, HashMap<String, i32>> {
//fn loop_files(files:Vec<&str>) -> HashMap<&str, HashMap<&&str, i32>> {
    //println!("//// files array: {:?}", files);
    let mut results = HashMap::new();
    for file in files {
        /*
        print!("WAAAAAAAAAAAAAAA {file} !!!!!!!!!!!!!!!");
        println!("file: {}|||", file);
         */
        let (_, extension) = file.rsplit_once('.').unwrap();
        if extension != "rb" {
            println!("Continue because {} it is not a ruby file", file);
            continue;
        }
        let diff_data = io::get_diff_file(file);
        //println!("\\\\\\\\\\ ANALYZING file: {} DIFF DATA: {}\\\\\\\\", file, diff_data);

        let r = analyze(diff_data.split("\n").collect::<Vec<&str>>());
        results.insert(file, r);
    }
    results
}

fn show_offenses(results: HashMap<&str, HashMap<String, i32>>) {
    if results.is_empty() { return ; }
    for (file, offenses) in &results {
        /*
        println!("{} - {:?}", file, offenses);
        for (offense, times) in offenses {
            println!("  {} - {}", offense, times);
        }
        */
        println!("{}", file);
        for (offense, times) in offenses {
            println!("  {} - {}", offense, times);
        }

    }
}

fn main() {
    let s = io::get_diff_files();
    //println!("FILES: {}", s);
    let res = loop_files(s.split("\n").collect::<Vec<&str>>());
    show_offenses(res);
}












