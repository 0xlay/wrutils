mod file_obj;
use crate::file_obj::find_all_file;

fn main() {
    let file_objects = find_all_file("*.*".to_string()).expect("File not found!");
    for file in file_objects {
        println!("{} {} {}KB {}",
            file.attributes(), file.name(), file.size() / 1024, file.creation_time()
        );
    }
}
