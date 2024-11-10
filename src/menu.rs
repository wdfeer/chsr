use std::io::stdin;

const WELCOME: &str = "Welcome to chsr!\n[P]lay\n[Q]uit\n";

/// Prints out the welcome message and prompts for input. Breaks on 'play'.
pub fn loop_main_menu() {
    println!("{}", WELCOME);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read stdin!");

        if match_input(input.as_str())() {
            break;
        };
    }
}

fn match_input(input: &str) -> fn() -> bool {
    match input.to_lowercase().chars().next().unwrap() {
        'p' => play,
        'q' => quit,
        _ => on_invalid_input
    }
}

fn play() -> bool {
    true
}

fn quit() -> bool {
    println!("Bye!");
    std::process::exit(0);
}

fn on_invalid_input() -> bool {
    println!("Invalid input! Try again.");
    false
}