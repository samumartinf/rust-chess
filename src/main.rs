use std::{collections::{HashMap}};
use color_eyre::eyre::Result;

const PIECE_BIT:u8 = 128u8;
const WHITE_BIT:u8 = 64u8;
const PAWN_BIT:u8 = 8u8;
const CHECK_PIECE:u8 = 0b00001111;
const KING:u8 = 0u8;
const QUEEN:u8 = 1u8;
const BISHOP:u8 = 2u8;
const KNIGHT:u8 = 4u8;
const ROOK:u8 = 6u8;
const ROW:u8 = 16u8;
const COL:u8 = 1u8;

// Instructions:
/*
-  The position of a piece is given by byte, the first four bits represent the row, counting from top to bottom, the columns from left to right 
-  Each piece has a unique key that is represented by a byte. That key follows the following structure. 
    - The first bit represents whether the piece exists or not 
    - The second bit indicates whether a piece is white or black (white=1, black=0)
    - The last four bits represent the pieces (numbers 0-15) as below
*/
fn main() -> Result<()>{
    color_eyre::install()?;
    let mut board: Board = Board::init();
    board.update_hashmap();

    let pos_string:String = String::from("a2");
    let position = positionHelper::letter_to_position_byte(pos_string);
    let pos_letter =  positionHelper::position_byte_to_letter(position);
    let white_pawn = Piece::init_from_binary(PIECE_BIT+WHITE_BIT+PAWN_BIT);
    let possible_positions: Vec<String> = white_pawn.possible_moves(position, &board).iter().map(|x| positionHelper::position_byte_to_letter(*x)).collect();

    let mut game = Game{white_turn: true, moves_done: Vec::new(), board: &board, game_done:false};

    while !game.game_done {
        game.board.show();
        game.white_turn = !game.white_turn;
        game.game_done = true;
    }
    print!("The possible positions for the pawn at {} are {:?}", pos_letter, possible_positions);

    Ok(())
}


struct Game<'a> {
    white_turn: bool, 
    moves_done: Vec<u32>,
    board: &'a Board,
    game_done: bool,
}


#[derive(Debug, Clone)]
struct Piece {
    binary: u8,
    is_white: bool,
    class: PieceType,
}

#[derive(Debug, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    fn pawn_moves(self, position:u8, board:Board) -> Vec<u8> {
        let mut possible_positions = Vec::new();

        // White pawns move in the negative direction
        if self.is_white {
            possible_positions.push(position - 16);

            println!("The row of the pawn is identified as {}", positionHelper::get_row(position));
            if positionHelper::get_row(position) == 6 { 
                possible_positions.push(position - 32);
            }
        }

        // Black paws move in the positive direction
        else {
            possible_positions.push(position + 16);
            if positionHelper::get_row(position) == 1 {
                possible_positions.push(position + 32);
            }
        }

        let mut final_positions = Vec::new();
        for pos in possible_positions {
            println!("{}", pos);
            if positionHelper::validate_position(pos) {
                final_positions.push(pos);
            }
        }

        //Handle taking pieces
        let piece1 = board.pieces.get(&(position + ROW + COL));
        let piece2 = board.pieces.get(&(position + ROW - COL));
        if piece1.is_some() { 
            println!("There do be a piece here");    
        }
        
        final_positions
    }

    fn king_moves(self, position:u8, board:Board) -> Vec<u8> {
        let mut possible_positions = Vec::new();

        possible_positions.push(position+1);
        possible_positions.push(position-1);
        possible_positions.push(position+16);
        possible_positions.push(position-16);
        possible_positions.push(position+16+1);
        possible_positions.push(position+16-1);
        possible_positions.push(position-16+1);
        possible_positions.push(position-16-1);

        let mut final_positions: Vec<u8> = Vec::new();
        for pos in possible_positions {
            if positionHelper::validate_position(pos) {
                final_positions.push(pos);
            }
        }

        return final_positions;
    }

}

impl BasicPiece for Piece {
    fn is_move_valid(&self, position:u8, board: Board) -> bool {
        true
    }

    fn possible_moves(&self, position:u8, board:&Board) -> Vec<u8> {
        let mut possible_positions = Vec::new();
        match self.class {
            PieceType::Pawn => possible_positions = Piece::pawn_moves(self.clone(), position, board.clone()),
            PieceType::King => possible_positions = Piece::king_moves(self.clone(), position, board.clone()),
            PieceType::Bishop => possible_positions.push(positionHelper::letter_to_position_byte(String::from("e4"))),
            PieceType::Queen => possible_positions.push(positionHelper::letter_to_position_byte(String::from("e4"))),
            PieceType::Rook=> possible_positions.push(positionHelper::letter_to_position_byte(String::from("e4"))),
            PieceType::Knight=> possible_positions.push(positionHelper::letter_to_position_byte(String::from("e4"))),
        }

        return possible_positions;
    }

    fn init_from_binary(binary: u8) -> Self {
        let is_white = (binary & WHITE_BIT) == WHITE_BIT;
        // The alive bit might mess this up
        let binary_piece = binary & CHECK_PIECE;

        let piece_type = match binary_piece {
            8u8..=16u8 => PieceType::Pawn,
            0u8 => PieceType::King,
            1u8 => PieceType::Queen,
            2u8 | 3u8 => PieceType::Bishop,
            4u8 | 5u8 => PieceType::Knight,
            6u8 | 7u8 => PieceType::Rook,
            _ => panic!("This piece does not exist!. The binary is {}", binary),
        };
        
        Self { binary: binary, is_white: is_white, class: piece_type}
    }

    fn text_repr(&self) -> String {
        let mut return_string = String::from("");
        let mut color_string = String::from("");

        if self.is_white {
            color_string = String::from("w");
        } 
        else {
            color_string = String::from("b");
        }

        let piece_string = match self.class {
            PieceType::Pawn => "p".to_string(),
            PieceType::King => "K".to_string(),
            PieceType::Queen => "Q".to_string(),
            PieceType::Bishop => "B".to_string(),
            PieceType::Knight => "k".to_string(),
            PieceType::Rook => "R".to_string(),
        };
        return_string.push_str(&color_string);
        return_string.push_str(&piece_string);
        return return_string;
    }
}

trait BasicPiece {
    fn is_move_valid(&self, position:u8, board: Board) -> bool;
    fn init_from_binary(binary: u8) -> Self;
    fn text_repr(&self) -> String;
    fn possible_moves(&self, position:u8, board: &Board) -> Vec<u8>;
}

#[derive(Debug, Clone)]
struct Board {
    pieces: HashMap<u8,u8>,  // HashMap<positionByte, pieceByte>
    state: [u8;64]           // arr[index] = pieceByte
}

impl Board {
    fn show(&self) {
        println!("  |----|----|----|----|----|----|----|----|");
        let mut row_count = 8;
        for row in 0..8 {
            print!("{} ", row_count);
            row_count -= 1;
            print!("|");

            for col in 0..8 {
                print!(" ");

                // Piece print
                if self.state[row*8 + col]  == 0u8 {
                    print!("  ");
                } else {
                    let piece = Piece::init_from_binary(self.state[row*8 + col]);
                    print!("{}", piece.text_repr());
                }
                
                print!(" |");
            }
            println!();
            println!("  |----|----|----|----|----|----|----|----|");
        }
        println!("    a    b    c    d    e    f    g    h  ");

    }

    fn init() -> Self { 
        let mut state = [0u8; 64];
        let mut pieces:HashMap<u8, u8> = HashMap::new();

        
        // black pawns
        let mut first_bpawn = PIECE_BIT + PAWN_BIT;
        for i in 0..8 {
            state[i+8] = first_bpawn;
            first_bpawn += 1;
        }

        // white pawns
        let mut first_wpawn = PIECE_BIT + PAWN_BIT + WHITE_BIT;
        for i in 0..8 {
            state[i+48] = first_wpawn;
            first_wpawn += 1;
        }

        // white large pieces
        state[0+56] = ROOK + PIECE_BIT + WHITE_BIT;
        state[1+56] = KNIGHT + PIECE_BIT + WHITE_BIT;
        state[2+56] = BISHOP + PIECE_BIT + WHITE_BIT;
        state[3+56] = QUEEN + PIECE_BIT + WHITE_BIT;
        state[4+56] = KING + PIECE_BIT + WHITE_BIT;
        state[5+56] = BISHOP + PIECE_BIT + WHITE_BIT + 1;
        state[6+56] = KNIGHT + PIECE_BIT + WHITE_BIT + 1;
        state[7+56] = ROOK + PIECE_BIT + WHITE_BIT + 1;

        // black large pieces
        state[0] = ROOK + PIECE_BIT;
        state[1] = KNIGHT + PIECE_BIT ;
        state[2] = BISHOP + PIECE_BIT;
        state[3] = QUEEN + PIECE_BIT;
        state[4] = KING + PIECE_BIT; 
        state[5] = BISHOP + PIECE_BIT +  1;
        state[6] = KNIGHT + PIECE_BIT + 1;
        state[7] = ROOK + PIECE_BIT + 1;


        // Populate hashmap -> done in the update_hashmap
        Self { pieces: pieces, state: state }
    }

    fn update_hashmap(&mut self) {
        for index in 0..self.state.len() {
            if self.state[index] != 0 {
                let pos_byte = positionHelper::index_to_position_byte(index);
                self.pieces.insert(pos_byte, self.state[index]);
            }
        }
    }
}


pub mod positionHelper {
    pub fn position_byte_to_index(byte: u8) -> usize {
        let row_selector:u8 = 0b11110000;
        let col_selector:u8 = 0b00001111;
        
        let row = (row_selector & byte) >> 4;
        let col = col_selector & byte;

        (row*8 + col) as usize
    }

    pub fn index_to_position_byte(index: usize) -> u8 {
        let col = index as u8 % 8;
        let  mut row = index as u8 / 8u8;
        row <<= 4;
        row | col
    }

    pub fn position_byte_to_letter(byte: u8) -> String {
        let row_selector:u8 = 0b11110000;
        let col_selector:u8 = 0b00001111;
        
        let row = (row_selector & byte) >> 4;
        let col = col_selector & byte;
        
        let mut return_string = String::from("");
        
        let letter_char = ('a' as u8 + col) as char;
        let num_char = ('8' as u8 - row) as char;

        return_string.push(letter_char);
        return_string.push(num_char);
        return_string
    }

    pub fn letter_to_position_byte(letters: String) -> u8{
        let mut letters_copy = letters.clone();
        let num_char = letters_copy.pop().unwrap();
        let letter_char = letters_copy.pop().unwrap();
        let row = 7 - (num_char as u8 - '1' as u8);
        let col = letter_char as u8 - 'a' as u8;
        (row << 4) | col
    }

    pub fn get_row(byte: u8) -> u8 {
        let row_selector:u8 = 0b11110000;
        (row_selector & byte) >> 4
    }

    pub fn get_col(byte: u8) -> u8 {
        let col_selector:u8 = 0b00001111;
        col_selector & byte
    }

    pub fn validate_position(position:u8) -> bool {
        let index_position = position_byte_to_index(position);
        if index_position >= 64 {
            return false;
        }
        if get_col(position) > 7 {
            return false;
        }
        if get_row(position) > 7 {
            return false;
        }

        return true;
    }
}
#[cfg(test)]
mod tests {
    use crate::{Board, Piece, BasicPiece, PIECE_BIT, QUEEN, PAWN_BIT, WHITE_BIT, positionHelper, KING};

    #[test]
    fn test_index_to_letters() {
        let pos_byte = positionHelper::index_to_position_byte(3); // 3 = Black Queen
        let cell = positionHelper::position_byte_to_letter(pos_byte);
        assert_eq!(pos_byte, 0b00000011);
        assert_eq!(cell, "d8");
    }

    #[test]
    fn test_letters_to_index() {
        let cell = String::from("d8");
        let pos_byte = positionHelper::letter_to_position_byte(cell);
        println!("The position byte returned is {}", pos_byte);
        let index = positionHelper::position_byte_to_index(pos_byte);
        assert_eq!(pos_byte, 0b00000011);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_state_pieces_parity() {
        let mut board = Board::init();
        board.update_hashmap();
        let piece = *board.pieces.get(&0b00000011).unwrap();
        assert_eq!(piece, PIECE_BIT+QUEEN); // Black queen should be on index 3 after init() 
    }

    #[test]
    fn test_pawn_initial_move() {
        let mut board = Board::init();
        let pos_string:String = String::from("a2");
        let position = positionHelper::letter_to_position_byte(pos_string);
        let white_pawn = Piece::init_from_binary(PIECE_BIT+WHITE_BIT+PAWN_BIT);
        let possible_positions: Vec<String> = white_pawn.possible_moves(position, &board)
        .iter()
        .map(|x| positionHelper::position_byte_to_letter(*x))
        .collect();
        assert_eq!(possible_positions, vec!["a3", "a4"]);
    }

    #[test]
    fn test_king_moves() {
        let board = Board::init();
        let pos_string:String = String::from("a1");
        let position = positionHelper::letter_to_position_byte(pos_string);
        let king = Piece::init_from_binary(PIECE_BIT+WHITE_BIT+KING);
        let mut possible_positions: Vec<String> = king.possible_moves(position, &board)
        .iter()
        .map(|x| positionHelper::position_byte_to_letter(*x))
        .collect();
        possible_positions.sort();
        println!("The positions output for the King are: {:?}", possible_positions);
        assert_eq!(possible_positions, vec!["a2", "b1", "b2"]);
    }

}
