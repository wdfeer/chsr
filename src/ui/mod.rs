use std::io::stdin;

const WELCOME: &str = "Welcome to chsr!\n[P]lay\n[Q]uit\n";

pub fn start_game() {
    println!("{}", WELCOME);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read stdin!");

    match_input(input.as_str())();
}

fn match_input(input: &str) -> fn() {
    match input.to_lowercase().chars().next().unwrap() {
        'p' => play,
        'q' => quit,
        _ => on_invalid_input
    }
}

fn play() {
    println!("TODO: start gaming");
}

fn quit() {
    println!("Bye!");
    std::process::exit(0);
}

fn on_invalid_input() {
    println!("Invalid input! Quitting the game...");
    std::process::exit(1);
}