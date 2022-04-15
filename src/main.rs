mod getword;
mod getstructure;
use getstructure::get_structure;
use getword::get_word;
use rand::Rng;
use std::env;


fn string_cleanup(str:String) -> String {
	let mut char_vec: Vec<char> = str.chars().collect();
	char_vec[0] = char_vec[0].to_ascii_uppercase();
	// It's absolutely ridiculous how much work it takes to convert the first letter of a String to uppercase in Rust.
	char_vec[(str.len()-1) as usize] = '.';
	// I never want to touch this cleanup function again.
	return char_vec.into_iter().collect();
}

fn main() {/*
	let adverbs = include_bytes!("adverbs.txt");
	let ageadjectives = include_bytes!("ageadjectives.txt");
	let colouradjectives = include_bytes!("colouradjectives.txt");
	let intransitiveverbs = include_bytes!("verbs.txt");
	let materialadjectives = include_bytes!("materialadjectives.txt");
	let nouns = include_bytes!("nouns.txt");
	let opinionadjectives = include_bytes!("opinionadjectives.txt");
	let pluralnouns = include_bytes!("nouns.txt");
	let singularnouns = include_bytes!("singularnouns.txt");
	let sizeadjectives = include_bytes!("sizeadjectives.txt"); */
	let mut arguments: Vec<String> = env::args().collect();
	arguments.remove(0);
	let mut values = [2,1,1,1,1];
	let mut plural:bool = true;
	let mut plural2:bool = true;
	let mut cnt = 0;
	for arg in arguments{
		if arg == "h" {
			print!("Takes in arguments of x y structure plural/singular z plural/singular, where x is the number of adjectives and y is the number of adverbs. Defaults to 2, 1 respectively if nothing typed in. \nStructural argument is in the form of an integer, listed below. Defaults to structure 1.\nTo indicate plural or singular, use 'p' or 's' respectively. Defaults to plural.\nIf you are calling for a transitive sentence, use z to decide the number of adjectives of the second noun. Defaults to 1.\nIf you are calling for a transitive sentence, use the second 'p' and 's' to decide the plural/singular of the second noun. \nIf you want ANY value randomised, use '-'.");
			return;
		}
		// adjectives1 adverbs1 structuretype plurality adjectives2 plurality2
		if arg == "-" {
			if cnt == 0 || cnt == 1 || cnt == 4 {
				let mut rng = rand::thread_rng();
				values[cnt] = rng.gen_range(1..4);
			}
			if cnt == 2 {
				let mut rng = rand::thread_rng();
				values[cnt] = rng.gen_range(1..5);
			}
			if cnt == 3 {
				let mut rng = rand::thread_rng();
				plural = rng.gen_bool(0.5);
			}
			if cnt == 5 {
				let mut rng = rand::thread_rng();
				plural2 = rng.gen_bool(0.5);
			}
		}
		else if cnt == 3 {
			if arg=="p"{
				plural = true;
			}
			if arg=="s"{
				plural = false;
			}
		}
		else if cnt == 5 {
			if arg=="p"{
				plural2 = true;
			}
			if arg=="s"{
				plural2 = false;
			}
		}
		else {
			values[cnt] = arg.parse().expect("Incorrect argument submitted.");
		}
		cnt = cnt + 1;
	}
	let y = get_structure(values[0], values[1], values[2], plural,values[4], plural2);
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
	print!("\nArgument 'h' for help.");
	// I really need to add this in in the future.
	return;
}