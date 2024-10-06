fn main() {
    let mut running: bool = true;

    let mut board: chess::Board = chess::Board::init();

    let mut current_turn: bool = true;  // white moves

    while running {
        // print board
        // utils::clear_terminal_window();
        chess::print_board(&board);

        // make a move
        let chess_move = utils::input("Enter a move: ");
        println!("{}: {}", chess_move.clone(), chess::get_move_information(chess_move, &board, current_turn))
    }
}

mod chess {
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

    struct MoveInfo {
        valid: bool,
        en_passant: bool,  // the user info is not a guideline, it is used to compare the actual move notation and 
        check: bool,
        capture: bool,
        castle: bool,
        castle_type: u8,  // 0 - short, 1 - long
        promotion: bool,
        promotion_piece: PieceType,
        destination_square: String,
        origin_square: String,  // if the user provided the origin square that is
        piece_moved: PieceType
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

    fn get_piece_char(piece: Piece) -> char {
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
    pub fn get_move_information(chess_move: String, board: &Board, current_turn: bool) -> bool {
        return true;
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