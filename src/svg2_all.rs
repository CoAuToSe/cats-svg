use regex::*;

fn main() {
    // let args: Vec<String> = std::env::args().collect();

    // let query = &args[1];
    // let file_path = "";

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    let file_content = {
        let _file_path = std::path::PathBuf::from("src/svg2_all.csv");
        match std::fs::read_to_string(_file_path.clone()) {
            Ok(_file_content) => _file_content,
            Err(_error) => {
                eprintln!(
                    "Unable to read {:?}: {_error} | Defaulting to String::from(r\"\")",
                    match _file_path.canonicalize() {
                        Ok(_path) => _path,
                        Err(_) => _file_path,
                    }
                );
                String::from(r"")
            }
        }
    };

    let mut list_tag = vec![];
    for tag in Regex::new("<(?P<name>[^>]*)>")
        .unwrap()
        .captures_iter(&file_content)
    {
        // println!("{:?}", tag);
        list_tag.push(tag.name("name").unwrap().as_str())
    }

    println!("{:?}", list_tag);

    let file_content_deprecated = {
        let _file_path = std::path::PathBuf::from("src/svg4_deprecated.csv");
        match std::fs::read_to_string(_file_path.clone()) {
            Ok(_file_content) => _file_content,
            Err(_error) => {
                eprintln!(
                    "Unable to read {:?}: {_error} | Defaulting to String::from(r\"\")",
                    match _file_path.canonicalize() {
                        Ok(_path) => _path,
                        Err(_) => _file_path,
                    }
                );
                String::from(r"")
            }
        }
    };

    let mut list_tag_deprecated = vec![];
    for tag in Regex::new("<(?P<name>[^>]*)>")
        .unwrap()
        .captures_iter(&file_content_deprecated)
    {
        // println!("{:?}", tag);
        list_tag_deprecated.push(tag.name("name").unwrap().as_str())
    }

    println!("{:?}", list_tag_deprecated);

    let file_content_cat = {
        let _file_path = std::path::PathBuf::from("src/svg3_all_cat.csv");
        match std::fs::read_to_string(_file_path.clone()) {
            Ok(_file_content) => _file_content,
            Err(_error) => {
                eprintln!(
                    "Unable to read {:?}: {_error} | Defaulting to String::from(r\"\")",
                    match _file_path.canonicalize() {
                        Ok(_path) => _path,
                        Err(_) => _file_path,
                    }
                );
                String::from(r"")
            }
        }
    };

    let mut list_tag_cat_clean = vec![];
    for e in file_content_cat.split("\n") {
        let mut temp = vec![];

        let mut current_cat = "";
        for cat_name in Regex::new("^(?P<name_cat>[^,]*)")
            .unwrap()
            .captures_iter(&e)
        {
            current_cat = cat_name.name("name_cat").unwrap().as_str();
        }
        for tag in Regex::new("<(?P<name>[^>]*)>").unwrap().captures_iter(&e) {
            // println!("{:?}", tag);
            let tag_name = tag.name("name").unwrap().as_str();
            if !list_tag_deprecated.contains(&tag_name) {
                temp.push(tag_name)
            }
        }
        list_tag_cat_clean.push((current_cat, temp))
    }

    println!("{:#?}", list_tag_cat_clean);

    let mut list_tag_cat = vec![];
    for e in file_content_cat.split("\n") {
        let mut temp = vec![];

        let mut current_cat = "";
        for cat_name in Regex::new("^(?P<name_cat>[^,]*)")
            .unwrap()
            .captures_iter(&e)
        {
            current_cat = cat_name.name("name_cat").unwrap().as_str();
        }
        for tag in Regex::new("<(?P<name>[^>]*)>").unwrap().captures_iter(&e) {
            // println!("{:?}", tag);
            temp.push(tag.name("name").unwrap().as_str())
        }
        list_tag_cat.push((current_cat, temp))
    }

    println!("{:#?}", list_tag_cat);
}
