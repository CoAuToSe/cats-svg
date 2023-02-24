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
    // dbg!(args);
    let name_file = format!("{}", args[1]);
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_path(format!("./src/{}.csv", name_file)).unwrap();
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
    // println!("{list_name_args:?}");
    let mut list_balises = vec![];
    for e in list_name_args {
        if e.0.starts_with("<") {
            let mut my = String::from(e.0.clone().trim());
            my.remove(0);
            my.pop();
            my = my
                .split("-")
                .map(|a| uppercase_first(a))
                .collect::<Vec<String>>()
                .join("");
            list_balises.push(my);
        }
    }
    println!("{:?}", list_balises);
    {
        let string_path = format!("./src/{}.rs", name_file);
        let path = std::path::Path::new(&string_path);
        let display = path.display();
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match std::io::Write::write_all(
            &mut file,
            format!(
                "enum Balise{} {{\n\t{},\n\tOther\n}}",
                name_file.to_uppercase(),
                list_balises.join(",\n\t")
            )
            .as_bytes(),
        ) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

fn uppercase_first(data: &str) -> String {
    // Uppercase first letter.
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
        }
    }
    result
}
