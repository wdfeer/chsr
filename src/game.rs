use std::io::{stdin};

type Board = Vec<u8>;

pub fn loop_game() {
    let mut board= get_default_board();

    println!("The game begins!");

    loop {
        for i in 1..=2 {
            process_player(i, &mut board);
        }
    }
}

const EMPTY_PIECE: u8 = 0;
const KING: u8 = 1;
fn get_default_board() -> Board {
    let king_positions: Vec<usize> = vec![3, 64 - 4];

    let mut board: Board = vec![EMPTY_PIECE; 64];
    for p in king_positions {
        board[p] = KING;
    }

    board
}

fn print_king_positions(board: Board) {
    println!("The kings are at [{}]",
             get_kings(board)
                 .iter()
                 .map(|x| format!("{} ", x.to_string()))
                 .collect::<String>());
}
fn get_kings(board: Board) -> Vec<usize> {
    let mut positions = vec![];
    for i in 0..board.len() {
        if board[i] == KING {
            positions.push(i)
        }
    };
    positions
}

fn process_player(player: u8, mut board: &mut Board) {
    println!();
    print_king_positions(board.clone());
    println!("Player {}: make your move in the format 'x1 x2'", player);

    let mov = read_valid_move(player.clone(), board.clone());
    make_move(&mut board, mov);
    
    if get_kings(board.clone()).len() <= 1 {
        println!("Player {} wins!!!", player);
        std::process::exit(0);
    }
}

fn read_valid_move(player: u8, board: Board) -> Vec<usize> {
    loop {
        let mov = read_move();
        if is_valid_move(player, board.clone(), mov.clone()) {
            return mov
        } else {
            println!("Invalid move! Try again:")
        }
    };
}

fn is_valid_move(player: u8, board: Board, positions: Vec<usize>) -> bool {
    if positions.iter().any(|x| *x > 63) { return false }

    if board[positions[0]] == 0 { return false }

    let diff = positions[1] as i32 - positions[0] as i32;
    let adjacent = vec![1, 7, 8, 9].contains(&diff.abs());

    adjacent
}

fn read_move() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read stdin!");

    input.split(" ")
        .map(|x|
            str::parse::<usize>(x.trim()).expect("Failed to parse user input!"))
        .collect::<Vec<usize>>()
}

fn make_move(board: &mut Board, positions: Vec<usize>) {
    board[positions[1]] = board[positions[0]];
    board[positions[0]] = EMPTY_PIECE;
}