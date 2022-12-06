fn main() {
    let file_content = read_file(String::from(""));

    match &file_content {
        Some(value) => println!("{}", value),
        None => println!("no content"),
    }

    if let Some(value) = &file_content {
        println!("{}", value);
    }

    println!("{:?}", file_content);
}

fn read_file(path: String) -> Option<String> {
    if !path.is_empty() {
        Some(String::from("file content"))
    } else {
        None
    }
}
