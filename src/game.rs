use std::io::{stdin};

const BOARD_SIZE: usize = 64;
type Board = Vec<i8>;

pub fn loop_game() {
    let mut board= get_default_board();

    println!("The game begins!");

    loop {
        for i in 1..=2 {
            process_player(i, &mut board);
        }
    }
}

const EMPTY_PIECE: i8 = 0;
const KING: i8 = 1;
const ROOK: i8 = 2;
fn get_default_board() -> Board {
    let mut board: Board = vec![EMPTY_PIECE; 64];
    board[0] = ROOK;
    board[3] = KING;
    board[7] = ROOK;
    board[BOARD_SIZE - 8] = -ROOK;
    board[BOARD_SIZE - 5] = -KING;
    board[BOARD_SIZE - 1] = -ROOK;

    board
}

fn print_piece_positions(message: &str, piece: i8, board: Board) {
    println!("{message} {}",
             get_pieces(board, piece, true)
                 .iter()
                 .map(|x| format!("{} ", x.to_string()))
                 .collect::<String>());
}

fn get_pieces(board: Board, piece: i8, ignore_team: bool) -> Vec<usize> {
    let mut positions = vec![];
    for i in 0..BOARD_SIZE {
        if board[i] == piece || (ignore_team && board[i].abs() == piece.abs()) {
            positions.push(i)
        }
    };
    positions
}

fn process_player(player: u8, mut board: &mut Board) {
    println!();
    print_piece_positions("The kings are at", KING, board.clone());
    print_piece_positions("The rooks are at", ROOK, board.clone());
    println!("Player {}: make your move in the format 'x1 x2'", player);

    let mov = read_valid_move(player.clone(), board.clone());
    make_move(&mut board, mov);
    
    if get_pieces(board.clone(), KING, true).len() <= 1 {
        println!("Player {} wins!!!", player);
        std::process::exit(0);
    }
}

fn read_valid_move(player: u8, board: Board) -> Vec<usize> {
    loop {
        let mov = get_valid_move(player, board.clone(), read_move());
        match mov {
            Ok(m) => {
                return m;
            }
            Err(e) => {
                println!("Invalid move: {}! Try again.", e);
            }
        }
    };
}

fn get_valid_move(player: u8, board: Board, inputs: Vec<String>) -> Result<Vec<usize>, &'static str> {
    let numbers = inputs.iter().map(|x| str::parse::<i16>(x).unwrap()).collect::<Vec<i16>>();

    // out of bounds check
    if numbers.iter().any(|x| *x < 0 || *x >= board.len() as i16) { return Err("position out of bounds") }

    let positions = numbers.iter().map(|x| *x as usize).collect::<Vec<usize>>();

    let attacker = board[positions[0]];
    if attacker == EMPTY_PIECE { return Err("cannot move empty space") }
    if (attacker >= 0) == (player == 2) { return Err("cannot move enemy pieces") }

    let defender = board[positions[1]];
    if (defender != EMPTY_PIECE) && ((attacker < 0) == (defender < 0)) { return Err("cannot capture own pieces") }

    if can_piece_move(attacker.abs(), positions[0], positions[1]){
        Ok(positions)
    } else {
        Err("this piece cannot move like that")
    }
}

fn can_piece_move(piece: i8, from: usize, to: usize) -> bool {
    let diff = to as i32 - from as i32;
    match piece {
        KING => vec![1, 7, 8, 9].contains(&diff.abs()),
        ROOK => diff % 8 == 0 || (to / 8) == (from / 8),
        _ => panic!("Piece {} is invalid!", piece)
    }
}

fn raycast(board: Board, origin: usize, next_pos: fn(usize) -> usize) -> Result<Vec<usize>, None> {
    // TODO: Implement checking 'next_pos' squares of the board until hitting a piece or board ending
}

fn read_move() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read stdin!");

    input.split(" ").map(|x|x.trim().to_string()).collect::<Vec<String>>()
}

fn make_move(board: &mut Board, positions: Vec<usize>) {
    board[positions[1]] = board[positions[0]];
    board[positions[0]] = EMPTY_PIECE;
}
