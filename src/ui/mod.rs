use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const WELCOME: &str = "Welcome to chsr!\n[P]lay\n[Q]uit\n";

pub fn start_game() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::clear::All, WELCOME).expect("Printing to stdout failed!");
    stdout.flush().unwrap();

    for key in stdin.keys() {
        match_key(key.unwrap());
        stdout.flush().unwrap();
    }
}

fn match_key(key: Key) {
    match key {
        Key::Char('p') => play(),
        Key::Char('q') => quit(),
        _ => ()
    }
}

fn play() {
    println!("TODO: start gaming");
}

fn quit() {
    println!("TODO: quit the game");
}