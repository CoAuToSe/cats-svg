use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(args);
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_path("./src/aze_r.csv").unwrap();
    // Loop over each record.

    let mut list_name_args = vec![];
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.

        // println!("{:?}", record);
        list_name_args.push((String::from(&record[0]), String::from(&record[2])))
        // for e in &record {

        //     println!("{:?}", e)
        // }
    }
    println!("{list_name_args:?}")
}
