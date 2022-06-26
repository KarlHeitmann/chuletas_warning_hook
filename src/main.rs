
mod io;

fn loop_files(files:Vec<&str>) {
    println!("{:?}", files);
    for file in files {
        println!("file: {}|||", file);
        let (_, extension) = file.rsplit_once('.').unwrap();
        if extension != "rb" {
            continue
        }
        println!("analyzing...");
        let diff_data = io::get_diff_file(file);
        println!("{}", diff_data);
    }
}

fn main() {
    let s = io::get_diff_files();
    loop_files(s.split("\n").collect::<Vec<&str>>());
}
