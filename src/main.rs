mod getword;
use getword::get_word;
use std::io;
use std::vec;
use std::str;
use rand::Rng;
use std::env;

fn get_structure(adjectives:i32, adverbs:i32) -> [char;15] {
	let mut structarray: [char; 15] = ['x';15];
	let mut rng = rand::thread_rng();
	let mut cnt:i32 = 0;
	if rng.gen_range(1..3) == 1 {
		structarray[0] = '0';
		cnt = cnt + 1;
		// Why is 'the' represented by a '0', you might ask? Because t is going to be used for transitive verbs. I hope.
	}
	let mut plural:bool = true;
	let mut cnt2 = adjectives;
	let adjarray = ['O','S','A','C','M'];
	let mut adjarray2 = ['X';5];
	while cnt2!=0{
		adjarray2[(5-cnt2) as usize] = adjarray[rng.gen_range(0..5)];
		cnt2 = cnt2-1;
	}
	// Populates adjarray2 with the appropriate number of capital letters (representing adjectives).
	for n in adjarray {
		for m in adjarray2 {
			if n == m {
				structarray[cnt as usize]=n;
				cnt = cnt + 1;
			}
		}
	}
	// Ensures all the adjectives generated are present + maintains the unconscious order of adjectives.
	if structarray[0]=='0'{
		if rng.gen_range(1..3) == 1{
			structarray[cnt as usize]='p';
			plural = true;
		} else {
			structarray[cnt as usize]='s';
			plural = false;
		}
	}
		// Maybe I should move plural-singular decision to above.
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
	// I really need to add this in in the future.
}