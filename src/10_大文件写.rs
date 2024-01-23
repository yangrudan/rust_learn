use std::env::args;
use std::fs::File;
use std::io::Write;

fn main() {
    let args = args().collect::<Vec<String>>();
    let n = args[1].parse::<usize>().unwrap();
    println!("将要写入{}行", n);

    let mut file = File::create("data/a.txt").unwrap();
    for line in Lines::new().take(n){
        println!("{line}");
        file.write_all(line.as_bytes()).unwrap();
    }
}

struct Lines(usize);

impl Lines {
    fn new() -> Lines{
        Lines(0)
    }
}

impl Iterator for Lines{
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let n:usize = 10000000000 + self.0;
        self.0 += 1;
        let line = format!("{}\n", n);
        Some(line)
    }
}

// cargo run 6 写入6行