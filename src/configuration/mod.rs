pub struct ConfigParams {
	pub display_help : bool,
    pub output_directory : String,
	pub cuesheet_path : String
}

impl ConfigParams {
	pub fn new() -> ConfigParams {
		ConfigParams{display_help: false, output_directory: String::new(), cuesheet_path: String::new()}
	}
}

pub fn print_help_message() {
	eprintln!(
r#"Usage: csrs [options] cue_file
Options (no =, space-separated):
	-h | --help       - show this message
	-o | --output-dir - set output directory

Examples:
	csrs -o /home/user/files xyz.cue
	csrs --output-dir /home/user/files xyz.cue
"#);
}

pub fn parse_args() -> ConfigParams {
	let args : Vec<String> = std::env::args().collect();
	let mut out = ConfigParams::new();

	if args.len() < 2 {
		panic!("Not enough arguments passed");
	}

	// Find help message
	for arg in &args {
		if arg == "-h" || arg == "--help" {
			out.display_help = true;
			break;
		}
	}

	let mut outdir_position : usize = 0;
	for arg in &args {
		if arg == "-o" || arg == "--output-dir" {
			outdir_position += 1;
			break;
		}

		outdir_position += 1;
	}

	out.output_directory = match args.get(outdir_position) {
		Some(x) => x.clone(),
		None => String::new()
	};

	out.cuesheet_path = match args.last() {
		Some(x) => x.clone(),
		None => unreachable!()
	};

	return out;
}
