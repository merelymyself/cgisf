use rand::Rng;

fn get_starting_structure1(adjectives:i32, plural:bool) -> Vec<char> {
    let mut structarray:Vec<char> = Vec::new();
    let mut rng = rand::thread_rng();
    if plural == false || rng.gen_bool(0.5) {
        structarray.push('0');
        // Why is 'the' represented by a '0', you might ask? Because t is going to be used for transitive verbs. I hope.
    }
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
    if plural == true {
        structarray.push('p');
    } else {
        structarray.push('s');
    }
    return structarray;
}

fn get_ending_structure1(adverbs:i32, plural:bool, exstruct:Vec<char>) -> Vec<char> {
    let mut structarray = exstruct;
    if plural == true {
        structarray.push('i');
    }
    else {
        structarray.push('I');
    }
    for _ in 0..adverbs{
        structarray.push('a');
    }
    structarray.push('x');
    return structarray;
}

fn get_ending_structure2(adverbs:i32, plural:bool, exstruct:Vec<char>) -> Vec<char> {
    let mut structarray = exstruct;
    for _ in 0..adverbs{
        structarray.push('a');
    }
    if plural == true {
        structarray.push('v');
    }
    else {
        structarray.push('V');
    }
    structarray.push('x');
    return structarray;
}

pub fn get_structure(adjectives:i32, adverbs:i32, plural:bool, structure:i32) -> Vec<char>{
    return if structure == 1 {
        get_ending_structure1(adverbs, plural, get_starting_structure1(adjectives, plural))
    } else if structure == 2 {
        get_ending_structure2(adverbs, plural, get_starting_structure1(adjectives, plural))
    } else {
        Vec::from(['x'])
    }
}