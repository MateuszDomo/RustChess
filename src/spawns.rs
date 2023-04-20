use crate::{board::Board, SquareXYPositions, PieceType};




fn spawn_pieces(board: Board, squareXYPositions: SquareXYPositions) {

    let square = 0;
    for piece in board.squares{
        let is_white: bool = (0b00001000 & piece) == 0b00001000;
        let pawn = PieceType::Pawn as u8;
        let bishop = PieceType::Bishop as u8;
        let knight = PieceType::Knight as u8;
        let krook = PieceType::Rook as u8;
        let queen = PieceType::Queen as u8;
        let king = PieceType::King as u8;

        match piece{
            pawn => spawn_pawn(),
            bishop => spawn_bishop(),
            knight => spawn_knight(),
            rook => spawn_rook(),
            queen => spawn_queen(),
            king => spawn_king(),
        }
     
    }
}

fn spawn_pawn() {

}

fn spawn_bishop() {

} 

fn spawn_knight() {

}

fn spawn_rook() {

}

fn spawn_queen(){

}

fn spawn_king(){

}
