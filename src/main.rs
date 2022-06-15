mod getstructure;
mod getword;
use clap::Parser;
use getstructure::get_structure;
use getword::get_word;
use rand::Rng;

fn string_cleanup(str: String) -> String {
    let mut char_vec: Vec<char> = str.chars().collect();
    char_vec[0] = char_vec[0].to_ascii_uppercase();
    // It's absolutely ridiculous how much work it takes to convert the first letter of a String to uppercase in Rust.
    char_vec[(str.len() - 1) as usize] = '.';
    // I never want to touch this cleanup function again.
    char_vec.into_iter().collect()
}

/// A random sentence generator.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of adjectives in the first portion of the sentence
    #[clap(short = 'a', long = "adjectives", value_parser, default_value_t = 2)]
    adjectives1: i32,
    /// Number of adverbs in the first portion of the sentence
    #[clap(short = 'v', long = "adverbs", value_parser, default_value_t = 1)]
    adverbs1: i32,
    /// Structure of the sentence.
    /// Takes the following structures for the following values:
    /// 1: Colourless Green Ideas Sleep Furiously. (Adjectives-Noun-Verb-Adverbs)
    /// 2: Colourless Green Ideas Furiously Sleep. (Adjectives-Noun-Adverbs-Verb)
    /// 3: Colourless Green Ideas Hit Furiously Red Sheep. (Adjectives-Noun-Verb-Adverbs-Adjectives-Noun)
    /// 4: Colourless Green Ideas Furiously Hit Red Sheep. (Adjectives-Noun-Verb-Adverbs-Noun-Adjectives)
    #[clap(
        short = 's',
        long = "structure",
        value_parser,
        default_value_t = 1,
        verbatim_doc_comment
    )]
    structuretype: i32,
    /// Whether the first noun is plural. 0 is false and 1 is true.
    #[clap(short = 'p', long = "plural", value_parser, default_value_t = 1)]
    plurality: i32,
    /// Number of adjectives for the second part of the sentence. No effect if sentence structure
    /// is 3 or 4.
    #[clap(
        short = 'A',
        long = "adjectives_two",
        value_parser,
        default_value_t = 1
    )]
    adjectives2: i32,
    /// Whether the second noun is plural. 0 is false and 1 is true. No effect if sentence structure is 3 or 4.
    #[clap(short = 'P', long = "plural_two", value_parser, default_value_t = 0)]
    plurality2: i32,
}

fn main() {
    /*
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
    let mut args = Args::parse();
    let mut rng = rand::thread_rng();
    if args.adverbs1 < 0 {
        args.adverbs1 = rng.gen_range(1..4);
    }
    if args.adjectives1 < 0 {
        args.adjectives1 = rng.gen_range(1..4);
    }
    if args.adjectives2 < 0 {
        args.adjectives2 = rng.gen_range(1..4);
    }
    if args.structuretype < 0 {
        args.structuretype = rng.gen_range(1..5);
    }
    let plural1 = {
        if args.plurality == 0 {
            false
        } else if args.plurality == 1 {
            true
        } else {
            rng.gen_bool(0.5)
        }
    };
    let plural2 = {
        if args.plurality2 == 0 {
            false
        } else if args.plurality2 == 1 {
            true
        } else {
            rng.gen_bool(0.5)
        }
    };
    let y = get_structure(
        args.adjectives1,
        args.adverbs1,
        args.structuretype,
        plural1,
        args.adjectives2,
        plural2,
    );
    let mut final_sentence = String::new();
    for letter in y {
        if letter == 'x' {
            break;
        } else {
            let mut word = get_word(letter);
            word.push(' ');
            final_sentence.push_str(&word);
        }
    }
    print!("{}", string_cleanup(final_sentence));
}
