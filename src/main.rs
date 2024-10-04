fn main() {
//     let mut running = true;

//     while running {
//         print_board();
//     }
}


pub fn print_board(board: chess::Board){
    println!("imagine a board here");
}

mod chess {
    pub enum PieceType {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
        Empty
    }
    
    pub struct Piece {  // a container holding a piece data ON THE BOARD
        move_count: u8,
        piece_type: PieceType
        color: bool
    }

    pub struct Board {
        board: [[Piece; 8]; 8],

        can_perform_en_passant_if_possible: bool,
        total_move_count: u16
    } impl Board {
        fn init() {
            let mut board: Board = Board {
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
            
        }
    }
}
