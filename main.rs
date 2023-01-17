use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please Provide A File To Convert");
        return;
    }
    let input_file = &args[1];

    let fpi = File::open(input_file).expect("File Not Found.");

    let mut reader = io::BufReader::new(fpi);
    
    let mut headings = String::new();
    reader.read_line(&mut headings).unwrap();

    //headings
    let mut num_headings = 0;
    print!("|");
    for heading in headings.split(",").into_iter(){
        let heading_printable: Vec<&str> = heading.split_terminator("\n").collect();

        print!(" {} |", heading_printable[0]);
        num_headings += 1;
    }
    print!("\n");

    print!("|");
    for _ in 0..num_headings{
        print!("-|");
    }
    print!("\n");

    loop{
        let mut line = String::new();
        let res = reader.read_line(&mut line).unwrap();
        if res == 0 {break}
        print!("|");
        for entry in line.split(",").into_iter(){
            let entry_printable: Vec<&str> = entry.split_terminator("\n").collect();
            print!(" {} |", entry_printable[0]);
        }
        print!("\n");
    }    
}
