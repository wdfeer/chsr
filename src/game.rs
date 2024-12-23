use std::io::{stdin};

const BOARD_WIDTH: u8 = 8;
const BOARD_SIZE: usize = (BOARD_WIDTH * BOARD_WIDTH) as usize;
type Board = Vec<i8>;

pub fn loop_game() {
    let mut board = get_default_board();

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
const BISHOP: i8 = 3;
const KNIGHT: i8 = 4;
const QUEEN: i8 = 5;
fn get_default_board() -> Board {
    let mut board: Board = vec![EMPTY_PIECE; 64];
    board[0] = ROOK;
    board[1] = KNIGHT;
    board[2] = BISHOP;
    board[3] = QUEEN;
    board[4] = KING;
    board[5] = BISHOP;
    board[6] = KNIGHT;
    board[7] = ROOK;
    board[BOARD_SIZE - 8] = -ROOK;
    board[BOARD_SIZE - 7] = -KNIGHT;
    board[BOARD_SIZE - 6] = -BISHOP;
    board[BOARD_SIZE - 5] = -KING;
    board[BOARD_SIZE - 4] = -QUEEN;
    board[BOARD_SIZE - 3] = -BISHOP;
    board[BOARD_SIZE - 2] = -KNIGHT;
    board[BOARD_SIZE - 1] = -ROOK;

    board
}

fn position_to_pair(pos: usize) -> (u8, u8) {
    ((pos % 8).try_into().unwrap(), (pos / 8).try_into().unwrap())
}

fn pair_to_position(x: u8, y: u8) -> usize {
    x as usize + y as usize * 8
}

fn pos_to_str(pos: usize) -> String {
    let (x, y) = position_to_pair(pos);
    format!("{x}|{y}")
}

fn print_piece_positions(message: &str, piece: i8, board: Board) {
    println!("{message} {}",
             get_pieces(board, piece, true)
                 .iter()
                 .map(|x| format!("{} ", pos_to_str(*x)))
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
    print_piece_positions("The knights are at", KNIGHT, board.clone());
    print_piece_positions("The bishops are at", BISHOP, board.clone());
    print_piece_positions("The rooks are at", ROOK, board.clone());
    print_piece_positions("The queens are at", QUEEN, board.clone());
    println!("Player {}: make your move in the format 'x1 y1 x2 y2'", player);

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
    if numbers.iter().any(|x| *x < 0 || *x >= BOARD_WIDTH as i16) { return Err("position out of bounds"); }

    let coordinates = numbers.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    let positions = vec![pair_to_position(coordinates[0], coordinates[1]), pair_to_position(coordinates[2], coordinates[3])];

    let attacker = board[positions[0]];
    if attacker == EMPTY_PIECE { return Err("cannot move empty space"); }
    if (attacker >= 0) == (player == 2) { return Err("cannot move enemy pieces"); }

    let defender = board[positions[1]];
    if (defender != EMPTY_PIECE) && ((attacker < 0) == (defender < 0)) { return Err("cannot capture own pieces"); }

    if can_piece_move(board, attacker, positions[0], positions[1]) {
        Ok(positions)
    } else {
        Err("this piece cannot move like that")
    }
}

fn can_piece_move(board: Board, piece: i8, from: usize, to: usize) -> bool {
    let diff = to as i32 - from as i32;
    match piece.abs() {
        KING => vec![1, 7, 8, 9].contains(&diff.abs()),
        ROOK =>
            // this could be simplified to a single 'multicast' call,
            // but that would decrease performance (doesn't really matter)
            (diff % 8 == 0 && multicast(board.clone(), from, vec![
                |x| x + 8, // get next square in the file
                |x| x - 8 // get previous square in the file
            ], to))
                || ((to / 8) == (from / 8) && multicast(board.clone(), from, vec![
                |x| if x % 8 < 7 { x + 1 } else { x }, // get next square in the rank
                |x| if x % 8 > 0 { x - 1 } else { x } // get previous square in the rank
            ], to)),
        BISHOP => (diff % 7 == 0 && multicast(board.clone(), from, vec![
                |x| if x / 8 > 0 && x % 8 < 7 {x - 7} else { x } // get next square in previous rank
                |x| if x % 8 > 0 {x + 7} else { x } // get previous square in next rank
            ], to))
                || (diff % 9 == 0 && multicast(board.clone(), from, vec![
                |x| if x % 8 < 7 {x + 9} else { x } // get next square in next rank
                |x| if x / 8 > 0 && x % 8 > 0 {x - 9} else { x } // get previous square in previous rank
            ], to)),
        KNIGHT => vec![6, 10, 15, 17].contains(&diff.abs()),
        QUEEN => can_piece_move(board.clone(), ROOK, from, to) ||
                can_piece_move(board.clone(), BISHOP, from, to),
        _ => panic!("Piece {} is invalid!", piece)
    }
}

/// Raycasts in different directions with the supplied 'vec_next_pos' functions.
/// True if any of them hit the target.
fn multicast(board: Board, origin: usize, vec_next_pos: Vec<fn(usize) -> usize>, target: usize) -> bool {
    vec_next_pos.iter().any(|f| raycast(board.clone(), origin, *f, target))
}

/// Calls 'next_pos' recursively until 'target' is reached.
/// False if the position goes out of bounds or hits another piece.
fn raycast(board: Board, origin: usize, next_pos: fn(usize) -> usize, target: usize) -> bool {
    fn is_within_bounds(p: usize) -> bool { p < BOARD_SIZE}
    fn can_move_through(board: &Board, p: usize) -> bool { board[p] == EMPTY_PIECE }

    let mut last_pos: usize = origin;
    let mut pos: usize = next_pos(origin);
    while pos != last_pos &&
            is_within_bounds(pos) &&
            (can_move_through(&board, pos) || pos == target) {
        last_pos = pos;
        if last_pos == target { return true }

        pos = next_pos(pos);
    };

    false
}

fn read_move() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read stdin!");

    input.split(" ").map(|x| x.trim().to_string()).collect::<Vec<String>>()
}

fn make_move(board: &mut Board, positions: Vec<usize>) {
    board[positions[1]] = board[positions[0]];
    board[positions[0]] = EMPTY_PIECE;
}
