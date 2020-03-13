use std::io;

mod processing;
mod conversion;
mod configuration;

fn main() -> io::Result<()> {
    let config = configuration::parse_args();

    // Show help message and exit
    if config.display_help {
        configuration::print_help_message();
        return Ok(());
    }

    if !std::path::Path::new(&config.cuesheet_path).exists() {
        eprintln!("Cue file '{}' does not exist", config.cuesheet_path);
        configuration::print_help_message();
        return Ok(());
    }

    let base_path = match std::path::Path::new(&config.cuesheet_path).parent() {
        Some(x) => x,
        None => panic!("Path had no base component"),
    };

    let cuesheet_info = match processing::process_cuesheet(&config.cuesheet_path) {
        Ok(x) => x,
        Err(e) => panic!(e),
    };

    // For the testing phase, print out the information
    println!("{}", cuesheet_info);

    for track in &cuesheet_info.tracks {
        println!("{}", track);
    }

    conversion::split_using_cuesheet(&cuesheet_info, &base_path);

    Ok(())
}
