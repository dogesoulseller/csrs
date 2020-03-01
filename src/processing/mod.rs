mod defs;
mod track_process;
mod util;

use defs::*;
use track_process::*;
use util::*;

fn get_file_value(lines : & Vec<String>, f : Field) -> Option<String> {
	match f {
		Field::ARTIST | Field::TITLE => {
			for l in lines {
				if l.contains("FILE") { // If reached file, it's too far
					break;
				} else if l.contains(f.to_search_pat()) {
					return get_quoted_val(&l).or_else(|| get_unquoted_val(f, l));
				}
			}

			return None;
		}

		Field::FILENAME => {
			unsafe {

			for l in lines {
				if l.contains("FILE") {
					let line_nows = l.trim_end().trim_start().to_owned();

					if l.ends_with("WAVE") || l.ends_with("AIFF") {
						let line_trimmed = line_nows.get_unchecked(0..line_nows.len()-4).trim_end().trim_start();

						return get_quoted_val(&line_trimmed).or_else(|| get_unquoted_val(Field::FILENAME, &line_trimmed));
					} else if l.ends_with("MP3") {
						let line_trimmed = line_nows.get_unchecked(0..line_nows.len()-3).trim_end().trim_start();

						return get_quoted_val(&line_trimmed).or_else(|| get_unquoted_val(Field::FILENAME, &line_trimmed));
					} else {
						panic!("Invalid or unsupported FILE type");
					}
				}
			}

			return None;

			}
		}

		_ => None
	}
}

pub fn process_cuesheet(path : &std::string::String) -> Result<CuesheetInfo, String> {
	let contents = match std::fs::read_to_string(path) {
		Ok(x) => x,
		Err(e) => return Err("Failed to read contents ".to_owned() + &e.to_string())
	};

	let mut out = CuesheetInfo::new();

	let mut file_lines : Vec<String> = Vec::new();
	for l in contents.lines() {
		file_lines.push(l.to_owned());
	}

	out.file_name = match get_file_value(&file_lines, Field::FILENAME) {
		Some(x) => x,
		None => return Err("Failed to get file value".to_owned())
	};

	out.artist = get_file_value(&file_lines, Field::ARTIST).unwrap_or_else(|| String::new());
	out.album_name = get_file_value(&file_lines, Field::TITLE).unwrap_or_else(|| String::new());

	// Find first TRACK
	let mut first_track : usize = 0;
	for l in &file_lines {
		if l.contains("TRACK") {
			break;
		} else {
			first_track += 1;
		}
	}

	if first_track == file_lines.len() {
		return Err("Failed to find a track".to_owned())
	}

	let mut file_track_lines : Vec<String> = Vec::new();

	unsafe {
		for l in file_lines.get_unchecked(first_track..) {
			file_track_lines.push(l.to_owned());
		}
	}

	out.tracks = process_tracks(&mut file_track_lines, &out.artist);

	Ok(out)
}