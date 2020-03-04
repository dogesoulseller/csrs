use std::io;

mod processing;
mod conversion;

fn main() -> io::Result<()> {
    let mut arguments = std::env::args();

    let file_path = match arguments.nth(1) {
        Some(x) => x,
        None => std::string::String::new(),
    };

    let base_path = match std::path::Path::new(&file_path).parent() {
        Some(x) => x,
        None => panic!("Path had no base component"),
    };

    let cuesheet_info = match processing::process_cuesheet(&file_path) {
        Ok(x) => x,
        Err(e) => panic!(e),
    };

    // For the testing phase, just print out the information
    println!("{}", cuesheet_info);

    for track in &cuesheet_info.tracks {
        println!("{}", track);
    }

    conversion::split_using_cuesheet(&cuesheet_info, &base_path);

    Ok(())
}
