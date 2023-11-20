use color_eyre::eyre::Result;
use Cherris::{self, ChessGame};


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
    let mut board: Cherris::Board = Cherris::Board::init();
    board.update_hashmap();

    let mut game = Cherris::Game::init();

    game.play();

    Ok(())
}

