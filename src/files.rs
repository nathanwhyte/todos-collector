use dirs::home_dir;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	path::Path,
};

// create a file at 'given_path'
pub fn create_file(given_path: &str) -> File {
	let new_path = build_path(given_path);
	let file_path = Path::new(new_path.as_str());

	return match File::create(&file_path) {
		Ok(file) => file,
		Err(why) => panic!("could not create {}: {}", file_path.display(), why),
	};
}

// open a file at 'given_path'
pub fn open_file(given_path: &str) -> File {
	let new_path = build_path(given_path);
	let file_path = Path::new(new_path.as_str());

	return match File::open(&file_path) {
		Ok(file) => file,
		Err(why) => panic!("could not open {}: {}", file_path.display(), why),
	};
}

// create a path to a file
fn build_path(path: &str) -> String {
	let home_dir = home_dir().unwrap();
	let mut home_dir_string = String::from(home_dir.to_str().unwrap());
	home_dir_string.push('/');

	if path.starts_with("~/") {
		return path.replace("~/", home_dir_string.clone().as_str());
	}

	return String::from(path);
}

// read indiviual lines of a given file into a vector
pub fn read_lines(file: File) -> Vec<String> {
	let mut lines: Vec<String> = Vec::new();

	let file_reader = BufReader::new(file);
	for line in file_reader.lines() {
		lines.push(line.unwrap());
	}

	return lines;
}

pub fn get_file_name(path: &str) -> String {
	if path.is_empty() {
		panic!("Cannot get the name of an empty path String!");
	}

	if !path.contains('/') {
		return String::from(path);
	}

	let mut file_name: String = String::new();

	let mut counter: usize = 0;
	let mut current_char: char = path.chars().nth_back(0).unwrap();
	while current_char != '/' {
		file_name.insert(0, current_char);
		counter += 1;
		current_char = path.chars().nth_back(counter).unwrap();
	}

	return file_name;
}
