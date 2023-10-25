use std::fs::File;
use std::path::Path;
use std::io::Read;

mod class_todo;
use self::class_todo::Todo;
mod class_cursor;
use self::class_cursor::Cursor;

const FILENAME: &str = "todo.txt";

fn read_file(file_name: &str) -> String {
    match Path::new(file_name).exists() {
        true => {
            let mut contents = String::new();
            File::open(file_name)
                .expect("Should have been able to open the file")
                .read_to_string(&mut contents)
                .expect("Should have been able to read the file");
            contents
        }
        false => {
            File::create(file_name).expect("Should have been able to create the file");
            String::new()
        }
    }
}

fn validate_file(raw_data: String) -> Vec<Todo> {
    if raw_data.len() == 0 {
        return Vec::new();
    }
    let mut usable_data: Vec<Todo> = Vec::new();
    for line in raw_data.split("\n") {
        usable_data.push(if line.len() == 0 {Todo::new("")} else {Todo::new(&line)});
    }
    return usable_data
}

fn init() {}

fn main() {
    init();
    let mut todos = validate_file(read_file(FILENAME));
    let mut selected: Cursor = Cursor::new(0);
    for i in 0..todos.len() {
        println!("{}", todos[i]);
    }
}
