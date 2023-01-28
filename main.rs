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
    
    let mut count = 0;
    loop{
        let mut line = String::new();

        let res = reader.read_line(&mut line).unwrap();
        if res == 0 {break}

        let col_count = line.matches(",").count() + 1;

        let csv_line = line.replace(",", "|").replace("\n", "|\n");
        print!("|{}", csv_line);

        //headings
        if count == 0{
            for _ in 0..col_count{
                print!("|-");
            }
            print!("|\n");
        }
        

        count += 1;
    }
}
