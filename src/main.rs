use std::env;

fn print_file_details(filepath: String) {
    println!("Filepath: '{}'", filepath);
}

fn main() {
    if env::args().len() == 1 {
        println!("Too few args!");
        return;
    }

    for arg in env::args().skip(1) {
        print_file_details(arg);
    }
}
