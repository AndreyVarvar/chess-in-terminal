fn main() {
    let mut running: bool = true;

    let mut board: chess::Board = chess::Board::init();

    let mut current_turn: bool = true;  // white moves

    while running {
        // print board
        // utils::clear_terminal_window();
        // chess::print_board(&board);

        // make a move
        let chess_move: String = utils::input("Enter a move: ");
        let move_info: chess::MoveInfo = chess::get_move_information(chess_move, &board, current_turn);

        println!("valid notation: {}", move_info.valid_notation);
        println!("valid move: {}", move_info.valid_move);
        println!("en passant: {}", move_info.en_passant);
        println!("check: {}", move_info.check);
        println!("mate: {}", move_info.mate);
        println!("capture: {}", move_info.capture);
        println!("castle: {}", move_info.castle);
        println!("castle type: {}", move_info.castle_type);
        println!("promotion: {}", move_info.promotion);
        println!("promotion piece: {}", TEMPTEMPTEMP(move_info.promotion_piece));
        println!("destination square: {}", move_info.destination_square);
        println!("origin square: {}", move_info.origin_square);
        println!("piece moved: {}", TEMPTEMPTEMP(move_info.piece_moved));
        println!("invalidity explanation: {}", move_info.invalidity_explanation);
    }
}

fn TEMPTEMPTEMP(piece: chess::PieceType) -> String {
    match piece {
        chess::PieceType::Pawn => "pawn".to_string(),
        chess::PieceType::Knight => "knight".to_string(),
        chess::PieceType::Bishop => "bishop".to_string(),
        chess::PieceType::Rook => "rook".to_string(),
        chess::PieceType::Queen => "queen".to_string(),
        chess::PieceType::King => "king".to_string(),
        chess::PieceType::Empty => "empty".to_string()
    }
}

mod chess {
    use crate::TEMPTEMPTEMP;

    const WHITE_PIECES: [char; 6] = ['♚', '♛', '♜', '♞', '♝', '♟'];
    const BLACK_PIECES: [char; 6] = ['♔', '♕', '♖', '♘', '♗', '♙'];

    // defining main chess types
    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum PieceType {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
        Empty
    }
    
    #[derive(Copy, Clone)]
    pub struct Piece {  // a container holding data of the piece that is ON THE BOARD
        move_count: u8,
        piece_type: PieceType,
        color: bool
    }

    pub struct Board {
        board: [[Piece; 8]; 8],

        can_perform_en_passant_if_possible: bool,
        total_move_count: u16
    } impl Board {
        pub fn init() -> Board {
            let board: Board = Board {
                board: [
                    [Piece{move_count: 0, piece_type: PieceType::Rook,  color: false}, Piece{move_count: 0, piece_type: PieceType::Knight, color: false}, Piece{move_count: 0, piece_type: PieceType::Bishop, color: false}, Piece{move_count: 0, piece_type: PieceType::Queen, color: false}, Piece{move_count: 0, piece_type: PieceType::King,  color: false}, Piece{move_count: 0, piece_type: PieceType::Bishop, color: false}, Piece{move_count: 0, piece_type: PieceType::Knight, color: false}, Piece{move_count: 0, piece_type: PieceType::Rook,  color: false}],
                    [Piece{move_count: 0, piece_type: PieceType::Pawn,  color: false}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color: false}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color: false}, Piece{move_count: 0, piece_type: PieceType::Pawn,  color: false}, Piece{move_count: 0, piece_type: PieceType::Pawn,  color: false}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color: false}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color: false}, Piece{move_count: 0, piece_type: PieceType::Pawn,  color: false}],
                    [Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}],
                    [Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}],
                    [Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}],
                    [Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty,  color: false}, Piece{move_count: 0, piece_type: PieceType::Empty, color: false}],
                    [Piece{move_count: 0, piece_type: PieceType::Pawn,  color:  true}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color:  true}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color:  true}, Piece{move_count: 0, piece_type: PieceType::Pawn,  color:  true}, Piece{move_count: 0, piece_type: PieceType::Pawn,  color:  true}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color:  true}, Piece{move_count: 0, piece_type: PieceType::Pawn,   color:  true}, Piece{move_count: 0, piece_type: PieceType::Pawn,  color:  true}],
                    [Piece{move_count: 0, piece_type: PieceType::Rook,  color:  true}, Piece{move_count: 0, piece_type: PieceType::Knight, color:  true}, Piece{move_count: 0, piece_type: PieceType::Bishop, color:  true}, Piece{move_count: 0, piece_type: PieceType::Queen, color:  true}, Piece{move_count: 0, piece_type: PieceType::King,  color:  true}, Piece{move_count: 0, piece_type: PieceType::Bishop, color:  true}, Piece{move_count: 0, piece_type: PieceType::Knight, color:  true}, Piece{move_count: 0, piece_type: PieceType::Rook,  color:  true}]
                ],

                can_perform_en_passant_if_possible: false,
                total_move_count: 0
            };

            return board;
            
        }
    }

    pub struct MoveInfo {  // the only things we need are origin square and destination square, everything else it to make sure the player wrote a correct notation, not to make the move (though it does help)
        pub valid_notation: bool,
        pub valid_move: bool,
        pub en_passant: bool,
        pub check: bool,
        pub mate: bool,
        pub capture: bool,
        pub castle: bool,
        pub castle_type: u8,  // 0 - short, 1 - long
        pub promotion: bool,
        pub promotion_piece: PieceType,
        pub destination_square: String,
        pub origin_square: String,  // if the user provided the origin square that is
        pub piece_moved: PieceType,
        pub invalidity_explanation: String
    }

    // printing board
    pub fn print_board(board: &Board){
        println!();
        println!();
        println!("     a b c d e f g h");
        println!();
        for y in 0..8 {
            let mut row_str = String::new();

            for x in 0..8 {
                if board.board[y][x].piece_type == PieceType::Empty {
                    row_str.push(if (x+y)%2 == 0 { '■' } else { '□' });
                } else {
                    row_str.push(get_piece_char(board.board[y][x]));
                }

                row_str.push(' ');
            }

            println!(" {}   {}  {}", 8-y, row_str, 8-y);
        }
        println!();
        println!("     a b c d e f g h");
        println!();
        println!();
    }

    pub fn get_piece_char(piece: Piece) -> char {
        if (piece.color == true) {
            match (piece.piece_type) {
                PieceType::King => WHITE_PIECES[0],
                PieceType::Queen => WHITE_PIECES[1],
                PieceType::Rook => WHITE_PIECES[2],
                PieceType::Knight => WHITE_PIECES[3],
                PieceType::Bishop => WHITE_PIECES[4],
                PieceType::Pawn => WHITE_PIECES[5],
                _ => ' '  // this condition is never met anyways
            }
        } else {  // the only difference is which colored pieces we are using, i just dont know how to abstract this. It is easy to read anyways, so i dont really see a need to refactor this
            match (piece.piece_type) {
                PieceType::King => BLACK_PIECES[0],
                PieceType::Queen => BLACK_PIECES[1],
                PieceType::Rook => BLACK_PIECES[2],
                PieceType::Knight => BLACK_PIECES[3],
                PieceType::Bishop => BLACK_PIECES[4],
                PieceType::Pawn => BLACK_PIECES[5],
                _ => ' '  // this condition is never met anyways
            }
        }
    }


    // validating moves
    pub fn get_move_information(chess_move: String, board: &Board, current_turn: bool) -> MoveInfo {
        let mut move_info: MoveInfo = MoveInfo {
            valid_notation: true, // we assume they are correct until we stumble across irregularities
            valid_move: true,
            en_passant: false,
            check: false,
            mate: false,
            capture: false,
            castle: false,
            castle_type: 0,
            promotion: false,
            promotion_piece: PieceType::Empty,
            destination_square: String::from("imagination square"),
            origin_square: String::from("4th dimension"),
            piece_moved: PieceType::Empty,
            invalidity_explanation: String::new()
        };

        get_move_notation_information(chess_move, &mut move_info); // get's information out of the notation, such as origin and destination squares
        validate_move_notation(&mut move_info);


        return move_info;
    }

    fn validate_move_notation(move_notation_info: &mut MoveInfo) {
        // this function balidates the correctness of the notation structure, not of the move itself
        // we find all of the combinations that should not be present in a single notation. For example promotion and en passant.

        if move_notation_info.en_passant == true { 
            if move_notation_info.capture == false { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "En passant is considered a capture. Put 'x' before destination square".to_string() }
            if move_notation_info.castle == true { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "You can't en passant while castling".to_string() }
            if move_notation_info.promotion == true { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "It's impossible to promote during en passant".to_string() }
            if move_notation_info.piece_moved != PieceType::Pawn { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "Only pawn can perform en passant".to_string() }
        }

        if move_notation_info.check == true {
            if move_notation_info.mate == true { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "You can't specify both 'check' and 'mate' in a single notation".to_string() }
            if move_notation_info.capture == false { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "En passant is considered a capture. Put 'x' before destination square".to_string() }
        }

        if move_notation_info.capture == true {
            if move_notation_info.castle == true { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "Can't capture anything while castling".to_string() }
        }

        if move_notation_info.castle == true {
            if move_notation_info.promotion == true { move_notation_info.valid_notation = false; move_notation_info.invalidity_explanation = "Can't promote while castling".to_string() }
        }
    }

    fn get_move_notation_information(mut chess_move: String, move_info: &mut MoveInfo) {
        // check if it is en passant
        if chess_move.ends_with(" e.p.") { move_info.en_passant = true; chess_move.truncate(chess_move.len() - 5);}; // since normal move notation uses ASCII characters (otherwise illegal move notation), we dont need to worry about 'str.len != number of chars'.
        
        // check if it is checkmate
        if chess_move.ends_with("#") { move_info.mate = true; chess_move.truncate(chess_move.len() - 1); }

        // check if it is a check
        if chess_move.ends_with("+") { move_info.check = true; chess_move.truncate(chess_move.len() - 1); }

        // check if it is a promotion
        if chess_move.ends_with("=Q") || chess_move.ends_with("=R") || chess_move.ends_with("=N") || chess_move.ends_with("=B") { 
            move_info.promotion = true; 
            move_info.promotion_piece = get_piece_by_char(chess_move.chars().last().unwrap()); // we only need the last char anyways
            chess_move.truncate(chess_move.len() - 2); 
        }

        // check if the move was a castle
        if chess_move.contains("O-O") {
            move_info.castle = true;
            if chess_move.contains("O-O-O") {
                move_info.castle_type = 1;
            }  // else, castle type = 0 (default)
        } else {
            // by this point the last 2 chars absolutelly MUST be the destination square.
            if chess_move.len() >= 2 {
                move_info.destination_square = chess_move.chars().last().unwrap().to_string();  // i couldnt find a better way to get the last 2 chars from the string
                chess_move.truncate(chess_move.len() - 1);
                move_info.destination_square.insert(0, chess_move.chars().last().unwrap());
                chess_move.truncate(chess_move.len() - 1);
            }

            // check, if it is a capture
            if chess_move.ends_with("x") { move_info.capture = true; chess_move.truncate(chess_move.len() - 1); }

            // now we check which piece was moved
            if chess_move.len() >= 1 {
                let piece_moved = get_piece_by_char(chess_move.chars().nth(0).unwrap());
                match piece_moved {
                    PieceType::Empty => move_info.piece_moved = PieceType::Pawn,
                    _ => { move_info.piece_moved = piece_moved; chess_move.remove(0); }
                }
            } else {
                move_info.piece_moved = PieceType::Pawn;  // every other piece leaves information about which piece was moved, except for pawns. If no information is left by thin point, it means a pawn was moved.
            }

            // now, all that is left is the optional disambiguating notation
            move_info.origin_square = chess_move;
        }
    }

    fn get_piece_by_char(piece_char: char) -> PieceType {
        match piece_char {
            'K' => PieceType::King,
            'Q' => PieceType::Queen,
            'R' => PieceType::Rook,
            'B' => PieceType::Bishop,
            'N' => PieceType::Knight,
            _ => PieceType::Empty
        }
    }
}

mod utils {
    use std::io;
    use std::io::Write;


    pub fn input(text: &str) -> String {
        // similar to python's input function
        print!("{}", text);
        io::stdout().flush().unwrap();  // some post online said this is necessary
    
        let mut ans = String::new();
    
        io::stdin().read_line(&mut ans).expect("Error getting input");
    
        return String::from(ans.trim());
    }

    pub fn clear_terminal_window() {
        print!("{esc}c", esc = 27 as char);
    }
}