use super::defs::*;
use super::util::*;

pub fn get_track_index(lines : &Vec<String>) -> usize {
	for l in lines {
		if l.contains("TRACK") {
			let mut split = l.trim_end().trim_start().split_whitespace();

			return split.nth(1).or_else(|| panic!("Track line had no index value")).unwrap().parse::<usize>().unwrap();
		}
	}

	unreachable!()
}

pub fn get_track_value(lines : &Vec<String>, f : Field) -> Option<String> {
	match f {
		Field::ARTIST | Field::START | Field::PREGAP | Field::TITLE => {
			for l in lines {
				if l.contains(f.to_search_pat()) {
					return get_quoted_val(&l).or_else(|| get_unquoted_val(f, l));
				}
			}

			return None
		}

		_ => None
	}

}

pub fn process_tracks(lines : &mut Vec<String>, file_artist : &String) -> Vec<CuesheetTrack> {
	let mut out_vec : Vec<CuesheetTrack> = Vec::new();
	let mut line_sets : Vec<Vec<String>> = Vec::new();
	let mut tmp_vec : Vec<String> = Vec::new();

	// Make sets of track lines
	for l in lines {
		if l.contains("TRACK") { // Track start
			line_sets.push(tmp_vec);
			tmp_vec = Vec::new();
			tmp_vec.push(l.to_owned());
		} else {
			tmp_vec.push(l.to_owned());
		}
	}

	line_sets.push(tmp_vec);

	// Parse track sets
	for ls in unsafe {line_sets.get_unchecked(1..)} {
		let mut track = CuesheetTrack::new();
		track.track_start = get_track_value(&ls, Field::START).or_else(|| panic!("Track has no start time")).unwrap();
		track.idx = get_track_index(&ls);

		track.artist = get_track_value(&ls, Field::ARTIST).unwrap_or(file_artist.clone());
		track.pregap_start = get_track_value(&ls, Field::PREGAP).unwrap_or_default();
		track.track_name = get_track_value(&ls, Field::TITLE).unwrap_or_default();

		out_vec.push(track);
	}

	// Get end timestamps
	for track_no in 0..out_vec.len()-1 {
		unsafe {
			out_vec.get_unchecked_mut(track_no).track_end = match out_vec.get(track_no+1) {
				Some(x) => x.track_start.clone(),
				None => String::new()
			};
		}
	}

	return out_vec;
}
