mod getword;
use getword::get_word;
use std::io;
use std::vec;
use std::str;
use rand::Rng;
use std::env;

fn get_structure(adjectives:i32, adverbs:i32) -> Vec<char> {
	let mut structarray:Vec<char> = Vec::new();
	let mut rng = rand::thread_rng();
	let mut has_the:bool = false;
	if rng.gen_range(1..3) == 1 {
		structarray.push('0');
		has_the = true;
		// Why is 'the' represented by a '0', you might ask? Because t is going to be used for transitive verbs. I hope.
	}
	let mut plural:bool = true;
	let adjarray = ['O','S','A','C','M'];
	let mut adjarray2 = Vec::new();
	for _ in 0..adjectives {
		adjarray2.push(adjarray[rng.gen_range(0..5)]);
	}
	// Populates adjarray2 with the appropriate number of capital letters (representing adjectives).
	for n in adjarray {
		for m in &adjarray2 {
			if &n == m {
				structarray.push(n);
				// incredible that an '&' will give me such grief. adjarray2 has to be borrowed to not affect it, in the non-existent future use case.
			}
		}
	}
	// Ensures all the adjectives generated are present + maintains the unconscious order of adjectives.
	if has_the==true{
		// I will no doubt regret this scuffed implementation of a "has the word 'the'" check in the future.
		if rng.gen_range(1..3) == 1{
			structarray.push('p');
			plural = true;
		} else {
			structarray.push('s');
			plural = false;
		}
	}
		// Maybe I should move plural-singular decision to above.
	else {
		structarray.push('p');
	}
	for _ in 0..adverbs{
		structarray.push('a');
	}
	if plural == true {
		structarray.push('i');
	}
	else {
		structarray.push('I');
	}
	structarray.push('x');
	return structarray;
}

fn string_cleanup(str:String) -> String {
	let mut char_vec: Vec<char> = str.chars().collect();
	char_vec[0] = char_vec[0].to_ascii_uppercase();
	// It's absolutely ridiculous how much work it takes to convert the first letter of a String to uppercase in Rust.
	char_vec[(str.len()-1) as usize] = '.';
	// I never want to touch this cleanup function again.
	return char_vec.into_iter().collect();
}

fn main() {
	let mut arguments: Vec<String> = env::args().collect();
	arguments.remove(0);
	let mut values = [2,1];
	let mut cnt = 0;
	for arg in arguments{
		if arg == "h" {
			print!("Type in 'cgisf x y', where x is the number of adjectives and y is the number of adverbs. Defaults to 2, 1 respectively if nothing typed in. If you want it randomised, use '-'.");
			return;
		}
		if arg == "-" {
			let mut rng = rand::thread_rng();
			values[cnt] = rng.gen_range(1..4);
			cnt = cnt + 1;
		}
		else {
			values[cnt] = arg.parse().expect("That is not a number.");
			cnt = cnt + 1;
		}
	}
	let y = get_structure(values[0], values[1]);
	let y2: String = y.iter().collect();
	println!("{}", y2);
	let mut final_sentence = String::new();
	for letter in y {
		if letter == 'x' {
			break;
		}
		else {
			let mut word = get_word(letter);
			word.push(' ');
			final_sentence.push_str(&word);
		}
	}

	print!("{}", string_cleanup(final_sentence));
	print!("\nType in 'cgisf h' for help.");
	// I really need to add this in in the future.
	return;
}