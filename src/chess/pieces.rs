#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug)]
pub enum Color {
    WHITE,
    BLACK,
}
#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug)]
pub enum Kind {
    PAWN,
    KNIGHT,
    ROOK,
    BISHOP,
    QUEEN,
    KING
}
#[derive(Clone,Copy)]
pub struct ChessPiece {
    pub kind: Kind,
    pub color: Color,
}
impl ChessPiece {
    pub fn new(color: Color, kind: Kind) -> Self {
        Self{color, kind}
    }
}
