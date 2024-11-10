use std::io::{stdin, BufRead};

const EMPTY_PIECE: u8 = 0;
const KING: u8 = 1;

const KING_POSITIONS: Vec<u8> = vec![3, 64 - 4];

pub fn loop_game() {
    let mut board: Vec<u8> = vec![EMPTY_PIECE; 64];
    for p in KING_POSITIONS {
        board[p] = KING;
    }

    println!("The game begins! The 2 kings are at [{}]", KING_POSITIONS.split(","));

    loop {
        println!("Player 1: make your move in the format 'x1 x2'");
        make_move(&mut board, read_move())
    }
}

fn read_move() -> Vec<u8> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read stdin!");

    input.split(" ").map(|x| u8::try_from(x)).collect().collect()
}

fn make_move(mut board: &Vec<u8>, positions: Vec<u8>) {
    board[positions[1]] = board[positions[0]];
    board[positions[0]] = EMPTY_PIECE;
}