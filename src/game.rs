use std::io::{stdin};

pub fn loop_game() {
    let mut board= get_default_board();

    println!("The game begins!");
    print_king_positions(board.clone());

    loop {
        println!("Player 1: make your move in the format 'x1 x2'");
        make_move(&mut board, read_move())
    }
}

const EMPTY_PIECE: u8 = 0;
const KING: u8 = 1;
fn get_default_board() -> Vec<u8> {
    let king_positions: Vec<usize> = vec![3, 64 - 4];

    let mut board: Vec<u8> = vec![EMPTY_PIECE; 64];
    for p in king_positions {
        board[p] = KING;
    }

    board
}

fn print_king_positions(board: Vec<u8>) {
    println!("The kings are at [{}]",
             get_kings(board)
                 .iter()
                 .map(|x| x.to_string())
                 .collect::<String>());
}
fn get_kings(board: Vec<u8>) -> Vec<usize> {
    let mut positions = vec![];
    for i in 0..board.len() {
        if board[i] == KING {
            positions.push(i)
        }
    };
    positions
}

fn read_move() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read stdin!");

    input.split(" ").map(|x| str::parse::<usize>(x).expect("Failed to parse user input!")).collect::<Vec<usize>>()
}

fn make_move(board: &mut Vec<u8>, positions: Vec<usize>) {
    board[positions[1]] = board[positions[0]];
    board[positions[0]] = EMPTY_PIECE;
}