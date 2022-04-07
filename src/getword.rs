use std::str;
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::io::Error;

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

/*
The full linkage of characters and what they represent:
n - nouns (general), made plural by adding an 's' at the end.
s - nouns (singular), only exist in singular form
p - nouns (plural), only exist in plural form
O - opinion adjective
S - size adjective
A - age adjective
C - colour adjective
M - material adjective
t - transitive verbs, aka noun verb noun.
i - intransitive verbs, aka noun verb.
a - adverbs
0 - 'the', couldn't think of any other way to implement it.
I - intransitive verb, except for the singular form - aka 'coders code' vs 'coder codes'
 */

pub fn get_word(firstletter:char) -> String {
    let orderforfunction: [char;11] = ['n','s','p','O','S','A','C','M','t','i','a'];
    let txtarray: [&str; 11] = ["./txt/nouns.txt", "./txt/singularnouns.txt", "./txt/pluralnouns.txt", "./txt/opinionadjectives.txt", "./txt/sizeadjectives.txt", "./txt/ageadjectives.txt", "./txt/colouradjectives.txt", "./txt/materialadjectives.txt", "./txt/transitiveverbs.txt", "./txt/intransitiveverbs.txt", "./txt/adverbs.txt"];
    let mut wordtype: usize = 0;
    if firstletter == '0' {
        return String::from("the");
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