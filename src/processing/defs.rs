use std::fmt;

pub struct CuesheetTrack {
	pub idx : usize,
	pub track_name : String,
	pub artist : String,
	pub pregap_start : String,
	pub track_start : String,
	pub track_end : String
}

pub struct CuesheetInfo {
	pub file_name : String,
	pub artist : String,
	pub album_name : String,
	pub tracks : Vec<CuesheetTrack>
}

impl CuesheetTrack {
	pub fn new() -> CuesheetTrack {
		CuesheetTrack { idx: 0,
			track_name: std::string::String::new(), artist: std::string::String::new(),
			pregap_start: std::string::String::new(),
			track_start: std::string::String::new(), track_end: std::string::String::new()
		}
	}
}

impl fmt::Display for CuesheetTrack {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "TRACK {} - {} by {}\n\tPregap: {}\n\tStart: {}",
			self.idx, self.track_name, self.artist, self.pregap_start, self.track_start)
	}
}

impl CuesheetInfo {
	pub fn new() -> CuesheetInfo {
		CuesheetInfo {file_name: std::string::String::new(), artist: std::string::String::new(), album_name: std::string::String::new(), tracks: Vec::new() }
	}
}

impl fmt::Display for CuesheetInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Cuesheet at {}\n{} by {}\nContains {} tracks", self.file_name, self.album_name, self.artist, self.tracks.len())
	}
}

pub enum Field {
	ARTIST,
	TITLE,
	FILENAME,
	PREGAP,
	START,
}

impl Field {
	pub fn to_search_pat(&self) -> &'static str {
		match self {
			Field::ARTIST => "PERFORMER",
			Field::TITLE => "TITLE",
			Field::PREGAP => "INDEX 00",
			Field::START => "INDEX 01",
			Field::FILENAME => "FILE"
		}
	}
}