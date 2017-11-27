extern crate termion;
use termion::{color};

/** quick n dirty just to build and get some output **/
fn main() {
    print_board( fen_to_board(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq"
    ));
    println!();
    print_board( fen_to_board(
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR"
    ));
}

/** prints a board adding terminal color **/
// TODO: last move highlight
// TODO: castling availability
fn print_board ( board: [[char; 8]; 8] ) {
    //println!("{:?}", board);
    for row in 0..8 {
        for col in 0..8 {
            match (col+row)%2 {
                0 => 
                    print!("{}{}", color::Bg(color::Red), board[row][col] ),
                _ => 
                    print!("{}{}", color::Bg(color::Black), board[row][col] )
            }
        }
        println!("{}", color::Bg(color::Reset) );
    }

}
/** convert Forsyth-Edwards Notation to a UTF-8 string with a board */
fn fen_to_board (input_str: &str) -> [[char; 8]; 8] {
    // TODO must be a less asinine way to generate the next var
    let mut out = [
        [' ',' ',' ',' ',' ',' ',' ',' ',],
        [' ',' ',' ',' ',' ',' ',' ',' ',],
        [' ',' ',' ',' ',' ',' ',' ',' ',],
        [' ',' ',' ',' ',' ',' ',' ',' ',],
        [' ',' ',' ',' ',' ',' ',' ',' ',],
        [' ',' ',' ',' ',' ',' ',' ',' ',],
        [' ',' ',' ',' ',' ',' ',' ',' ',],
        [' ',' ',' ',' ',' ',' ',' ',' ',]];
    let mut col = 0;
    let mut row = 0;
    for (inter_index, inter_content) in input_str.chars().enumerate() {
        //println!("{},{} = {}", row, col, inter_content);
        if col > 7 {
            col = 0;
            row +=1;
        }
        if row > 7 {
            break;
        }
        if inter_content == '8'{
            row += 1;
            col = 0;
            continue;
        } else if inter_content == '7' {
            col += 7;
            continue;
        } else if inter_content == '6' {
            col += 6;
            continue;
        } else if inter_content == '5' {
            col += 5;
            continue;
        } else if inter_content == '4' {
            col += 4;
            continue;
        } else if inter_content == '3' {
            col += 3;
            continue;
        } else if inter_content == '2' {
            col += 2;
            continue;
        } else if inter_content == '1' {
            col += 1;
            continue;
        } else if inter_content == '/' {
            continue;
        }
        out[row][col] = piece_from_char(inter_content);
        //
        col+=1;
    }
    return out;
}


/** naive map to the UTF-8 chars we want to draw **/
fn piece_from_char( input_char :char ) -> char {
    match input_char {
        // black
        'n' => '♘',
        'r' => '♖',
        'b' => '♗',
        'q' => '♕',
        'k' => '♔',
        'p' => '♙',
        // white
        'N' => '♞',
        'R' => '♜',
        'B' => '♝',
        'Q' => '♛',
        'K' => '♚',
        'P' => '♟',
        _ => ' '
    }

}
