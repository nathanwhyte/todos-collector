mod files;

use std::{
	env,
	fs::{File, OpenOptions},
	io::{BufRead, BufReader, Write},
	process::Command,
};

static TODO_LIST_TARGET_PATH: &str = "/home/natew/Dropbox/Org/planner/todo-list.org";

fn main() {
	let args: Vec<String> = env::args().collect();
	collect_todos(&args);
	init_target_file();
	read_todos_list_file();
}

fn collect_todos(args: &Vec<String>) {
	if args.len() > 1 {
		Command::new("sh")
			.arg("find-org-files.sh")
			.arg("-d")
			.arg(&args[1])
			.output()
			.expect("Failed to execute command");
	} else {
		Command::new("sh")
			.arg("find-org-files.sh")
			.output()
			.expect("Failed to execute command");
	}
}

fn init_target_file() {
	let mut target_file: File = files::create_file(TODO_LIST_TARGET_PATH);
	writeln!(target_file, "#+TITLE: global TODO list\n",).expect("Error initializing target file!");
}

fn read_todos_list_file() {
	let todo_files_list: File = files::open_file("todo-files.txt");

	let file_reader = BufReader::new(todo_files_list);
	for line in file_reader.lines() {
		handle_todo_file(line.unwrap());
	}
}

fn handle_todo_file(todo_file_path: String) {
	let lines: Vec<String> = files::read_lines(files::open_file(&todo_file_path));
	let mut todo_list_target: File = OpenOptions::new()
		.write(true)
		.append(true)
		.open(TODO_LIST_TARGET_PATH)
		.unwrap();
	let todo_file_name: String = files::get_file_name(&todo_file_path);

	writeln!(
		todo_list_target,
		"{}",
		build_print_string(lines, todo_file_name)
	)
	.expect("Error writing to target file!");
}

fn build_print_string(lines: Vec<String>, todo_file_name: String) -> String {
	let mut write_string: String = String::new();
	write_string.push_str("* ");
	write_string.push_str(&todo_file_name);
	write_string.push_str("\n");

	let mut under_todo_header: bool = false;
	// let mut current_shallowest_depth: i32 = 0;
	for line in lines {
		if line.contains("* TODO ") {
			under_todo_header = true;
			write_string.push_str(&line);
			write_string.push_str("\n");
		} else if under_todo_header {
			write_string.push_str(&line);
			write_string.push_str("\n");
		}

		if line.is_empty() {
			under_todo_header = false;
		}
	}

	return write_string;
}
