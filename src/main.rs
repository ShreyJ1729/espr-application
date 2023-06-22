fn main() {
    let mut states = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut steps = vec![];

    while !states.is_empty() {
        print!("\x1B[2J\x1B[1;1H");
        println!("Select an option from below:");
        println!("1. Press one");
        println!("2. Press two opposite");
        println!("3. Press two adjacent");
        println!("4. Press three");
        println!("5. Press all");

        println!("Remaining: {} states\n", states.len());
        for state in states.iter() {
            print_state(*state);
            println!(" ");
        }

        // take user input and parse it into an integer. if user input is not an integer or the integer is not in the range of 1-5, ask for input again
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        steps.push(input);

        states = states
            .iter()
            // execute user action
            .map(|state| match input {
                1 => press_one(*state),
                2 => press_opposite(*state),
                3 => press_adjacent(*state),
                4 => press_three(*state),
                5 => press_all(*state),
                _ => *state,
            })
            // then rotate the state, storing all possible rotations
            .flat_map(|state| (0..4).map(move |positions| rotate(state, positions)))
            // then filter out the unlocked state
            .filter(|state| *state != 0)
            .collect();

        // remove duplicates
        states.sort();
        states.dedup();
    }

    // print out the steps
    println!("Solved in {} steps", steps.len());
    for step in steps.iter() {
        match step {
            1 => println!("Press one"),
            2 => println!("Press two opposite"),
            3 => println!("Press two adjacent"),
            4 => println!("Press three"),
            5 => println!("Press all"),
            _ => println!("Unknown step"),
        }
    }
}

fn print_state(state: i32) {
    print!("{:04b}", state);
}

fn press_one(state: i32) -> i32 {
    state ^ 0b1000
}

fn press_adjacent(state: i32) -> i32 {
    state ^ 0b1100
}

fn press_opposite(state: i32) -> i32 {
    state ^ 0b1010
}

fn press_three(state: i32) -> i32 {
    state ^ 0b1110
}

fn press_all(state: i32) -> i32 {
    state ^ 0b1111
}

fn rotate(state: i32, positions: i32) -> i32 {
    let mut state = state;
    for _ in 0..positions {
        state = (state << 1) | (state >> 3);
        state &= 0b1111;
    }
    state
}
