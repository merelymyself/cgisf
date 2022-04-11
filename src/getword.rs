use std::str;
use rand::Rng;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::io::Error;

/*fn count_lines_better(filename:&str) -> i32 {
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
*/

fn count_lines_string(str:&String) -> i32 {
    let mut lines = str.lines();
    return lines.count() as i32;
}

fn get_word_no(count:i32, str:&String) -> String {
    let mut count2 = count - 1;
    let mut lines = str.lines();
    while count2 != 0 {
        lines.next();
        count2 = count2 - 1;
    }
    return lines.next().unwrap().to_string();
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
    let orderforfunction: [char;11] = ['a','A','C','i','M','n','O','p','s','S','t'];
    let words = [String::from_utf8(include_bytes!("adverbs.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("ageadjectives.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("colouradjectives.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("intransitiveverbs.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("materialadjectives.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("nouns.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("opinionadjectives.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("pluralnouns.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("singularnouns.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("sizeadjectives.txt").to_vec()).expect("error"), String::from_utf8(include_bytes!("transitiveverbs.txt").to_vec()).expect("error")];
    let mut wordtype: usize = 0;
    if firstletter == '0' {
        return String::from("the");
    }
    else if firstletter == 'I' {
        /*let mut rng = rand::thread_rng();
        let outputnumber = rng.gen_range(1..(count_lines_better("./txt/intransitiveverbs.txt")));
        let mut tempword = get_line_at(Path::new("intransitiveverbs.txt"), outputnumber as usize).expect("Error!");
        tempword.push('s');
        return tempword;*/
        let mut rng = rand::thread_rng();
        let outputnumber = rng.gen_range(1..(count_lines_string(&words[3])));
        let mut word = get_word_no(outputnumber, &words[3]);
        word.push('s');
        return word;
    }
    else {
        for n in 0..11 {
            if firstletter == orderforfunction[n] {
                wordtype = n;
            }
        }
        let mut rng = rand::thread_rng();
        let outputnumber = rng.gen_range(1..(count_lines_string(&words[wordtype])));
        return get_word_no(outputnumber, &words[wordtype]);
    }
}