fn main() {
    println!("{}", fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq"));
}

/** convert Forsyth-Edwards Notation to a UTF-8 string with a board */
fn fen_to_board (input_str: &str) -> String {
    let mut out = String::with_capacity(64);
    for (inter_index, inter_content) in input_str.chars().enumerate() {
        if inter_index > 64 {
            // notation we will use might have captured pieces after board
            break;
        }
        out.push( piece_from_char(inter_content) );
    }
    return out;
}


/** naive map to the utf-8 chars we want to draw **/
fn piece_from_char( input_char :char ) -> char {
    match input_char {
        '/' => '\n', // FEN use this. we can optimize and assume the breaks later on
        'n' => '♘',
        'r' => '♖',
        'b' => '♗',
        'q' => '♕',
        'k' => '♔',
        'p' => '♙',
        'N' => '♞',
        'R' => '♜',
        'B' => '♝',
        'Q' => '♛',
        'K' => '♚',
        'P' => '♟',
        _ => ' '
    }

}
