use std::io;
use std::vec;
use std::str;
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::io::Error;
use std::env;

fn count_lines_better(filename:&str) -> i32 {
	let file: BufReader<File> = BufReader::new(File::open(filename).expect("Unable to open file"));
	let mut cnt  = 0;
	for _ in file.lines() {
		cnt = cnt + 1;
	}
	cnt
}

fn get_line_at(path: &Path, line_num: usize) -> Result<String, Error> {
	let file = File::open(path).expect("File not found or cannot be opened");
	let content = BufReader::new(&file);
	let mut lines = content.lines();
	lines.nth(line_num).expect("No line found at that position")
}

fn get_word(firstletter:char) -> String {
	let orderforfunction: [char;11] = ['n','s','p','O','S','A','C','M','t','i','a'];
	let txtarray: [&str; 11] = ["./txt/nouns.txt", "./txt/singularnouns.txt", "./txt/pluralnouns.txt", "./txt/opinionadjectives.txt", "./txt/sizeadjectives.txt", "./txt/ageadjectives.txt", "./txt/colouradjectives.txt", "./txt/materialadjectives.txt", "./txt/transitiveverbs.txt", "./txt/intransitiveverbs.txt", "./txt/adverbs.txt"];
	let mut wordtype: usize = 0;
	if firstletter == 't' {
		return String::from("the")
	}
	else if firstletter == 'I' {
		let mut rng = rand::thread_rng();
		let outputnumber = rng.gen_range(1..(count_lines_better("./txt/intransitiveverbs.txt")));
		let mut tempword = get_line_at(Path::new("./txt/intransitiveverbs.txt"),outputnumber as usize).expect("Error!");
		tempword.push('s');
		return tempword;
	}
	else {
		for n in 0..11 {
			if firstletter == orderforfunction[n] {
				wordtype = n;
			}
		}
		let mut rng = rand::thread_rng();
		let outputnumber = rng.gen_range(1..(count_lines_better(txtarray[wordtype])));
		get_line_at(Path::new(txtarray[wordtype]),outputnumber as usize).expect("Error!")
	}
}

fn get_structure(adjectives:i32, adverbs:i32) -> [char;15] {
	let mut structarray: [char; 15] = ['x';15];
	let mut rng = rand::thread_rng();
	let mut cnt:i32 = 0;
	if rng.gen_range(1..3) == 1 {
		structarray[0] = 't';
		cnt = cnt + 1;
	}
	let mut plural:bool = true;
	let mut cnt2 = adjectives;
	let adjarray = ['O','S','A','C','M'];
	let mut adjarray2 = ['X';5];
	while cnt2!=0{
		adjarray2[(5-cnt2) as usize] = adjarray[rng.gen_range(0..5)];
		cnt2 = cnt2-1;
	}
	for n in adjarray {
		for m in adjarray2 {
			if n == m {
				structarray[cnt as usize]=n;
				cnt = cnt + 1;
			}
		}
	}
	if structarray[0]=='t'{
		if rng.gen_range(1..3) == 1{
			structarray[cnt as usize]='p';
			plural = true;
		} else {
			structarray[cnt as usize]='s';
			plural = false;
		}
	}
	else {
		structarray[cnt as usize]='p';
	}
	cnt = cnt + 1;
	for _ in 0..adverbs{
		structarray[cnt as usize]='a';
		cnt = cnt + 1;
	}
	if plural == true {
		structarray[cnt as usize]='i';
	}
	else {
		structarray[cnt as usize] = 'I';
	}
	structarray
}

fn main() {
	let arguments: Vec<String> = env::args().collect();
	if arguments[0] == "h"{

	}
	let y = get_structure(2, 1);
	let y2: String = y.iter().collect();
	println!("{}", y2);
	for letter in y {
		if letter == 'x' {
			break;
		}
		else {
			print!("{} ", get_word(letter));
		}
	}
	print!("\nType in 'cgisf h' for help.")
}