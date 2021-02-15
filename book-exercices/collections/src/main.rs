use std::io::stdin;

mod exercises;

fn main() {
    let cstdin = stdin();
    loop {
        print_help();
        let mut buffer = String::new();
        cstdin
            .read_line(&mut buffer)
            .expect("Failed to read terminal input!");

        let trimmed = buffer.trim();

        match trimmed {
            "1" => exercises::array_info::read_and_process(),
            "2" => exercises::pig_latin::read_and_process(),
            "5" => break,
            _ => print_help(),
        }
        print!("\n\n\n");
    }
    println!("Exiting");
}

fn print_help() {
    println!("Please enter a valid exercise number:");
    println!("1 - Array Info");
    println!("2 - Pig latin");
    println!("5 - Exit");
}
