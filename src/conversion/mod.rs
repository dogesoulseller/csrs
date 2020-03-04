use super::processing::defs::*;
use std;

const FRAME_TO_MS : f64 = 75.0/1000.0;

fn cue_stamp_to_ffmpeg_stamp(stamp: &str) -> String {
	if stamp == "" {
		return "".to_owned();
	}

	let components : Vec<&str> = stamp.split(":").collect();
	assert_eq!(components.len(), 3);

	let _min = unsafe{i32::from_str_radix(components.get_unchecked(0), 10).unwrap()};

	let sec = unsafe{i32::from_str_radix(components.get_unchecked(1), 10).unwrap()};
	let msec = (unsafe{i32::from_str_radix(components.get_unchecked(2), 10).unwrap() as f64} * FRAME_TO_MS) as i32;

	if _min <= 60 {
		return format!("0:{:02}:{:02}.{:03}", _min, sec, msec);
	} else {
		let hours = _min / 60;
		let min = _min % 60;
		return format!("{:01}:{:02}:{:02}.{:03}", hours, min, sec, msec);
	};
}

fn find_ffmpeg() -> Option<std::path::PathBuf> {
	let ffmpeg_path_name = if cfg!(target_os="windows") {
		std::path::Path::new("./ffmpeg.exe")
	} else {
		std::path::Path::new("./ffmpeg")
	};

	let ffmpeg_executable = match std::env::current_exe() {
		Ok(x) => match x.parent() {
			Some(x) => x.join(ffmpeg_path_name),
			None => return None
		},
		Err(_) => return None
	};

	// TODO: Search for ffmpeg in system path

	return Some(ffmpeg_executable)
}

pub fn split_using_cuesheet(file: &CuesheetInfo, cue_path_base: &std::path::Path) {
	let input_file = cue_path_base.join(&file.file_name);
	let ffmpeg_executable = find_ffmpeg().or_else(|| panic!("Could not find ffmpeg executable")).unwrap();

	let file_extension = std::path::Path::new(&file.file_name).extension().unwrap().to_str().unwrap();

	if !input_file.exists() {
		panic!("Audio file specified in cuesheet does not exist");
	}

	if !ffmpeg_executable.exists() {
		panic!("Could not find ffmpeg executable");
	}

	// TODO: Map extension to format
	// TODO: Allow setting maximum thread count

	let mut to_execute : Vec<(std::process::Command, Vec<String>)> = Vec::with_capacity(file.tracks.len());
	for track in &file.tracks {
		let start_time = cue_stamp_to_ffmpeg_stamp(&track.track_start);
		let end_time = cue_stamp_to_ffmpeg_stamp(&track.track_end);

		let output_file = if &track.artist != "" && &track.track_name != "" {
			cue_path_base.join(format!("{:02}. {} - {}.{}", &track.idx, &track.artist, &track.track_name, file_extension))
		} else if &track.artist == "" && &track.track_name != "" {
			cue_path_base.join(format!("{:02}. {}.{}", &track.idx, &track.track_name, file_extension))
		} else {
			cue_path_base.join(format!("{:02}.{}", &track.idx, file_extension))
		};

		let ffmpeg_args = if end_time.as_str() == "" {
			vec!(String::from("-loglevel"), String::from("quiet"), String::from("-hide_banner"), String::from("-nostats"),
				String::from("-i"), input_file.to_str().unwrap().to_owned(),
				String::from("-ss"), start_time,
				String::from("-c:a"), String::from("flac"), String::from("-c:v"), String::from("copy"),
				String::from("-y"),
				output_file.to_str().unwrap().to_owned())
		} else {
			vec!(String::from("-loglevel"), String::from("quiet"), String::from("-hide_banner"), String::from("-nostats"),
				String::from("-i"), input_file.to_str().unwrap().to_owned(),
				String::from("-ss"), start_time, String::from("-to"), end_time,
				String::from("-c:a"), String::from("flac"), String::from("-c:v"), String::from("copy"),
				String::from("-y"),
				output_file.to_str().unwrap().to_owned())
		};

		to_execute.push((std::process::Command::new(&ffmpeg_executable), ffmpeg_args));
	}

	let mut threads : Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(file.tracks.len());
	for mut exec in to_execute {
		threads.push(std::thread::spawn(|| {
			if let Ok(_) = exec.0.args(exec.1).spawn() {
			} else {
				panic!("Failed to spawn processing thread");
			}

			return;
		}));
	}

	// Wait for processing to finish
	for t in threads {
		if let Ok(_) = t.join() {
		} else {
			panic!("Failed to join thread");
		}
	}
}
