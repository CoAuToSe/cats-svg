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
    let name_enum = name_file.to_uppercase();
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
    let mut list_name = vec![];
    for e in list_name_args {
        let mut my = String::from(e.0.clone().trim());
        if e.0.starts_with("<") {
            my.remove(0);
            my.pop();
        } else {
        }
        let mi = my
            .split("-")
            .map(|a| to_enum_name(a))
            .collect::<Vec<String>>()
            .join("");
        list_balises.push((mi.clone(), my));
        list_name.push(mi);
    }
    println!("{:?}", list_balises);

    let mut to_write = format!(
        "#![allow(unused)]\n#[derive(Default, Debug)]\npub enum Balise{name_enum} {{\n\t{},\n\t#[default]\n\tOther\n}}",
        list_name.join(",\n\t")
    );
    to_write.push_str(&format!(
        r"

impl Balise{name_enum} {{
    pub fn name(&self) -> Option<String> {{
        match self {{
"
    ));
    // TODO
    for (name, identifier) in list_balises {
        to_write.push_str(&format!(
            "\t\t\tBalise{name_enum}::{name} => Some(String::from(\"{identifier}\")),\n"
        ));
    }
    to_write.push_str(&format!("\t\t\tBaliseSVG::Other => None"));
    to_write.push_str(
        r"
        }
    }
}
",
    );
    {
        let string_path = format!("./src/{}.rs", name_file);
        let path = std::path::Path::new(&string_path);
        let display = path.display();
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match std::io::Write::write_all(&mut file, to_write.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

// some-string -> SomeString
// some_string -> SomeString
// some:string -> SomeString
// 100 -> N100
fn to_enum_name(name: &str) -> String {
    let mut change_case = false;
    let mut s = String::with_capacity(name.len());
    for (idx, c) in name.chars().enumerate() {
        if idx == 0 {
            if c.is_digit(10) {
                s.push('N');
                s.push(c);
            } else {
                s.push(c.to_uppercase().next().unwrap());
            }

            continue;
        }

        if c == '-' || c == '_' || c == ':' {
            change_case = true;
            continue;
        }

        if change_case {
            s.push(c.to_uppercase().next().unwrap());
            change_case = false;
        } else {
            s.push(c);
        }
    }

    s
}
