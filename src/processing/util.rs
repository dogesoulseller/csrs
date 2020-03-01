use super::defs::Field;

pub fn get_quoted_val(in_s: &str) -> Option<String> {
	let mut s : String = in_s.trim_start().trim_end().to_owned();

	let first = match s.find('"') {
		Some(x) => x,
		None => return None
	};

	let last = match s.rfind('"') {
		Some(x) => x,
		None => unreachable!()
	};

	if first == last {
		return None
	}

	unsafe {
		Some(s.get_unchecked_mut(first+1..last).to_owned())
	}
}

// Does not have to worry about FILE field types
pub fn get_unquoted_val(f : Field, in_s: &str) -> Option<String> {
	unsafe {
		Some(in_s.trim_end().trim_start().get_unchecked(f.to_search_pat().len() + 1..).to_owned())
	}
}