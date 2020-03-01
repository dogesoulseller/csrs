use std::io;

mod processing;

fn main() -> io::Result<()> {
    let mut arguments = std::env::args();

    let file_path = match arguments.nth(1) {
        Some(x) => x,
        None => std::string::String::new(),
    };

    let cuesheet_info = match processing::process_cuesheet(&file_path) {
        Ok(x) => x,
        Err(e) => panic!(e),
    };

    // For the testing phase, just print out the information
    println!("{}", cuesheet_info);
    Ok(())
}
