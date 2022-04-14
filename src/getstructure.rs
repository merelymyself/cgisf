use rand::Rng;

fn get_starting_structure1(adjectives:i32, plural:bool) -> Vec<char> {
    let mut struct_array:Vec<char> = Vec::new();
    let mut rng = rand::thread_rng();
    if plural == false || rng.gen_bool(0.5) {
        struct_array.push('0');
        // Why is 'the' represented by a '0', you might ask? Because t is going to be used for transitive verbs. I hope.
    }
    let adj_array = ['O','S','A','C','M'];
    let mut adj_array2 = Vec::new();
    for _ in 0..adjectives {
        adj_array2.push(adj_array[rng.gen_range(0..5)]);
    }
    // Populates adj_array2 with the appropriate number of capital letters (representing adjectives).
    for n in adj_array {
        for m in &adj_array2 {
            if &n == m {
                struct_array.push(n);
                // incredible that an '&' will give me such grief. adj_array2 has to be borrowed to not affect it, in the non-existent future use case.
            }
        }
    }
    // Ensures all the adjectives generated are present + maintains the unconscious order of adjectives.
    if plural == true {
        struct_array.push('p');
    } else {
        struct_array.push('s');
    }
    return struct_array;
}

fn get_ending_structure1(adverbs:i32, plural:bool, ex_struct:Vec<char>) -> Vec<char> {
    let mut struct_array = ex_struct;
    if plural == true {
        struct_array.push('i');
    }
    else {
        struct_array.push('I');
    }
    for _ in 0..adverbs{
        struct_array.push('a');
    }
    struct_array.push('x');
    return struct_array;
}

fn get_ending_structure2(adverbs:i32, plural:bool, ex_struct:Vec<char>) -> Vec<char> {
    let mut struct_array = ex_struct;
    for _ in 0..adverbs{
        struct_array.push('a');
    }
    if plural == true {
        struct_array.push('v');
    }
    else {
        struct_array.push('V');
    }
    struct_array.push('x');
    return struct_array;
}

fn get_ending_structure3(adverbs:i32, plural:bool, adjectives2:i32, plural2:bool, ex_struct:Vec<char>) -> Vec<char> {
    let mut rng = rand::thread_rng();
    let mut struct_array = ex_struct;
    if plural == true {
        struct_array.push('v');
    }
    else {
        struct_array.push('V');
    }
    for _ in 0..adverbs {
        struct_array.push('a');
    }
    if plural2 == false || rng.gen_bool(0.5) {
        struct_array.push('0');
    }
    let adj_array = ['O','S','A','C','M'];
    let mut adj_array2 = Vec::new();
    for _ in 0..adjectives2 {
        adj_array2.push(adj_array[rng.gen_range(0..5)]);
    }
    for n in adj_array {
        for m in &adj_array2 {
            if &n == m {
                struct_array.push(n);
            }
        }
    }
    if plural2 == true {
        struct_array.push('p');
    } else {
        struct_array.push('s');
    }
    struct_array.push('x');
    return struct_array;
}

pub fn get_structure(adjectives:i32, adverbs:i32, structure:i32, plural:bool, adjectives2:i32, plural2:bool) -> Vec<char>{
    return if structure == 1 {
        get_ending_structure1(adverbs, plural, get_starting_structure1(adjectives, plural))
    } else if structure == 2 {
        get_ending_structure2(adverbs, plural, get_starting_structure1(adjectives, plural))
    } else if structure == 3 {
        get_ending_structure3(adverbs, plural, adjectives2,plural2, get_starting_structure1(adjectives, plural))
    } else {
        Vec::from(['x'])
    }
}